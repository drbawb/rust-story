use std::collections::hash_map::{Entry, HashMap};

use graphics::Graphics;
use sprite::{Motion, Facing, Looking};
use sprite::{self, Sprite, Drawable, Updatable};
use units::{self, AsGame};

// empty weapon for interacting and shit
static NULL_OFS_X: units::Tile = units::Tile(0);
static NULL_OFS_Y: units::Tile = units::Tile(0);

// start of weapon sprite-sheet column
static WEAPON_OFS_X: units::Tile = units::Tile(0);
static WEAPON_OFS_Y: units::Tile = units::Tile(6);

// y-ofs for directional facing
static F_WEST_OFS: units::Tile = units::Tile(0);
static F_EAST_OFS: units::Tile = units::Tile(1);

// y-ofs for Look-UP
static F_UP_OFS:   units::Tile = units::Tile(2);
static F_DOWN_OFS: units::Tile = units::Tile(4);

pub type MotionTup = (Motion, Facing, Looking);
type WeaponSprite = Box<sprite::Updatable<units::Game>>;

pub struct Weapon {
	sprites: HashMap<MotionTup, WeaponSprite>,

	x: units::Game,
	y: units::Game,

	movement: MotionTup,
}

impl Weapon {
	pub fn new(display: &mut Graphics) -> Weapon {
		// load weapons for every possible movement
		let weapon_map = HashMap::<MotionTup, WeaponSprite>::new();
		
		let mut new_weapon = Weapon {
			sprites: weapon_map,

			x: units::Game(0.0),
			y: units::Game(0.0),

			movement: (Motion::Standing, Facing::East, Looking::Horizontal)
		};

		for motion in sprite::MOTIONS.iter() {
			for facing in sprite::FACINGS.iter() {
				for looking in sprite::LOOKINGS.iter() {
					new_weapon.load_sprite(display, (*motion, *facing, *looking));
				}
			}
		}

		new_weapon
	}

	// make sure weapon is pulled from sprite table correctly.
	fn load_sprite(&mut self, display: &mut Graphics, movement: MotionTup) {
		let offset_coords = match movement {
			// gun only points east or west when looking @ background
			(Motion::Interacting, Facing::West, _) => { (WEAPON_OFS_X, WEAPON_OFS_Y+F_WEST_OFS) },
			(Motion::Interacting, Facing::East, _) => { (WEAPON_OFS_X, WEAPON_OFS_Y+F_EAST_OFS) },

			// gun can point east/west
			(_, Facing::West, Looking::Horizontal) => { (WEAPON_OFS_X, WEAPON_OFS_Y+F_WEST_OFS) },
			(_, Facing::East, Looking::Horizontal) => { (WEAPON_OFS_X, WEAPON_OFS_Y+F_EAST_OFS) },
			
			// or east/west both up & down
			(_, Facing::West, Looking::Up) => { (WEAPON_OFS_X, WEAPON_OFS_Y+F_WEST_OFS+F_UP_OFS) },
			(_, Facing::East, Looking::Up) => { (WEAPON_OFS_X, WEAPON_OFS_Y+F_EAST_OFS+F_UP_OFS) },

			(_, Facing::West, Looking::Down) => { (WEAPON_OFS_X, WEAPON_OFS_Y+F_WEST_OFS+F_DOWN_OFS) },
			(_, Facing::East, Looking::Down) => { (WEAPON_OFS_X, WEAPON_OFS_Y+F_EAST_OFS+F_DOWN_OFS) },
		};

		match self.sprites.entry(movement) {
			Entry::Vacant(entry) => {
				let file_path = format!("assets/base/Arms.bmp");
				let loaded_sprite = box sprite::Sprite::new(
					display,
					offset_coords,
					(units::HalfTile(3), units::HalfTile(2)),
					file_path
				) as Box<sprite::Updatable<_>>;

				entry.insert(loaded_sprite);
			},

			_ => {},
		};
	}

	pub fn set_motion(&mut self, movement: MotionTup) {
		self.movement = movement;
	}

	pub fn draw<C>(&mut self, 
	               display: &mut Graphics, 
	               coords: (C,C), 
	               flip_h: bool,
	               flip_v: bool)
	where C: units::AsGame {

		let (x,y) = coords;
		let (ox, oy) = match self.movement {
			(_, Facing::West, _) => (units::HalfTile(1), units::HalfTile(0)),
			(_, Facing::East, _) => (units::HalfTile(0),  units::HalfTile(0)),
		};

		let (fx, fy) = (x.to_game() - ox, y.to_game() - oy);

		let sprite = self.sprites.get_mut(&self.movement).unwrap();
		sprite.flip(flip_h, flip_v);
		sprite.draw(display, (fx, fy));
	}
}