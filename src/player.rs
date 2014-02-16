use std::f64;
use std::cmp;
use std::hashmap::HashMap;

use game::graphics;
use game::sprite;

use game::map;
use game::collisions::{Info,Rectangle};

// physics
static FRICTION: f64 				= 0.00049804687;	// (pixels / ms) / ms
static GRAVITY: f64					= 0.00078125;		// (pixels / ms) / ms

static WALKING_ACCEL: f64 			= 0.00083007812;	// (pixels / ms) / ms
static MAX_VELOCITY_X: f64 			= 0.15859375;		// (pixels / ms)
static MAX_VELOCITY_Y: f64			= 0.2998046875;		// (pixels / ms)

static	AIR_ACCELERATION: f64		= 0.0003125;		// (pixels / ms) / ms
static 	JUMP_GRAVITY: f64			= 0.0003125;		// (pixels / ms) / ms
static 	JUMP_SPEED: f64				= 0.25;				// (pixels / ms)


// player sprite animation
static CHAR_OFFSET: i32				= 12;
static SPRITE_NUM_FRAMES: i32		= 3;
static SPRITE_FPS: i32				= 20;

// motion
static STAND_FRAME: i32 			= 0;
static JUMP_FRAME: i32 				= 1;
static FALL_FRAME: i32 				= 2;

// horizontal facing (Facing)
static FACING_WEST: i32 			= 0 + CHAR_OFFSET;
static FACING_EAST: i32 			= 1 + CHAR_OFFSET;

// vertical facing (Looking)
static WALK_UP_OFFSET: i32			= 3;
static JUMP_DOWN_FRAME:  i32		= 6;
static STAND_DOWN_FRAME: i32 		= 7;

// collision detection boxes
static X_BOX: Rectangle = 	Rectangle {x: 6, y: 10, width: 20, height: 12 };
static Y_BOX: Rectangle = 	Rectangle {x: 10, y: 2, width: 12, height: 30 };


/// Encapsulates the pysical motion of a player as it relates to
/// a sprite which can be animated, positioned, and drawn on the screen.
pub struct Player {
	priv sprites: HashMap<(sprite::Motion,sprite::Facing,sprite::Looking), ~sprite::Updatable:>,
	
	// positioning
	priv x: i32,
	priv y: i32,
	priv movement: (sprite::Motion, sprite::Facing, sprite::Looking),
	priv on_ground: bool,

	// physics
	priv elapsed_time: sprite::Millis,
	priv velocity_x: f64,
	priv velocity_y: f64,
	priv accel_x: int,

	priv is_jump_active: bool
}


