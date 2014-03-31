use collections::hashmap::HashMap;

use game::graphics;
use game::sprite;


use game::collisions::{Info,Rectangle};
use game::map;

use game::units;
use game::units::{AsGame,HALF_TILE};

type MotionTup = (sprite::Motion, sprite::Facing, sprite::Looking);

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

static HEALTH_BAR_X: units::Game = units::Tile(2).to_game();

/// Encapsulates the pysical motion of a player as it relates to
/// a sprite which can be animated, positioned, and drawn on the screen.
pub struct Player {
	priv sprites:   HashMap<MotionTup, ~sprite::Updatable>,
	priv hud:       ~sprite::Updatable,
	priv hud_fill:  ~sprite::Updatable,
	priv three:     ~sprite::Updatable,

	// positioning
	priv x: units::Game,
	priv y: units::Game,
	priv movement:  MotionTup,
	priv on_ground: bool,

	// physics
	priv elapsed_time:  units::Millis,
	priv velocity_x:    units::Velocity,
	priv velocity_y:    units::Velocity,
	priv accel_x:       int,

	// state
	priv is_interacting:  bool,
	priv is_invincible:   bool,
	priv is_jump_active:  bool,

	// timers
	priv invincible_time: units::Millis,
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
			HashMap::<MotionTup, ~sprite::Updatable>::new();

		// "statics" 
		let HEALTH_BAR_Y      = units::Tile(2).to_game();
		let HEALTH_BAR_OFS_X  = units::Game(0.0);
		let HEALTH_BAR_OFS_Y  = units::Game(5.0) * HALF_TILE;
		let HEALTH_BAR_WIDTH  = units::Tile(4).to_game();
		let HEALTH_BAR_HEIGHT = units::HALF_TILE;

		let HEALTH_FILL_X      = units::Game(5.0) * units::HALF_TILE;
		let HEALTH_FILL_Y      = units::Tile(2).to_game();
		let HEALTH_FILL_OFS_X  = units::Game(0.0);
		let HEALTH_FILL_OFS_Y  = units::Game(3.0) * HALF_TILE;
		let HEALTH_FILL_WIDTH  = (units::Game(5.0) * HALF_TILE) - units::Game(2.0);
		let HEALTH_FILL_HEIGHT = units::HALF_TILE;

		let health_bar_sprite = ~sprite::Sprite::new(
			graphics, 
			(HEALTH_BAR_X, HEALTH_BAR_Y),
			(HEALTH_BAR_OFS_X, HEALTH_BAR_OFS_Y),
			(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT),
			~"assets/base/TextBox.bmp",
		) as ~sprite::Updatable;

		let health_fill_sprite = ~sprite::Sprite::new(
			graphics,
			(HEALTH_FILL_X, HEALTH_FILL_Y),
			(HEALTH_FILL_OFS_X, HEALTH_FILL_OFS_Y),
			(HEALTH_FILL_WIDTH, HEALTH_FILL_HEIGHT),
			~"assets/base/TextBox.bmp",
		) as ~sprite::Updatable;

