pub use game::sprite;
pub use game::units;
pub use game::graphics;

pub struct CaveBat {
	coords: (units::Game, units::Game),
	sprite: Option<~sprite::Updatable>,
}

impl CaveBat {
	pub fn new(
		graphics: &mut graphics::Graphics, 
		x: units::Game, y: units::Game
	) -> CaveBat {

		CaveBat { coords: (x,y), sprite: None }
	}

	pub fn draw(&self, display: &graphics::Graphics) {
		match self.sprite {
			Some(ref sprite) => { sprite.draw(display); }
			None => {}
		}
	}
}