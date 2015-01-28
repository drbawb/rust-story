use std::collections::hash_map::{HashMap, Entry};
use std::num::Float;

use graphics;
use sprite::{self, Facing, Looking, Motion, Updatable};

use collisions::{Info,Rectangle};
use map::{self, TileType};

use units;
use units::AsGame;

type MotionTup = (Motion, Facing, Looking);

// physics
static FRICTION: units::Acceleration   = units::Acceleration(0.00049804687);
static GRAVITY:  units::Acceleration   = units::Acceleration(0.00078125);

static WALKING_ACCEL:  units::Acceleration  = units::Acceleration(0.00083007812);
static MAX_VELOCITY_X: units::Velocity      = units::Velocity(0.15859375);
static MAX_VELOCITY_Y: units::Velocity      = units::Velocity(0.2998046875);

static AIR_ACCELERATION: units::Acceleration  = units::Acceleration(0.0003125);
static JUMP_GRAVITY:     units::Acceleration  = units::Acceleration(0.0003125);
static JUMP_SPEED:       units::Velocity      = units::Velocity(0.25);
static SHORT_JUMP_SPEED: units::Velocity      = units::Velocity(0.25 / 1.5);

// player sprite animation
static CHAR_OFFSET:        uint          = 12;
static SPRITE_NUM_FRAMES:  units::Frame  = 3;
static SPRITE_FPS:         units::Fps    = 20;

// motion
static STAND_FRAME: units::Tile   = units::Tile(0);
static JUMP_FRAME:  units::Tile   = units::Tile(1);
static FALL_FRAME:  units::Tile   = units::Tile(2);

// horizontal facing (Facing)
static FACING_WEST: units::Tile  = units::Tile(0 + CHAR_OFFSET);
static FACING_EAST: units::Tile  = units::Tile(1 + CHAR_OFFSET);

// vertical facing (Looking)
static WALK_UP_OFFSET:   units::Tile  = units::Tile(3);
static JUMP_DOWN_FRAME:  units::Tile  = units::Tile(6);
static STAND_DOWN_FRAME: units::Tile  = units::Tile(7);

// collision detection boxes
// (expressed as `units::Game`)
static X_BOX: Rectangle = Rectangle {
	x: units::Game(6.0), y: units::Game(10.0), 
	width: units::Game(20.0), height: units::Game(12.0)
};
static Y_BOX: Rectangle = Rectangle {
	x: units::Game(10.0), y: units::Game(2.0), 
	width: units::Game(12.0), height: units::Game(30.0)
};

static DAMAGE_INVINCIBILITY: units::Millis  = units::Millis(3000);
static INVINCIBILITY_FLASH:  units::Millis  = units::Millis(50);

static HEALTH_BAR_X: units::Tile           = units::Tile(2);
static HEALTH_BAR_Y: units::Tile           = units::Tile(2);
static HEALTH_BAR_OFS_X: units::HalfTile   = units::HalfTile(0);
static HEALTH_BAR_OFS_Y: units::HalfTile   = units::HalfTile(5);
static HEALTH_BAR_W: units::HalfTile       = units::HalfTile(8);
static HEALTH_BAR_H: units::HalfTile       = units::HalfTile(1);

static HEALTH_FILL_X: units::HalfTile          = units::HalfTile(7);
static HEALTH_FILL_Y: units::HalfTile          = units::HalfTile(4);
static HEALTH_FILL_OFS_X: units::HalfTile  = units::HalfTile(0);
static HEALTH_FILL_OFS_Y: units::HalfTile  = units::HalfTile(3);

static FILL_SHIFT: units::Game         = units::Game(2.0);	
static HEALTH_FILL_W: units::HalfTile  = units::HalfTile(5);
static HEALTH_FILL_H: units::HalfTile  = units::HalfTile(1);

/// Encapsulates the pysical motion of a player as it relates to
/// a sprite which can be animated, positioned, and drawn on the screen.
pub struct Player {
	// assets
	sprites:   HashMap<MotionTup, Box<sprite::Updatable<units::Game>>>,
	three:     Box<sprite::Updatable<units::Tile>>,
	hud:       Box<sprite::Updatable<units::Tile>>,
	hud_fill:  Box<sprite::Updatable<units::HalfTile>>,

	// positioning
	x: units::Game, 
	y: units::Game,
	movement:  MotionTup,
	on_ground: bool,

	// physics
	elapsed_time:  units::Millis,
	velocity_x:    units::Velocity,
	velocity_y:    units::Velocity,
	accel_x:       int,

