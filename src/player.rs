use std::f64;
use std::cmp;
use std::hashmap::HashMap;

use game::graphics;
use game::sprite;

static SLOWDOWN_VELOCITY: f64 	= 0.8;
static WALKING_ACCEL: f64 		= 0.0012;
static MAX_VELOCITY: f64 		= 0.325;

pub struct Player {
	priv sprites: HashMap<(sprite::Motion,sprite::Facing), ~sprite::Updatable>,
	
	// positioning
	priv x: i16,
	priv y: i16,
	priv movement: (sprite::Motion, sprite::Facing),
	priv last_facing: sprite::Facing,

	// physics
	priv elapsed_time: sprite::Millis,
	priv velocity_x: f64,
	priv accel_x: f64
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
			last_facing: sprite::East,
			
			velocity_x: 0.0,
			accel_x: 0.0
		}
	}

	pub fn startMovingLeft(&mut self) {
		self.movement = (sprite::Walking, sprite::West);
		self.last_facing = sprite::West;
		self.accel_x = -WALKING_ACCEL;
	}
	pub fn startMovingRight(&mut self) {
		self.movement = (sprite::Walking, sprite::East);
		self.last_facing = sprite::East;
		self.accel_x = WALKING_ACCEL;
	}
	pub fn stopMoving(&mut self) {
		self.movement = (sprite::Standing, self.last_facing);
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
		self.set_position((self.x, self.y));

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

		// mut-ref the struct and update its time
		self.sprites.get_mut(&self.movement).update(elapsed_time);
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