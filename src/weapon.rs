use std::collections::hash_map::{Entry, HashMap};
use std::cell::RefCell;
use std::rc::Rc;

use collisions::Rectangle;
use graphics::Graphics;
use map::{self, TileType};
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

// bullet offsets
static B_OFS_RIGHT: units::Tile = units::Tile(0);
static  B_OFS_DOWN: units::Tile = units::Tile(1);
static  B_OFS_LEFT: units::Tile = units::Tile(2);
static    B_OFS_UP: units::Tile = units::Tile(3);

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

#[derive(Copy, Clone)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Clone)]
pub struct Bullet {
	// sprite
	sprite_up:    Rc<RefCell<Sprite>>,
	sprite_down:  Rc<RefCell<Sprite>>,
	sprite_left:  Rc<RefCell<Sprite>>,
	sprite_right: Rc<RefCell<Sprite>>,

	// physics & positioning
	x: units::Game,
	y: units::Game,
	vx: units::Velocity,
	vy: units::Velocity,

	// states
	collided:  bool,
	direction: Direction,
}

impl Bullet {
	pub fn new(display: &mut Graphics) -> Bullet {
		let file_path = "assets/base/Bullet.bmp";
		
		let ox = units::Tile(4);
		let oy = units::Tile(1);
		
		let ow = units::Tile(1);
		let oh = units::Tile(1);

		// load the facings for the four ordinal directions
		let b_right = sprite::Sprite::new(
			display,
			(ox + B_OFS_RIGHT, oy),
			(ow, oh),
			file_path.to_string()
		);

		let b_down = sprite::Sprite::new(
			display,
			(ox + B_OFS_DOWN, oy),
			(ow, oh),
			file_path.to_string()
		);

		let b_left = sprite::Sprite::new(
			display,
			(ox + B_OFS_LEFT, oy),
			(ow, oh),
			file_path.to_string()
		);

		let b_up = sprite::Sprite::new(
			display,
			(ox + B_OFS_UP, oy),
			(ow, oh),
			file_path.to_string()
		);

		Bullet {
			// sprite
			sprite_up:    Rc::new(RefCell::new(   b_up)),
			sprite_down:  Rc::new(RefCell::new( b_down)),
			sprite_left:  Rc::new(RefCell::new( b_left)),
			sprite_right: Rc::new(RefCell::new(b_right)),

			// physics & positioning
			x:      units::Game(0.0),
			y:      units::Game(0.0),
			vx: units::Velocity(0.0),
			vy: units::Velocity(0.0),

			// states
			collided:  false,
			direction: Direction::Right,
		}
	}

	pub fn set_coords<C>(&mut self, coords: (C, C))
	where C: AsGame {
		let (x,y) = coords;
		self.x = x.to_game();
		self.y = y.to_game();
	}

	pub fn set_velocity(&mut self, 
	                    velocity: (units::Velocity, units::Velocity),
	                    direction: Direction) {

		let (vx,vy) = velocity;

		self.direction = direction;
		self.vx = vx;
		self.vy = vy;
	}

	pub fn is_off_screen(&self) -> bool {
		let off_x = self.x > units::Tile(20).to_game();
		let off_y = self.y > units::Tile(15).to_game();

		self.collided || (off_x || off_y)
	}

	pub fn draw(&mut self, display: &mut Graphics) {
		let mut sprite_ref = match self.direction {
			Direction::Up    => {    self.sprite_up.borrow_mut() },
			Direction::Down  => {  self.sprite_down.borrow_mut() },
			Direction::Left  => {  self.sprite_left.borrow_mut() },
			Direction::Right => { self.sprite_right.borrow_mut() },
		};

     	sprite_ref.draw(display, (self.x, self.y));
	}

	// integrate time over vx and vy to compute new positions
	pub fn update(&mut self, elapsed_time: units::Millis, tile_map: &mut map::Map) {
		let dx = self.vx * elapsed_time;
		let dy = self.vy * elapsed_time;

		let nx = self.x + dx;
		let ny = self.y + dy;

		// quick check collision at new position
		let mut hit_pos = None;
		let nrect = Rectangle {
			x: nx,
			y: ny,
			width:  units::Tile(1).to_game(),
			height: units::Tile(1).to_game(),
		};

		{ // borrow tiles for collision checking
			let tiles = tile_map.hit_scan(&nrect);
			
			for collision in tiles {
				self.collided = collision.tile.tile_type != TileType::Air;
				if self.collided { 
					hit_pos = Some((collision.row, collision.col));
					break;
				}
			}
		}

		// apply damage to the first tile we hit
		if let Some((units::Tile(row), units::Tile(col))) = hit_pos {
			tile_map.take_damage(row, col);
		}

		if !self.collided { self.x = nx; self.y = ny; }
	}
}