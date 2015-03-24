use graphics::Graphics;
use sprite::{Drawable, Sprite};
use units::{self, AsGame};

pub struct NumberSprite {
	sprite: Sprite,
}

impl NumberSprite {
	pub fn new(graphics: &mut Graphics, number: i32) -> NumberSprite {
		let digit = Sprite::new(
			graphics,
			(units::HalfTile(3), units::HalfTile(7)),
			(units::HalfTile(1), units::HalfTile(1)),
			format!("assets/base/TextBox.bmp"),
		);

		NumberSprite { sprite: digit }
	}
}

impl<C: AsGame> Drawable<C> for NumberSprite {
	fn draw(&mut self, display: &mut Graphics, coords: (C, C)) {
		self.sprite.draw(display, coords);
	}
}