	// state
	is_interacting:  bool,
	is_invincible:   bool,
	is_jump_active:  bool,

	// timers
	invincible_time: units::Millis,
}


impl Player {
	/// Loads and initializes a set of sprite-sheets for the various combinatoins of directions.
	/// (These incl: facing west and east for: standing, walking, jumping, falling.)
	///
	/// The player will spawn at `x` and `y`, though it will immediately be subject to gravity.
	/// The player is initailized `standing` facing `east`.
	/// The player will continue to fall until some collision is detected.
	pub fn new(graphics: &mut graphics::Graphics, x: units::Game, y: units::Game) -> Player {
		// insert sprites into map
		let sprite_map = 
			HashMap::<MotionTup, Box<sprite::Updatable<_>>>::new();

		let health_bar_sprite = box sprite::Sprite::new(
			graphics, 
			(HEALTH_BAR_OFS_X, HEALTH_BAR_OFS_Y),
			(HEALTH_BAR_W, HEALTH_BAR_H),
			format!("assets/base/TextBox.bmp"),
		) as Box<sprite::Updatable<_>>;

		let health_fill_sprite = box sprite::Sprite::new(
			graphics,
			(HEALTH_FILL_OFS_X, HEALTH_FILL_OFS_Y),
			(HEALTH_FILL_W.to_game() - FILL_SHIFT, HEALTH_FILL_H.to_game()),
			format!("assets/base/TextBox.bmp"),
		) as Box<sprite::Updatable<_>>;

		let digit_3 = box sprite::Sprite::new(
			graphics,
			(units::HalfTile(3), units::HalfTile(7)),
			(units::HalfTile(1), units::HalfTile(1)),
			format!("assets/base/TextBox.bmp"),
		);

		// construct new player
		let mut new_player = Player{
			elapsed_time: units::Millis(0),
			sprites:   sprite_map,
			hud:       health_bar_sprite,
			hud_fill:  health_fill_sprite,
			three:     digit_3,

			x: x,
			y: y,
			movement: (Motion::Standing, Facing::East, Looking::Horizontal),
			on_ground: false,
			
			velocity_x: units::Velocity(0.0),
			velocity_y: units::Velocity(0.0),
			accel_x: 1,

			is_interacting: false,
			is_jump_active: false,
			is_invincible:  false,

			invincible_time: units::Millis(0),
		};

		// load sprites for every possible movement tuple.
		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				for looking in sprite::LOOKINGS.iter() {
					new_player.load_sprite(graphics, (*motion, *facing, *looking));
				}
			}
		}

		new_player
	}

	/// Draws player to screen
	pub fn draw(&self, display: &graphics::Graphics) {
		if self.is_invincible && self.is_strobed() {
			return;
		} else {
			self.sprites[self.movement].draw(display, (self.x, self.y));
		}
	}

	/// Draws player's HUD if available
	pub fn draw_hud(&self, display: &graphics::Graphics) {
		if self.is_invincible && self.is_strobed() {
			return;
		} else {
			self.hud.draw(display,
			              (HEALTH_BAR_X,
			               HEALTH_BAR_Y));
			
			self.hud_fill.draw(display,
			                   (HEALTH_FILL_X,
			                    HEALTH_FILL_Y));
			
			self.three.draw(display, 
			                (units::Tile(3),
			                 units::Tile(2)));
		}
	}

	/// Updates player-state that relies on time data. (Namely physics calculations.)
	/// Determines which sprite-sheet should be used for thsi frame.
	/// Forwards the elapsed time to the current sprite.
	pub fn update(&mut self, elapsed_time: units::Millis, map: &map::Map) {
		// calculate current position
		self.elapsed_time = elapsed_time;
		
		// update sprite
		self.current_motion(); // update motion once at beginning of frame for consistency
		self.sprites[self.movement].update(elapsed_time);

		if self.is_invincible {
			self.invincible_time =
				self.invincible_time + elapsed_time;
			self.is_invincible = self.invincible_time < DAMAGE_INVINCIBILITY;
		}

		// run physics sim
		self.update_x(map);
		self.update_y(map);
	}

	fn update_x(&mut self, map: &map::Map) {
		// compute next velocity
		let accel_x: units::Acceleration = if self.accel_x < 0  {
			if self.on_ground() { -WALKING_ACCEL } else { -AIR_ACCELERATION }
		} else if self.accel_x > 0 {
			if self.on_ground() {  WALKING_ACCEL } else {  AIR_ACCELERATION }
		} else { units::Acceleration(0.0) };

		self.velocity_x = self.velocity_x + (accel_x * self.elapsed_time);

		// apply maximum bounds to velocity based on situation	
		if self.accel_x < 0 {
			self.velocity_x = units::Velocity((*self.velocity_x).max(-*MAX_VELOCITY_X));
		} else if self.accel_x > 0 {
			self.velocity_x = units::Velocity((*self.velocity_x).min( *MAX_VELOCITY_X));
		} else if self.on_ground() {
			let v_friction = FRICTION * self.elapsed_time;

			self.velocity_x = if self.velocity_x > units::Velocity(0.0) {
				units::Velocity(
					(*units::Velocity(0.0)).max(*(self.velocity_x - v_friction))
				)
			} else {
				units::Velocity(
					(*units::Velocity(0.0)).min(*(self.velocity_x + v_friction))
				)
			};
		}

		// x-axis collision checking 
		let delta = self.velocity_x * self.elapsed_time;
		if delta > units::Game(0.0) { // moving right
			// collisions right-side
			let mut info = self.get_collision_info(&self.right_collision(delta), map);
			self.x = if info.collided {
				self.velocity_x = units::Velocity(0.0);
				(info.col.to_game() - X_BOX.right())
			} else {
				(self.x + delta)
			};

			// collisions left-side
			info = self.get_collision_info(&self.left_collision(units::Game(0.0)), map);
			self.x = if info.collided {
				(info.col.to_game() + X_BOX.right())
			} else {
				self.x
			};

		} else { // moving left
			// collisions left-side
			let mut info = self.get_collision_info(&self.left_collision(delta), map);
			self.x = if info.collided {
				self.velocity_x = units::Velocity(0.0);
				(info.col.to_game() + X_BOX.right())
			} else {
				(self.x + delta) 
			};

			// collisions right-side
			info = self.get_collision_info(&self.right_collision(units::Game(0.0)), map);
			self.x = if info.collided {
				(info.col.to_game() - X_BOX.right()) 
			} else {
				self.x
			};
		}
	}

	fn update_y (&mut self, map: &map::Map) {
		// update velocity
		let gravity: units::Acceleration = 
			if self.is_jump_active 
			&& self.velocity_y < units::Velocity(0.0) {
				JUMP_GRAVITY
			} else {
				GRAVITY
			};

		let v_gravity = self.velocity_y + (gravity * self.elapsed_time);
		self.velocity_y = units::Velocity((*v_gravity).min(*MAX_VELOCITY_Y));

		// calculate delta
		let delta = self.velocity_y * self.elapsed_time;

		// check collision in direction of delta
		if delta > units::Game(0.0) {
			// react to collision
			let mut info = self.get_collision_info(&self.bottom_collision(delta), map);
			self.y = if info.collided {
				self.velocity_y = units::Velocity(0.0);
				self.on_ground = true;

				(info.row.to_game() - Y_BOX.bottom())
			} else {
				self.on_ground = false;
				(self.y + delta)
			};

			info = self.get_collision_info(&self.top_collision(units::Game(0.0)), map);
			self.y = if info.collided {
				(info.row.to_game() + Y_BOX.height())
			} else {
				self.y
			};

		} else {
			// react to collision
			let mut info = self.get_collision_info(&self.top_collision(delta), map);
			self.y = if info.collided {
				self.velocity_y = units::Velocity(0.0);
				(info.row.to_game() + Y_BOX.height())
			} else {
				self.on_ground = false;
				(self.y + delta)
			};

			info = self.get_collision_info(&self.bottom_collision(units::Game(0.0)), map);
			self.y = if info.collided {
				self.on_ground = true;
				(info.row.to_game() - Y_BOX.bottom())
			} else {
				self.y
			};
		}
	}

	fn get_collision_info(&self, hitbox: &Rectangle, tile_map: &map::Map) -> Info {
		let tiles = 
			tile_map.get_colliding_tiles(hitbox);

		let mut info = Info { collided: false, row: units::Tile(0), col: units::Tile(0) };
		for tile in tiles.iter() {
			if tile.tile_type == TileType::Wall {
				info = Info {collided: true, row: tile.row, col: tile.col};
				break;
			}
		}

		info
	}

	/// This updates the `self.movement` tuple
	/// The `Motion` is kept as-is, but the `Facing` portion of the tuple
	/// is replaced with `direction`.
	pub fn set_facing(&mut self, direction: sprite::Facing) {
		let (last_action, _, last_looking) = self.movement;
		self.movement = (last_action, direction, last_looking);
	}

	/// This updates the `self.movement` tuple
	/// The `Motion` is kept as-is, but the `Facing` portion of the tuple
	/// is replaced with `direction`.
	pub fn set_looking(&mut self, direction: sprite::Looking) {
		let (last_action, last_facing, _) = self.movement;
		self.movement = (last_action, last_facing, direction);
	}

	/// Loads a sprite for the selected `movement`, stores it in the player's sprite map.
	/// This exhaustively matches all tuples of (Motion,Facing,Looking), though certain
	/// sprites are considered invalid states [for e.g: walking + looking down]
	fn load_sprite(
		&mut self, 
		graphics: &mut graphics::Graphics, 
		movement: (sprite::Motion, sprite::Facing, sprite::Looking)
	) {
		match self.sprites.entry(movement) {
			Entry::Vacant(entry) => {
				let file_path = format!("assets/base/MyChar.bmp");
				let (motion, facing, _) = movement;
				let motion_frame = match motion {
					Motion::Standing | Motion::Walking => STAND_FRAME,
					Motion::Interacting => STAND_DOWN_FRAME,
					Motion::Jumping => JUMP_FRAME,
					Motion::Falling => FALL_FRAME
				};

				let facing_frame = match facing {
					Facing::West => FACING_WEST,
					Facing::East => FACING_EAST
				};

				let loaded_sprite = match movement {
					// static: standing in place
					  (Motion::Standing, _, looking)
					| (Motion::Interacting, _, looking) => {
						let looking_frame = match looking {
							Looking::Up => WALK_UP_OFFSET,
							_ => units::Tile(0)
						};
					
						box sprite::Sprite::new(
							graphics, 
							(motion_frame + (looking_frame), facing_frame), 
							(units::Tile(1), units::Tile(1)),	
							file_path
						) as Box<sprite::Updatable<_>>
					}

					// static: jumping or falling
					// (overrides 'STAND_DOWN_FRAME')
					(Motion::Jumping, _, looking)
					| (Motion::Falling, _, looking) => {
						let looking_frame = match looking { // ignored while jumping / falling for now
							Looking::Down => JUMP_DOWN_FRAME,
							Looking::Up => WALK_UP_OFFSET,
							_ => motion_frame
						};
						
						box sprite::Sprite::new(
							graphics,
							(looking_frame, facing_frame),
							(units::Tile(1), units::Tile(1)),
							file_path
						) as Box<sprite::Updatable<_>>
					}

					// dynamic: 
					(Motion::Walking, _, looking) => {
						let looking_frame = match looking {
							Looking::Up => WALK_UP_OFFSET,
							_ => units::Tile(0)
						};
		
						box sprite::AnimatedSprite::new(
							graphics, file_path,
							(motion_frame + looking_frame, facing_frame),
							(units::Tile(1), units::Tile(1)),
							SPRITE_NUM_FRAMES, SPRITE_FPS
						).unwrap() as Box<sprite::Updatable<_>>
					}
				};

				entry.insert(loaded_sprite);
			},

			_ => {},
		};
	}

	/// The player will immediately face `West`
	/// They will then accelerate at a constant rate in that direction.
	pub fn start_moving_left(&mut self) {
		self.is_interacting = false;
		self.set_facing(Facing::West);
		self.accel_x = -1;
	}

	/// The player will immediately face `East`
	/// They will then accelerate at a constant rate in that direction.
	pub fn start_moving_right(&mut self) {
		self.is_interacting = false;
		self.set_facing(Facing::East);
		self.accel_x = 1;
	}

	/// The player will immediately cease acceleration.
	/// They will still be facing the same direction as before this call.
	pub fn stop_moving(&mut self) {
		self.accel_x = 0;
	}

	pub fn look_up(&mut self) {
		self.is_interacting = false;
		self.set_looking(Looking::Up);
	}

	pub fn look_down(&mut self) {
		let(motion,_,looking) = self.movement;
		if looking == Looking::Down {return;}
		if motion == Motion::Walking {return;}
		
		self.is_interacting = self.on_ground();
		self.set_looking(Looking::Down);
	}

	pub fn look_horizontal(&mut self) {
		self.set_looking(Looking::Horizontal);
	}

	/// Resets the player's jump timer if they are currently on the ground.
	/// Otherwise: uses the remainder of the player's jump timer to extend
	/// their jump.
	///
	/// The effects of a jump against gravity are `instantaneous` and do not
	/// consider acceleration.
	pub fn start_jump(&mut self) {
		self.is_jump_active = true;
		self.is_interacting = false;

		if self.on_ground() {
			self.velocity_y = -JUMP_SPEED;
		}
	}

	/// A player will immediately cease their jump and become subject
	/// to the effects of gravity.
	///
	/// While the player is in this state: their remaining `jump time` is
	/// temporarily suspended.
	pub fn stop_jump(&mut self) {
		self.is_jump_active = false;
	}

	/// This is called to update the player's `movement` based on
	/// their current: acceleration, velocity, and collision state.
	///
	/// Ideally this should be called early-on, once per frame,
	/// so that the rest of the frames calculations `appear consistent`
	///
	/// This is because all updates determine which sprite-sheet to mutate
	/// based on `self.movement` -- so if self.movement is updated multiple
	/// times per frame then some sprite-sheet updates may get `lost.`
	pub fn current_motion(&mut self) {
		let (_, last_facing, last_looking) = self.movement;

		self.movement = if self.on_ground() {
			if self.is_interacting {
				(Motion::Interacting, last_facing, last_looking)
			} else if self.accel_x == 0 {
				(Motion::Standing, last_facing, last_looking)
			} else {
				(Motion::Walking, last_facing, last_looking)
			}	
		} else {
			if self.velocity_y < units::Velocity(0.0) {
				(Motion::Jumping, last_facing, last_looking)
			} else {
				(Motion::Falling, last_facing, last_looking)
			}
		};
	}
	
	/// A player's damage rectangle encompasses the whole player.
	pub fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.x + X_BOX.left(),
			y: self.y + Y_BOX.top(),
			width: X_BOX.width(),
			height: Y_BOX.height(),
		}
	}

	/// The player takes damage from the world
	pub fn take_damage(&mut self) {
		if self.is_invincible { return; }

		self.velocity_y = units::Velocity((*self.velocity_y).min(-*SHORT_JUMP_SPEED));

		self.is_invincible    = true;
		self.invincible_time  = units::Millis(0);

		println!("bat has collided with me! D:");
	}

	/// Returns true if the player is currently invisible due to an
	/// invincibility strobing effect.
	#[inline]
	fn is_strobed(&self) -> bool {
		let (units::Millis(ref invincible_time), units::Millis(ref flash_time)) =
			(self.invincible_time, INVINCIBILITY_FLASH);

		// how long player has been invincible over some strobe interval
		// if remainder is even: player should not be displayed
		(*invincible_time / *flash_time) % 2 == 0
	}

	pub fn center_x(&self) -> units::Game {
		self.x + units::HalfTile(1)
	}

	// x-axis collision detection
	fn left_collision(&self, delta: units::Game) -> Rectangle {
		assert!(delta <= units::Game(0.0));

		Rectangle {
			x: self.x + (X_BOX.left() + delta),
			y: self.y + X_BOX.top(),
			width: (X_BOX.width() / units::Game(2.0)) - delta,
			height: X_BOX.height()
		}
	}

	
	fn right_collision(&self, delta: units::Game) -> Rectangle {
		assert!(delta >= units::Game(0.0));
		
		Rectangle {
			x: self.x + X_BOX.left() + (X_BOX.width() / units::Game(2.0)),
			y: self.y + X_BOX.top(),
			width: 	(X_BOX.width() / units::Game(2.0)) + delta,
			height: X_BOX.height()
		}
	}

	// y-axis collision detection
	fn top_collision(&self, delta: units::Game) -> Rectangle {
		assert!(delta <= units::Game(0.0));

		Rectangle {
			x: self.x + Y_BOX.left(),
			y: self.y + (Y_BOX.top() + delta),
			width: Y_BOX.width(),
			height: (Y_BOX.height() / units::Game(2.0)) - delta
		}
	}

	fn bottom_collision(&self, delta: units::Game) -> Rectangle {
		assert!(delta >= units::Game(0.0));
		
		Rectangle {
			x: self.x + Y_BOX.left(),
			y: self.y + Y_BOX.top() + (Y_BOX.height() / units::Game(2.0)),
			width:  Y_BOX.width(),
			height: (Y_BOX.height() / units::Game(2.0)) + delta
		}
	}

	/// The player will collide w/ the ground at y-coord `320`
	/// Gravity cannot pull them below this floor.
	fn on_ground(&self) -> bool {
		self.on_ground
	}
}
