use std::f64;

use collections::hashmap::HashMap;

use game::collisions::Rectangle;
use game::sprite;
use game::graphics;

use game::units;
use game::units::AsGame;

static ANGULAR_VELOCITY: units::AngularVelocity 
	= units::AngularVelocity(120.0 / 1000.0); // 120 deg/sec, or .12 deg/ms

// location of first fluterring bat on sprite sheet
static X_OFFSET: units::Tile = units::Tile(2);
static Y_OFFSET: units::Tile = units::Tile(2);

// y-offsets for different horizontal facings.
static WEST_OFFSET: units::Tile = units::Tile(0);
static EAST_OFFSET: units::Tile = units::Tile(1);

static SPRITE_FRAMES: units::Frame =  3;
static SPRITE_FPS: units::Fps      = 15;

pub struct CaveBat {
	x: units::Game,
	y: units::Game,
	origin: (units::Game, units::Game),

	flight_angle: units::Degrees,

	facing:   sprite::Facing,
	sprites:  HashMap<sprite::Facing, ~sprite::Updatable<units::Game>>,
}

impl CaveBat {
	pub fn new(display: &mut graphics::Graphics,
	           x: units::Game, y: units::Game) -> CaveBat {
		
		let sprite_map = HashMap::<sprite::Facing, ~sprite::Updatable<_>>::new();

		let mut new_bat = CaveBat { 
			x: x, y: y,
			origin: (x,y),

			facing:        sprite::West,
			flight_angle:  units::Degrees(0.0), 

			sprites: sprite_map,
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
			|key| -> ~sprite::Updatable<_> {
				let asset_path = ~"assets/base/Npc/NpcCemet.bmp";
				let sprite_x = X_OFFSET;
				let sprite_y = match *key {
					sprite::West => Y_OFFSET + WEST_OFFSET,
					sprite::East => Y_OFFSET + EAST_OFFSET,
				};

				~sprite::AnimatedSprite::new(
						display, asset_path, 
						(sprite_x, sprite_y), 
						(units::Tile(1), units::Tile(1)),
						SPRITE_FRAMES, SPRITE_FPS
					).unwrap() as ~sprite::Updatable<_>
			}
		);
	}

	pub fn damage_rectangle(&self) -> Rectangle {
		Rectangle {
			x: self.x + units::HalfTile(1), y: self.y + units::HalfTile(1),
			width: units::Game(0.0), height: units::Game(0.0),
		}
	}
	
	fn center_x(&self) -> units::Game {
		self.x + units::HalfTile(1)
	}

	pub fn update(&mut self, elapsed_time: units::Millis, player_x: units::Game) {
		let av: units::Degrees = ANGULAR_VELOCITY * elapsed_time;
		let amp: units::Game = // peak height of the wave in game units
			units::HalfTile(5).to_game();
		
		self.flight_angle = self.flight_angle + av;
		let wave: units::Game = 
			units::Game(
				f64::sin(self.flight_angle.to_radians())
			);

		let (_,y0) = self.origin;
		self.y = y0 + (amp * wave);

		self.facing = if self.center_x() > player_x 
			{ sprite::West } else { sprite::East };
		

		let sprite_ref = self.sprites.get_mut(&self.facing);
		sprite_ref.update(elapsed_time);
	}

	pub fn draw(&self, display: &graphics::Graphics) {
		self.sprites.get(&self.facing).draw(display, (self.x, self.y));
	}
}
