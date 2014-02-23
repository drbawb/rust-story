use std::f64;

use game::sprite;
use game::graphics;

use game::units;
use game::units::{AsGame};

static ANGULAR_VELOCITY: units::AngularVelocity 
	= units::AngularVelocity(120.0 / 1000.0);

static X_OFFSET: units::Tile 	= units::Tile(2);
static Y_OFFSET: units::Tile 	= units::Tile(2);
static SPRITE_FRAMES: units::Frame	= 3;
static SPRITE_FPS: units::Fps 		= 15;

pub struct CaveBat {
	x: units::Game, 
	y: units::Game,

	flight_angle: units::Degrees,
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
						(X_OFFSET, Y_OFFSET), 
						(units::Tile(1), units::Tile(1)),
						SPRITE_FRAMES, SPRITE_FPS
					).unwrap() as ~sprite::Updatable;

		CaveBat { 
			x: x, y: y, 
			flight_angle: units::Degrees(0.0), 
			sprite: asset 
		}
	}

	pub fn update(&mut self, elapsed_time: units::Millis) {
		let av: units::Degrees = (ANGULAR_VELOCITY * elapsed_time);
		let amp: units::Game = // peak height of the wave in game units
			units::Tile(5).to_game() / units::Game(2.0);
		
		let wave: units::Game = 
			units::Game(
				f64::sin(self.flight_angle.to_radians())
			);
		
		self.flight_angle = self.flight_angle + av;
		let y1 = self.y + (amp * wave);

		self.sprite.update(elapsed_time);
		self.sprite.set_position((self.x, y1));
	}

	pub fn draw(&self, display: &graphics::Graphics) {
		self.sprite.draw(display);
	}
}