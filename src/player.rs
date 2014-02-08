use game::graphics;
use game::sprite;

pub struct Player {
	priv sprite: ~sprite::Sprite,
	priv x: i16,
	priv y: i16
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

		Player{sprite: ~quote, x: x, y: y}
	}
}

/* Proxies for drawable sprite traits */

/// Proxies animation calls to underlying sprite
impl sprite::Animatable for Player {
	fn step_time(&mut self, elapsed_time: sprite::Millis) {
		self.sprite.step_time(elapsed_time);
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