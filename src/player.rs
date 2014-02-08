use std::f64;
use std::cmp;

use game::graphics;
use game::sprite;

static SLOWDOWN_VELOCITY: f64 	= 0.8;
static WALKING_ACCEL: f64 		= 0.0012;
static MAX_VELOCITY: f64 		= 0.325;

pub struct Player {
	priv sprite: ~sprite::Sprite,
	
	priv x: i16,
	priv y: i16,

	priv velocity_x: f64,
	priv accel_x: f64
}

impl Player {
	pub fn new(x: i16, y: i16) -> Player {
		let mut quote;
		match sprite::Sprite::new(~"assets/MyChar.bmp", 3, 20) {
			Ok(loaded_sprite) => {
				quote = loaded_sprite;
				println!("sprite = ok");
			}
			Err(msg) => {
				println!("sprite err: {}", msg); 
				fail!("cannot create player w/o sprite resources");
			}
		}

		Player{
			sprite: ~quote, 
			
			x: x, 
			y: y,
			
			velocity_x: 0.0,
			accel_x: 0.0
		}
	}

	pub fn startMovingLeft(&mut self) {
		self.accel_x = -WALKING_ACCEL;
	}
	pub fn startMovingRight(&mut self) {
		self.accel_x = WALKING_ACCEL;
	}
	pub fn stopMoving(&mut self) {
		self.accel_x = 0.0;
	}
}

/* Proxies for drawable sprite traits */

/// Proxies animation calls to underlying sprite
impl sprite::Animatable for Player {
	fn step_time(&mut self, elapsed_time: sprite::Millis) {
		self.sprite.step_time(elapsed_time);
		
		let sprite::Millis(elapsed_time_ms) = elapsed_time;
		self.x += f64::round(
			self.velocity_x * elapsed_time_ms as f64
		) as i16;

		self.velocity_x += 
			self.accel_x * elapsed_time_ms as f64;

		if (self.accel_x < 0.0) {
			self.velocity_x = cmp::max(self.velocity_x, -MAX_VELOCITY);
		} else if (self.accel_x > 0.0) {
			self.velocity_x = cmp::min(self.velocity_x, MAX_VELOCITY);
		} else {
			self.velocity_x *= SLOWDOWN_VELOCITY;
		}
	}
}

/// Proxies update calls to underlying sprite
impl sprite::Updatable for Player {
	//! Reads current time-deltas and mutates state accordingly.
	fn update(&mut self) {
		self.sprite.update();
	}
}

/// Proxies draw calls to underlying sprite
impl sprite::Drawable for Player {
	/// Draws current state to `display`
	fn draw(&self, display: &graphics::Graphics) {
		self.sprite.draw_at(display, self.x, self.y);
	}
}