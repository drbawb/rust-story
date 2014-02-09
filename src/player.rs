use std::f64;
use std::cmp;

use game::graphics;
use game::sprite;

static SLOWDOWN_VELOCITY: f64 	= 0.8;
static WALKING_ACCEL: f64 		= 0.0012;
static MAX_VELOCITY: f64 		= 0.325;

pub struct Player {
	priv sprite: ~sprite::AnimatedSprite,
	
	// positioning
	priv x: i16,
	priv y: i16,

	priv last_facing: sprite::Facing,

	// physics
	priv elapsed_time: sprite::Millis,
	priv velocity_x: f64,
	priv accel_x: f64
}

impl Player {
	pub fn new(graphics: &mut graphics::Graphics, x: i16, y: i16) -> Player {
		let mut quote;
		match sprite::AnimatedSprite::new(graphics, ~"assets/MyChar.bmp", (0,0), 3, 20) {
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
			elapsed_time: sprite::Millis(0),
			sprite: ~quote, 
			
			x: x, 
			y: y,
			last_facing: sprite::East,
			
			velocity_x: 0.0,
			accel_x: 0.0
		}
	}

	pub fn startMovingLeft(&mut self) {
		self.last_facing = sprite::West;
		self.sprite.state = sprite::SpriteState(sprite::Walking, self.last_facing);
		self.accel_x = -WALKING_ACCEL;
	}
	pub fn startMovingRight(&mut self) {
		self.last_facing = sprite::East;
		self.sprite.state = sprite::SpriteState(sprite::Walking, self.last_facing);
		self.accel_x = WALKING_ACCEL;
	}
	pub fn stopMoving(&mut self) {
		self.sprite.state = sprite::SpriteState(sprite::Standing, self.last_facing);
		self.accel_x = 0.0;
	}
}

/* Proxies for drawable sprite traits */
/// Proxies update calls to underlying sprite
impl sprite::Updatable for Player {
	//! Reads current time-deltas and mutates state accordingly.
	fn update(&mut self, elapsed_time: sprite::Millis) {
		// calculate current position
		self.elapsed_time = elapsed_time;
		self.set_position((self.x, 32));
		
		// calculate next position
		let sprite::Millis(elapsed_time_ms) = self.elapsed_time;
		self.x += f64::round(
			self.velocity_x * elapsed_time_ms as f64
		) as i16;



		// compute velocity for next frame
		self.velocity_x += 
			self.accel_x * elapsed_time_ms as f64;

		if (self.accel_x < 0.0) {
			self.velocity_x = cmp::max(self.velocity_x, -MAX_VELOCITY);
		} else if (self.accel_x > 0.0) {
			self.velocity_x = cmp::min(self.velocity_x, MAX_VELOCITY);
		} else {
			self.velocity_x *= SLOWDOWN_VELOCITY;
		}

		self.sprite.update(elapsed_time);
	}

	fn set_position(&mut self, coords: (i16,i16)) {
		self.sprite.set_position(coords);
	}
}

/// Proxies draw calls to underlying sprite
impl sprite::Drawable for Player {
	/// Draws current state to `display`
	fn draw(&self, display: &graphics::Graphics) {
		self.sprite.draw(display);
	}
}