		let digit_3 = ~sprite::Sprite::new(
			graphics,
			(units::Tile(2).to_game(), units::Tile(2).to_game()),
			((units::Game(3.0) * HALF_TILE), (units::Game(7.0) * HALF_TILE)),
			(HALF_TILE, HALF_TILE),
			~"assets/base/TextBox.bmp",
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
			movement: (sprite::Standing, sprite::East, sprite::Horizontal),
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
			self.sprites.get(&self.movement).draw(display);
		}
	}

	/// Draws player's HUD if available
	pub fn draw_hud(&self, display: &graphics::Graphics) {
		if self.is_invincible && self.is_strobed() {
			return;
		} else {
			self.hud.draw(display);
			self.hud_fill.draw(display);

			self.three.draw(display);
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
		self.set_position((self.x, self.y));
		self.sprites.get_mut(&self.movement).update(elapsed_time);

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

		if self.accel_x < 0 {
			self.velocity_x =  units::max(self.velocity_x, -MAX_VELOCITY_X);
		} else if self.accel_x > 0 {
			self.velocity_x = units::min(self.velocity_x, MAX_VELOCITY_X);
		} else if self.on_ground() {
			self.velocity_x = if self.velocity_x > units::Velocity(0.0) {
				units::max(units::Velocity(0.0), self.velocity_x - (FRICTION * self.elapsed_time))
			} else {
				units::min(units::Velocity(0.0), self.velocity_x + (FRICTION * self.elapsed_time))
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

		self.velocity_y = units::min(
			self.velocity_y + (gravity * self.elapsed_time), 
			MAX_VELOCITY_Y
		);

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
			if tile.tile_type == map::Wall {
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

	/// Instructs the current sprite-sheet to position itself
	/// at the coordinates specified by `coords:(x,y)`.
	fn set_position(&mut self, coords: (units::Game, units::Game)) {
		self.sprites.get_mut(&self.movement).set_position(coords);
	}

	/// Loads a sprite for the selected `movement`, stores it in the player's sprite map.
	/// This exhaustively matches all tuples of (Motion,Facing,Looking), though certain
	/// sprites are considered invalid states [for e.g: walking + looking down]
	fn load_sprite(
		&mut self, 
		graphics: &mut graphics::Graphics, 
		movement: (sprite::Motion, sprite::Facing, sprite::Looking)
	) {
		self.sprites.find_or_insert_with(movement, |key| -> ~sprite::Updatable {
			let file_path = ~"assets/base/MyChar.bmp";
			let (motion, facing, _) = *key;
			let motion_frame = match motion {
				sprite::Standing | sprite::Walking => STAND_FRAME,
				sprite::Interacting => STAND_DOWN_FRAME,
				sprite::Jumping => JUMP_FRAME,
				sprite::Falling => FALL_FRAME
			};

			let facing_frame = match facing {
				sprite::West => FACING_WEST,
				sprite::East => FACING_EAST
			};

			match movement {
				// static: standing in place
				  (sprite::Standing, _, looking)
				| (sprite::Interacting, _, looking) => {
					let looking_frame = match looking {
						sprite::Up => WALK_UP_OFFSET,
						_ => units::Tile(0)
					};
				
					~sprite::Sprite::new(
						graphics, 
						(units::Game(0.0), units::Game(0.0)), 
						(motion_frame + (looking_frame), facing_frame), 
						(units::Tile(1), units::Tile(1)),	
						file_path
					) as ~sprite::Updatable 
				}

				// static: jumping or falling
				// (overrides 'STAND_DOWN_FRAME')
				(sprite::Jumping, _, looking)
				| (sprite::Falling, _, looking) => {
					let looking_frame = match looking { // ignored while jumping / falling for now
						sprite::Down => JUMP_DOWN_FRAME,
						sprite::Up => WALK_UP_OFFSET,
						_ => motion_frame
					};
					
					~sprite::Sprite::new(
						graphics,
						(units::Game(0.0), units::Game(0.0)),
						(looking_frame, facing_frame),
						(units::Tile(1), units::Tile(1)),
						file_path
					) as ~sprite::Updatable
				}

				// dynamic: 
				(sprite::Walking, _, looking) => {
					let looking_frame = match looking {
						sprite::Up => WALK_UP_OFFSET,
						_ => units::Tile(0)
					};
	
					~sprite::AnimatedSprite::new(
						graphics, file_path,
						(motion_frame + looking_frame, facing_frame),
						(units::Tile(1), units::Tile(1)),
						SPRITE_NUM_FRAMES, SPRITE_FPS
					).unwrap() as ~sprite::Updatable
				}
			}
		});
	}

	/// The player will immediately face `West`
	/// They will then accelerate at a constant rate in that direction.
	pub fn start_moving_left(&mut self) {
		self.is_interacting = false;
		self.set_facing(sprite::West);
		self.accel_x = -1;
	}

	/// The player will immediately face `East`
	/// They will then accelerate at a constant rate in that direction.
	pub fn start_moving_right(&mut self) {
		self.is_interacting = false;
		self.set_facing(sprite::East);
		self.accel_x = 1;
	}

	/// The player will immediately cease acceleration.
	/// They will still be facing the same direction as before this call.
	pub fn stop_moving(&mut self) {
		self.accel_x = 0;
	}

	pub fn look_up(&mut self) {
		self.is_interacting = false;
		self.set_looking(sprite::Up);
	}

	pub fn look_down(&mut self) {
		let(motion,_,looking) = self.movement;
		if looking == sprite::Down {return;}
		if motion == sprite::Walking {return;}
		
		self.is_interacting = self.on_ground();
		self.set_looking(sprite::Down);
	}

	pub fn look_horizontal(&mut self) {
		self.set_looking(sprite::Horizontal);
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
				(sprite::Interacting, last_facing, last_looking)
			} else if self.accel_x == 0 {
				(sprite::Standing, last_facing, last_looking)
			} else {
				(sprite::Walking, last_facing, last_looking)
			}	
		} else {
			if self.velocity_y < units::Velocity(0.0) {
				(sprite::Jumping, last_facing, last_looking)
			} else {
				(sprite::Falling, last_facing, last_looking)
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

		self.velocity_y = units::min(self.velocity_y, -SHORT_JUMP_SPEED);

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
		self.x + HALF_TILE
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
