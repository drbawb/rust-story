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

pub struct Player {
	priv sprites: HashMap<(sprite::Motion,sprite::Facing), ~sprite::Updatable>,
	
	// positioning
	priv x: i16,
	priv y: i16,
	priv movement: (sprite::Motion, sprite::Facing),

	// physics
	priv elapsed_time: sprite::Millis,
	priv velocity_x: f64,
	priv velocity_y: f64,
	priv accel_x: f64,

	priv jump: Jump
}

impl Player {
	pub fn new(graphics: &mut graphics::Graphics, x: i16, y: i16) -> Player {
		// insert sprites into map
		let mut sprite_map = HashMap::<(sprite::Motion,sprite::Facing), ~sprite::Updatable>::new();
		
		// walking
		/* graphics: &mut graphics::Graphics, 
		coords: (i16,i16), 
		offset: (i16,i16), 
		file_name: ~str
		*/
		sprite_map.insert(
			(sprite::Standing, sprite::West),
			~sprite::Sprite::new(graphics, (0,0), (0,0), ~"assets/MyChar.bmp") as ~sprite::Updatable
		);
		sprite_map.insert(
			(sprite::Standing, sprite::East),
			~sprite::Sprite::new(graphics, (0,0), (0, 1), ~"assets/MyChar.bmp") as ~sprite::Updatable
		);
		
		sprite_map.insert(
			(sprite::Walking, sprite::West),
			~sprite::AnimatedSprite::new(graphics, ~"assets/MyChar.bmp", (0,0), 3, 20).unwrap() as ~sprite::Updatable
		);
		sprite_map.insert(
			(sprite::Walking, sprite::East),
			~sprite::AnimatedSprite::new(graphics, ~"assets/MyChar.bmp", (0,1), 3, 20).unwrap() as ~sprite::Updatable
		);

		println!("map has been init to {:?}", (sprite::Standing as int, sprite::East as int));
		Player{
			elapsed_time: sprite::Millis(0),
			sprites: sprite_map,

			x: x, 
			y: y,
			movement: (sprite::Standing, sprite::East),
			
			velocity_x: 0.0,
			velocity_y: 0.0,
			accel_x: 0.0,

			jump: Jump::new()
		}
	}

	pub fn start_moving_left(&mut self) {
		self.movement = (sprite::Walking, sprite::West);
		self.accel_x = -WALKING_ACCEL;
	}
	pub fn start_moving_right(&mut self) {
		self.movement = (sprite::Walking, sprite::East);
		self.accel_x = WALKING_ACCEL;
	}
	pub fn stop_moving(&mut self) {
		let (_, last_facing) = self.movement; // copy last facing
		self.movement = (sprite::Standing, last_facing);
		self.accel_x = 0.0;
	}

	pub fn start_jump(&mut self) {
		if self.on_ground() {
			self.jump.reset();
			self.velocity_y = -JUMP_SPEED;
		} else if (self.velocity_y < 0.0) {
			self.jump.reactivate();
		}
	}

	pub fn stop_jump(&mut self) {
		self.velocity_y = 0.0;
	}

	pub fn on_ground(&self) -> bool {		
		true
	}
}

/* Proxies for drawable sprite traits */
/// Proxies update calls to underlying sprite
impl sprite::Updatable for Player {
	//! Reads current time-deltas and mutates state accordingly.
	fn update(&mut self, elapsed_time: sprite::Millis) {
		// calculate current position
		self.elapsed_time = elapsed_time;
		self.jump.update(elapsed_time);

		// update sprite
		self.set_position((self.x, self.y));
		self.sprites.get_mut(&self.movement).update(elapsed_time);

		// calculate next position
		let sprite::Millis(elapsed_time_ms) = self.elapsed_time;
		self.x += f64::round(
			self.velocity_x * elapsed_time_ms as f64
		) as i16;

		// compute velocity x for next frame
		self.velocity_x += 
			self.accel_x * elapsed_time_ms as f64;

		if (self.accel_x < 0.0) {
			self.velocity_x = cmp::max(self.velocity_x, -MAX_VELOCITY_X);
		} else if (self.accel_x > 0.0) {
			self.velocity_x = cmp::min(self.velocity_x, MAX_VELOCITY_X);
		} else {
			self.velocity_x *= SLOWDOWN_VELOCITY;
		}

		self.y += f64::round(
			self.velocity_y * elapsed_time_ms as f64
		) as i16;

		if self.jump.active() {
			// LOL DONT DO NOTHIN
		} else {
			self.velocity_y = cmp::min(
				self.velocity_y + GRAVITY * elapsed_time_ms as f64, 
				MAX_VELOCITY_Y
			)
		}

		
	}

	fn set_position(&mut self, coords: (i16,i16)) {
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

pub struct Jump {
	priv active: bool,
	priv time_remaining: sprite::Millis
}

impl Jump {
	pub fn new() -> Jump {
		return Jump{
			active: false,
			time_remaining: sprite::Millis(0)
		};
	}

	pub fn active(&self) -> bool {
		self.active
	}

	pub fn update(&mut self, elapsed_time: sprite::Millis) {
		if self.active {
			self.time_remaining = {
				// unpack millis to do calcs
				let sprite::Millis(elapsed_time_ms) = elapsed_time;
				let sprite::Millis(remaining_ms) = self.time_remaining;

				sprite::Millis(remaining_ms - elapsed_time_ms)
			};

			if self.time_remaining <= sprite::Millis(0) {
				self.active = false;
			}
		}
	}

	pub fn reset(&mut self) {
		self.time_remaining = JUMP_TIME;
		self.reactivate();
	}

	pub fn reactivate(&mut self) {
		self.active = self.time_remaining > sprite::Millis(0);
	}
	
	pub fn deactivate(&mut self) {
		self.active = false;
	}
}