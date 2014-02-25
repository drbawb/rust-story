use std::f64;

use collections::hashmap::HashMap;

use game::sprite;
use game::graphics;

use game::units;
use game::units::{AsGame};

static ANGULAR_VELOCITY: units::AngularVelocity 
	= units::AngularVelocity(120.0 / 1000.0); // 120 deg/sec, or .12 deg/ms

// location of first fluterring bat on sprite sheet
static X_OFFSET: units::Tile 	= units::Tile(2);
static Y_OFFSET: units::Tile 	= units::Tile(2);

// y-offsets for different horizontal facings.
static WEST_OFFSET: units::Tile = units::Tile(0);
static EAST_OFFSET: units::Tile = units::Tile(1);

static SPRITE_FRAMES: units::Frame	= 3;
static SPRITE_FPS: units::Fps 		= 15;

pub struct CaveBat {
	x: units::Game, 
	y: units::Game,

	flight_angle: units::Degrees,
	sprites: HashMap<sprite::Facing, ~sprite::Updatable>,
}

impl CaveBat {
	pub fn new(
		display: &mut graphics::Graphics, 
		x: units::Game, y: units::Game
	) -> CaveBat {
		let sprite_map = HashMap::<sprite::Facing, ~sprite::Updatable>::new();

		let mut new_bat = CaveBat { 
			x: x, y: y, 
			flight_angle: units::Degrees(0.0), 
			sprites: sprite_map
		};

		for facing in sprite::FACINGS.iter() {
			new_bat.load_sprite(display, *facing);
		}

		new_bat
	}

	fn load_sprite(&mut self, 
				   display: &mut graphics::Graphics, 
				   facing: sprite::Facing) {
		
		self.sprites.find_or_insert_with(facing, 
			|key| -> ~sprite::Updatable {
				let asset_path = ~"assets/base/Npc/NpcCemet.bmp";
				let sprite_x = X_OFFSET;
				let sprite_y = match facing {
					sprite::West => Y_OFFSET + WEST_OFFSET,
					sprite::East => Y_OFFSET + EAST_OFFSET,
				};

				~sprite::AnimatedSprite::new(
						display, asset_path, 
						(sprite_x, sprite_y), 
						(units::Tile(1), units::Tile(1)),
						SPRITE_FRAMES, SPRITE_FPS
					).unwrap() as ~sprite::Updatable
			}
		);
	}

	pub fn update(&mut self, elapsed_time: units::Millis) {
		let av: units::Degrees = ANGULAR_VELOCITY * elapsed_time;
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