impl Player {
	/// Loads and initializes a set of sprite-sheets for the various combinatoins of directions.
	/// (These incl: facing west and east for: standing, walking, jumping, falling.)
	///
	/// The player will spawn at `x` and `y`, though it will immediately be subject to gravity.
	/// The player is initailized `standing` facing `east`.
	/// The player will continue to fall until some collision is detected.
	pub fn new(graphics: &mut graphics::Graphics, x: i32, y: i32) -> Player {
		// insert sprites into map
		let sprite_map = 
			HashMap::<(sprite::Motion,sprite::Facing,sprite::Looking), ~sprite::Updatable:>::new();

		// construct new player
		let mut new_player = Player{
			elapsed_time: sprite::Millis(0),
			sprites: sprite_map,

			x: x, 
			y: y,
			movement: (sprite::Standing, sprite::East, sprite::Horizontal),
			on_ground: false,
			
			velocity_x: 0.0,
			velocity_y: 0.0,
			accel_x: 1,

			is_jump_active: false
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
		self.sprites.get(&self.movement).draw(display);
	}

	/// Updates player-state that relies on time data. (Namely physics calculations.)
	/// Determines which sprite-sheet should be used for thsi frame.
	/// Forwards the elapsed time to the current sprite.
	pub fn update(&mut self, elapsed_time: sprite::Millis, map: &map::Map) {
		// calculate current position
		self.elapsed_time = elapsed_time;
		
		// update sprite
		self.current_motion(); // update motion once at beginning of frame for consistency
		self.set_position((self.x, self.y));
		self.sprites.get_mut(&self.movement).update(elapsed_time);

		// run physics sim
		self.update_x(map);
		self.update_y(map);
	}

	fn update_x(&mut self, map: &map::Map) {
		// compute next velocity
		let sprite::Millis(elapsed_time_ms) = self.elapsed_time;	
		let accel_x = if self.accel_x < 0  {
			if self.on_ground() {
				-WALKING_ACCEL
			} else {
				-AIR_ACCELERATION
			}
		} else if self.accel_x > 0 {
			if self.on_ground() {
				WALKING_ACCEL
			} else {
				AIR_ACCELERATION
			}
		} else {
			0.0
		};

		self.velocity_x += 
			accel_x * elapsed_time_ms as f64;

		if self.accel_x < 0 {
			self.velocity_x = cmp::max(self.velocity_x, -MAX_VELOCITY_X);
		} else if self.accel_x > 0 {
			self.velocity_x = cmp::min(self.velocity_x, MAX_VELOCITY_X);
		} else if self.on_ground() {
			self.velocity_x = if self.velocity_x > 0.0 {
				cmp::max(0.0, self.velocity_x - (FRICTION * elapsed_time_ms as f64))
			} else {
				cmp::min(0.0, self.velocity_x + (FRICTION * elapsed_time_ms as f64))
			};
		}

		let delta = f64::round(
			self.velocity_x * elapsed_time_ms as f64
		) as int;

		// x-axis collision checking 
		if delta > 0 { // moving right
			// collisions right-side
			let mut info = self.get_collision_info(&self.right_collision(delta), map);
			self.x = if info.collided {
				self.velocity_x = 0.0;
				((info.col * sprite::TILE_SIZE as int) - X_BOX.right()) as i32
			} else {
				(self.x as int + delta) as i32
			};

			// collisions left-side
			info = self.get_collision_info(&self.left_collision(0), map);
			self.x = if info.collided {
				((info.col * sprite::TILE_SIZE as int) + X_BOX.right()) as i32
			} else {
				self.x
			};

		} else { // moving left
			// collisions left-side
			let mut info = self.get_collision_info(&self.left_collision(delta), map);
			self.x = if info.collided {
				self.velocity_x = 0.0;
				((info.col * sprite::TILE_SIZE as int) + X_BOX.right()) as i32
			} else {
				(self.x as int + delta) as i32
			};

			// collisions right-side
			info = self.get_collision_info(&self.right_collision(0), map);
			self.x = if info.collided {
				((info.col * sprite::TILE_SIZE as int) - X_BOX.right()) as i32
			} else {
				self.x
			};
		}
	}

	fn update_y (&mut self, map: &map::Map) {
		// determine effects of gravity
		let sprite::Millis(elapsed_time_ms) = self.elapsed_time;
		
		// update velocity
		let gravity: f64 = if self.is_jump_active && self.velocity_y < 0.0 {
			JUMP_GRAVITY
		} else {
			GRAVITY
		};

		self.velocity_y = cmp::min(
			self.velocity_y + gravity * elapsed_time_ms as f64, 
			MAX_VELOCITY_Y
		);

		// calculate delta
		let delta: int = f64::round(
			self.velocity_y * elapsed_time_ms as f64
		) as int;

		// check collision in direction of delta
		if delta > 0 {
			// react to collision
			let mut info = self.get_collision_info(&self.bottom_collision(delta), map);
			self.y = if info.collided {
				self.velocity_y = 0.0;
				self.on_ground = true;

				((info.row * sprite::TILE_SIZE as int) - Y_BOX.bottom()) as i32
			} else {
				self.on_ground = false;
				(self.y as int + delta) as i32
			};

			info = self.get_collision_info(&self.top_collision(0), map);
			self.y = if info.collided {
				((info.row * sprite::TILE_SIZE as int) + Y_BOX.height()) as i32
			} else {
				self.y
			};

		} else {
			// react to collision
			let mut info = self.get_collision_info(&self.top_collision(delta), map);
			self.y = if info.collided {
				self.velocity_y = 0.0;

				((info.row * sprite::TILE_SIZE as int) + Y_BOX.height()) as i32
			} else {
				self.on_ground = false;
				(self.y as int + delta) as i32
			};

			info = self.get_collision_info(&self.bottom_collision(0), map);
			self.y = if info.collided {
				self.on_ground = true;

				((info.row * sprite::TILE_SIZE as int) - Y_BOX.bottom()) as i32
			} else {
				self.y
			};
		}
	}

	fn get_collision_info(&self, hitbox: &Rectangle, tile_map: &map::Map) -> Info {
		let tiles = 
			tile_map.get_colliding_tiles(hitbox);

		let mut info = Info { collided: false, row: 0, col: 0 };
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
	fn set_position(&mut self, coords: (i32,i32)) {
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
		self.sprites.find_or_insert_with(movement, |key| -> ~sprite::Updatable: {
			let file_path = ~"assets/MyChar.bmp";
			let (motion, facing, _) = *key;
			let motion_frame = match motion {
				sprite::Standing | sprite::Walking => STAND_FRAME,
				sprite::Jumping => JUMP_FRAME,
				sprite::Falling => FALL_FRAME
			};

			let facing_frame = match facing {
				sprite::West => FACING_WEST,
				sprite::East => FACING_EAST
			};

			match movement {
				// static: standing in place
				(sprite::Standing, _, looking) => {
					let looking_frame = match looking {
						sprite::Up => WALK_UP_OFFSET,
						sprite::Down => STAND_DOWN_FRAME,
						_ => 0
					};
				
					~sprite::Sprite::new(graphics, (0,0), (motion_frame + (looking_frame), facing_frame), file_path) as ~sprite::Updatable: 
				}

				// static: jumping
				(sprite::Jumping, _, looking)
				| (sprite::Falling, _, looking) => {
					let looking_frame = match looking { // ignored while jumping / falling for now
						sprite::Down => JUMP_DOWN_FRAME,
						_ => motion_frame
					};
					
					~sprite::Sprite::new(graphics, (0,0), (looking_frame, facing_frame), file_path) as ~sprite::Updatable: 
				}

				// dynamic: 
				(sprite::Walking, _, looking) => {
					let looking_frame = match looking {
						sprite::Up => WALK_UP_OFFSET,
						_ => 0
					};
	
					~sprite::AnimatedSprite::new(graphics, file_path, (motion_frame + looking_frame, facing_frame), SPRITE_NUM_FRAMES, SPRITE_FPS).unwrap() as ~sprite::Updatable:
				}
			}
		});
	}

	/// The player will immediately face `West`
	/// They will then accelerate at a constant rate in that direction.
	pub fn start_moving_left(&mut self) {
		self.set_facing(sprite::West);
		self.accel_x = -1;
	}

	/// The player will immediately face `East`
	/// They will then accelerate at a constant rate in that direction.
	pub fn start_moving_right(&mut self) {
		self.set_facing(sprite::East);
		self.accel_x = 1;
	}

	/// The player will immediately cease acceleration.
	/// They will still be facing the same direction as before this call.
	pub fn stop_moving(&mut self) {
		self.accel_x = 0;
	}

	pub fn look_up(&mut self) {
		self.set_looking(sprite::Up);
	}

	pub fn look_down(&mut self) {
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
			if self.accel_x == 0 {
				(sprite::Standing, last_facing, last_looking)
			} else {
				(sprite::Walking, last_facing, last_looking)
			}	
		} else {
			if self.velocity_y < 0.0 {
				(sprite::Jumping, last_facing, last_looking)
			} else {
				(sprite::Falling, last_facing, last_looking)
			}
		}
	}

	// x-axis collision detection
	fn left_collision(&self, delta: int) -> Rectangle {
		assert!(delta <= 0);

		Rectangle {
			x: self.x as int + (X_BOX.left() + delta),
			y: self.y as int + X_BOX.top(),
			width: (X_BOX.width() / 2) - delta,
			height: X_BOX.height()
		}
	}

	
	fn right_collision(&self, delta: int) -> Rectangle {
		assert!(delta >= 0);
		
		Rectangle {
			x: self.x as int + X_BOX.left() + (X_BOX.width() / 2),
			y: self.y as int + X_BOX.top(),
			width: 	(X_BOX.width() / 2) + delta,
			height: X_BOX.height()
		}
	}

	// y-axis collision detection
	fn top_collision(&self, delta: int) -> Rectangle {
		assert!(delta <= 0);

		Rectangle {
			x: self.x as int + Y_BOX.left(),
			y: self.y as int + (Y_BOX.top() + delta),
			width: Y_BOX.width(),
			height: (Y_BOX.height() / 2) - delta
		}
	}

	fn bottom_collision(&self, delta: int) -> Rectangle {
		assert!(delta >= 0);
		
		Rectangle {
			x: self.x as int + Y_BOX.left(),
			y: self.y as int + Y_BOX.top() + (Y_BOX.height() / 2),
			width: 	Y_BOX.width(),
			height: (Y_BOX.height() / 2) + delta
		}
	}
	

	/// The player will collide w/ the ground at y-coord `320`
	/// Gravity cannot pull them below this floor.
	fn on_ground(&self) -> bool {			
		self.on_ground
	}
}
