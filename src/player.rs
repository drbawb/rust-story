use std::f64;
use std::cmp;
use std::hashmap::HashMap;

use game::graphics;
use game::sprite;

static SLOWDOWN_VELOCITY: f64 		= 0.8;
static WALKING_ACCEL: f64 			= 0.0012;
static MAX_VELOCITY_X: f64 			= 0.325;
static MAX_VELOCITY_Y: f64			= 0.325;

static GRAVITY: f64					= 0.0012;
static JUMP_SPEED: f64				= 0.325;
static JUMP_TIME: sprite::Millis	= sprite::Millis(275);


static CHAR_OFFSET: i32				= 0;
static FACING_WEST: i32 			= 0 + CHAR_OFFSET;
static FACING_EAST: i32 			= 1 + CHAR_OFFSET;

static STAND_FRAME: i32 			= 0;
static JUMP_FRAME: i32 				= 1;
static FALL_FRAME: i32 				= 2;



/// Encapsulates the pysical motion of a player as it relates to
/// a sprite which can be animated, positioned, and drawn on the screen.
pub struct Player {
	priv sprites: HashMap<(sprite::Motion,sprite::Facing,sprite::Looking), ~sprite::Updatable:>,
	
	// positioning
	priv x: i32,
	priv y: i32,
	priv movement: (sprite::Motion, sprite::Facing, sprite::Looking),

	// physics
	priv elapsed_time: sprite::Millis,
	priv velocity_x: f64,
	priv velocity_y: f64,
	priv accel_x: f64,

	priv jump: Jump
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

		let mut new_player = Player{
			elapsed_time: sprite::Millis(0),
			sprites: sprite_map,

			x: x, 
			y: y,
			movement: (sprite::Standing, sprite::East, sprite::Horizontal),
			
			velocity_x: 0.0,
			velocity_y: 0.0,
			accel_x: 0.0,

			jump: Jump::new()
		};

