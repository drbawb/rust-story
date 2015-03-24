use graphics::Graphics;
use sprite::{Drawable, Sprite};
use units::{AsGame, Game};

pub struct NumberSprite {
	sprite: Option<Sprite>,
}

impl NumberSprite {
	pub fn new(graphics: &mut Graphics, number: i32) -> NumberSprite {
		// grab apropriate digit from spritesheet
		// 	(units::HalfTile(3), units::HalfTile(7)),
		// 	(units::HalfTile(1), units::HalfTile(1)),
		// 	format!("assets/base/TextBox.bmp"),
		// );

		NumberSprite { sprite: None }
	}
}

impl<C: AsGame> Drawable<C> for NumberSprite {
	fn draw(&mut self, display: &mut Graphics, coords: (C, C)) {
		// normalize coordinates
		let (x,y) = coords;
		let (x,y) = (x.to_game(), y.to_game());
	}
}
