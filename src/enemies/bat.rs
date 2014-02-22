pub use game::sprite;
pub use game::units;
pub use game::graphics;

static X_OFFSET: uint 	= 2;
static Y_OFFSET: uint 	= 3;
static SPRITE_FRAMES: uint	= 3;
static SPRITE_FPS: uint 	= 15;

pub struct CaveBat {
	coords: (units::Game, units::Game),
	sprite: ~sprite::Updatable,
}

impl CaveBat {
	pub fn new(
		display: &mut graphics::Graphics, 
		x: units::Game, y: units::Game
	) -> CaveBat {
		let asset_path = ~"assets/base/Npc/NpcCemet.bmp";
		let asset = ~sprite::AnimatedSprite::new(
						display, asset_path, 
						(units::Tile(X_OFFSET), units::Tile(Y_OFFSET)), 
						(units::Tile(1), units::Tile(1)),
						SPRITE_FRAMES, SPRITE_FPS
					).unwrap() as ~sprite::Updatable;

		CaveBat { coords: (x,y), sprite: asset }
	}

	pub fn update(&mut self, elapsed_time: units::Millis) {
		self.sprite.update(elapsed_time);
		self.sprite.set_position(self.coords);
	}

	pub fn draw(&self, display: &graphics::Graphics) {
		self.sprite.draw(display);
	}
}