		// Load sprites for every possible movement tuple.
		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				for looking in sprite::LOOKINGS.iter() {
					new_player.load_sprite(graphics, (*motion, *facing, *looking));
				}
			}
		}

		new_player
	}

	/// First attempt at `safely loading sprites`
	/// Since I can't loop over enums basic plan is this:
	/// * `match{}` will provide compile time safety that map is fully initialized.
	///	* fast path will basically be a map access.
	fn load_sprite(
		&mut self, 
		graphics: &mut graphics::Graphics, 
		movement: (sprite::Motion, sprite::Facing, sprite::Looking)
	) {
		self.sprites.find_or_insert_with(movement, |key| -> ~sprite::Updatable: {
			let (motion, facing, looking) = *key;
			let motion_frame = match motion {
				sprite::Standing | sprite::Walking => STAND_FRAME,
				sprite::Jumping => JUMP_FRAME,
				sprite::Falling => FALL_FRAME
			};

			let facing_frame = match facing {
				sprite::West => FACING_WEST,
				sprite::East => FACING_EAST
			};

			let looking_frame = match looking {
				sprite::Up => 	3,
				sprite::Down => 6,
				sprite::Horizontal => 0
			};

			match movement {
				// static: looking up or down
					(_,_,sprite::Up)
				| 	(_,_, sprite::Down) => {
					~sprite::Sprite::new(graphics, (0,0), (looking_frame, facing_frame), ~"assets/MyChar.bmp") as ~sprite::Updatable: 
				}

				// static: falling, facing east or west
				(sprite::Falling,_,sprite::Horizontal) => {
					~sprite::Sprite::new(graphics, (0,0), (motion_frame, facing_frame), ~"assets/MyChar.bmp") as ~sprite::Updatable: 
				}

				// static: standing, facing east or west
				(sprite::Standing,_,sprite::Horizontal) => {
					~sprite::Sprite::new(graphics, (0,0), (motion_frame, facing_frame), ~"assets/MyChar.bmp") as ~sprite::Updatable: 
				}

				// dynamic: walking, facing east or west
					(sprite::Walking,_,sprite::Horizontal)
				| 	( sprite::Jumping,_,sprite::Horizontal) => {
					~sprite::AnimatedSprite::new(graphics, ~"assets/MyChar.bmp", (motion_frame, facing_frame), 3, 20).unwrap() as ~sprite::Updatable:
				}
			}
		});
	}

	/// The player will immediately face `West`
	/// They will then accelerate at a constant rate in that direction.
	pub fn start_moving_left(&mut self) {
		self.set_facing(sprite::West);
		self.accel_x = -WALKING_ACCEL;
	}

	/// The player will immediately face `East`
	/// They will then accelerate at a constant rate in that direction.
	pub fn start_moving_right(&mut self) {
		self.set_facing(sprite::East);
		self.accel_x = WALKING_ACCEL;
	}

	/// The player will immediately cease acceleration.
	/// They will still be facing the same direction as before this call.
	pub fn stop_moving(&mut self) {
		self.accel_x = 0.0;
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
		if self.on_ground() {
			self.jump.reset();
			self.velocity_y = -JUMP_SPEED;
		} else if self.velocity_y < 0.0 {
			self.jump.reactivate();
		}
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
			if self.accel_x == 0.0 {
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

	/// A player will immediately cease their jump and become subject
	/// to the effects of gravity.
	///
	/// While the player is in this state: their remaining `jump time` is
	/// temporarily suspended.
	pub fn stop_jump(&mut self) {
		self.velocity_y = 0.0;
		self.jump.deactivate();
	}

	/// The player will collide w/ the ground at y-coord `320`
	/// Gravity cannot pull them below this floor.
	pub fn on_ground(&self) -> bool {			
		self.y == 320
	}
}

/* Proxies for drawable sprite traits */
/// Proxies update calls to underlying sprite
impl sprite::Updatable for Player {
	/// Updates player-state that relies on time data. (Namely physics calculations.)
	/// Determines which sprite-sheet should be used for thsi frame.
	/// Forwards the elapsed time to the current sprite.
	fn update(&mut self, elapsed_time: sprite::Millis) {
		// calculate current position
		self.elapsed_time = elapsed_time;
		self.jump.update(elapsed_time);

		// update sprite
		self.current_motion(); // update motion once at beginning of frame for consistency
		self.set_position((self.x, self.y));
		self.sprites.get_mut(&self.movement).update(elapsed_time);

		// calculate next position
		let sprite::Millis(elapsed_time_ms) = self.elapsed_time;
		self.x += f64::round(
			self.velocity_x * elapsed_time_ms as f64
		) as i32;

		// compute velocity x for next frame
		self.velocity_x += 
			self.accel_x * elapsed_time_ms as f64;

		if self.accel_x < 0.0 {
			self.velocity_x = cmp::max(self.velocity_x, -MAX_VELOCITY_X);
		} else if self.accel_x > 0.0 {
			self.velocity_x = cmp::min(self.velocity_x, MAX_VELOCITY_X);
		} else if self.on_ground() {
			self.velocity_x *= SLOWDOWN_VELOCITY;
		}

		// determine effects of gravity
		self.y += f64::round(
			self.velocity_y * elapsed_time_ms as f64
		) as i32;

		if !self.jump.active() {
			self.velocity_y = cmp::min(
				self.velocity_y + GRAVITY * elapsed_time_ms as f64, 
				MAX_VELOCITY_Y
			)
		}

		// TODO: HACK FLOOR
		if self.y > 320 {
			self.y = 320;
			self.velocity_y = 0.0;
		}
	}

	/// Instructs the current sprite-sheet to position itself
	/// at the coordinates specified by `coords:(x,y)`.
	fn set_position(&mut self, coords: (i32,i32)) {
		self.sprites.get_mut(&self.movement).set_position(coords);
	}
}

/// Proxies draw calls to underlying sprite
impl sprite::Drawable for Player {
	/// Draws current state to `display`
	fn draw(&self, display: &graphics::Graphics) {
		self.sprites.get(&self.movement).draw(display);
	}
}

/// Maintains track of a player's available `jump time`.
pub struct Jump {
	priv active: bool,
	priv time_remaining: sprite::Millis
}

impl Jump {
	/// Initializes a jump which is not active and has no time remaining.
	pub fn new() -> Jump {
		return Jump{
			active: false,
			time_remaining: sprite::Millis(0)
		};
	}

	/// Returns true if the jump is currently using up `jump time`.
	pub fn active(&self) -> bool {
		self.active
	}

	/// If the jump is active: `elapsed_time` will be removed from
	/// the jump's remaining time.
	pub fn update(&mut self, elapsed_time: sprite::Millis) {
		if self.active {
			self.time_remaining = {
				// unpack millis to do calcs
				let sprite::Millis(elapsed_time_ms) = elapsed_time;
				let sprite::Millis(remaining_ms) = self.time_remaining;

				sprite::Millis(remaining_ms - elapsed_time_ms)
			};

			// check overflow because `sprite::Millis` is unsigned.
			if self.time_remaining <= sprite::Millis(0) 
				|| self.time_remaining > JUMP_TIME {
				self.active = false;
			}
		}
	}

	/// Resets jump's remaining time to some constant factor.
	/// NOTE: this also activates the jump.
	pub fn reset(&mut self) {
		self.time_remaining = JUMP_TIME;
		self.reactivate();
	}

	/// Activates the jump if there is time remaining.
	pub fn reactivate(&mut self) {
		self.active = self.time_remaining > sprite::Millis(0);
	}

	/// Suspends the jump timer.
	pub fn deactivate(&mut self) {
		self.active = false;
	}
}
