use std::collections::hash_map::{HashMap, Entry};
use num::Float;

use collisions::Rectangle;
use sprite::{self, Facing};
use graphics;

use units;
use units::AsGame;

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

	facing:   Facing,
	sprites:  HashMap<sprite::Facing, Box<sprite::Updatable<units::Game>>>,
}

impl CaveBat {
	pub fn new(display: &mut graphics::Graphics,
	               x: units::Game, y: units::Game) -> CaveBat {
		
		let sprite_map = HashMap::<sprite::Facing, Box<sprite::Updatable<_>>>::new();

		let mut new_bat = CaveBat { 
			x: x, y: y,
			origin: (x,y),

			facing:        Facing::West,
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

		match self.sprites.entry(facing) {
			Entry::Vacant(entry) => {
				let asset_path = format!("assets/base/Npc/NpcCemet.bmp");
				let sprite_x = X_OFFSET;
				let sprite_y = match facing {
					Facing::West => Y_OFFSET + WEST_OFFSET,
					Facing::East => Y_OFFSET + EAST_OFFSET,
				};

				entry.insert(Box::new(sprite::AnimatedSprite::new(
						display, asset_path, 
						(sprite_x, sprite_y), 
						(units::Tile(1), units::Tile(1)),
						SPRITE_FRAMES, SPRITE_FPS
					).unwrap()) as Box<sprite::Updatable<_>>);
			},
			_ => {},
		};
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
				(*self.flight_angle).to_radians().sin()
			);

		let (_,y0) = self.origin;
		self.y = y0 + (amp * wave);

		self.facing = if self.center_x() > player_x 
			{ Facing::West } else { Facing::East };
		
		self.sprites.get_mut(&self.facing).unwrap().update(elapsed_time);
	}

	pub fn draw(&mut self, display: &mut graphics::Graphics) {
		self.sprites.get_mut(&self.facing).unwrap().draw(display, (self.x, self.y));
	}
}
