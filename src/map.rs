use std::vec;
use std::rc::Rc;
use std::cell::RefCell;

use game::graphics;
use game::sprite;
use game::collisions::Rectangle;

priv enum TileType {
	Air,
	Wall
}

struct CollisionTile {
	tile_type: TileType,
	row: int, col: int
}

impl CollisionTile {
	pub fn new(row: int, col: int, tile_type: TileType) -> CollisionTile {
		CollisionTile {
			tile_type: tile_type,
			row: row, col: col
		}
	}
}

struct Tile {
	tile_type: TileType,
	sprite: Option<Rc<RefCell<~sprite::Updatable:>>>
}

impl Tile {
	/// Creates n air tile w/ no sprite.
	fn new() -> Tile {
		Tile {
			tile_type: Air,
			sprite: None
		}
	}

	/// Creates a tile of `tile_type` with `sprite.`
	fn from_sprite(
		sprite: Rc<RefCell<~sprite::Updatable:>>, 
		tile_type: TileType
	) -> Tile {
		// Return tile with Some(sprite)
		Tile {
			tile_type: tile_type,
			sprite: Some(sprite)
		}
	}
}

pub struct Map {
	priv tiles: ~[~[Rc<Tile>]]
}

impl Map {
	pub fn create_test_map(graphics: &mut graphics::Graphics) -> Map {
		static num_rows: uint = 15; // 480
		static num_cols: uint = 20; // 640

		let sprite = Rc::new(
			RefCell::new(
				~sprite::Sprite::new(
					graphics, 
					(0,0), 
					(1,0),
					~"assets/PrtCave.bmp"
				) as ~sprite::Updatable:
			)
		);

		let blank_tile = Rc::new(Tile::new());
		let wall_tile = Rc::new(Tile::from_sprite(sprite, Wall));
		
		let mut map = Map {
			tiles: vec::from_elem(num_rows,
				vec::from_elem(num_cols, blank_tile.clone())
			)
		};
	
		// init very top row
		for i in range(0, num_cols) {
			map.tiles[11][i] = wall_tile.clone(); // store a reference
		}

		map.tiles[10][3] 	= wall_tile.clone();
		map.tiles[10][5] 	= wall_tile.clone();
		
		map.tiles[9][4] 	= wall_tile.clone();
		map.tiles[8][3] 	= wall_tile.clone();
		map.tiles[7][2] 	= wall_tile.clone();
		
		map
	}

	pub fn get_colliding_tiles(&mut self, rectangle: &Rectangle) -> ~[CollisionTile] {
		let first_row 	= rectangle.top() / sprite::TILE_SIZE as int;
		let last_row 	= rectangle.bottom() / sprite::TILE_SIZE as int;
		let first_col 	= rectangle.left() 	/ sprite::TILE_SIZE as int;
		let last_col 	= rectangle.right() / sprite::TILE_SIZE as int;

		let mut collision_tiles: ~[CollisionTile] = ~[];
		for row in range(first_row, last_row + 1) {
			for col in range(first_col, last_col + 1) {
				collision_tiles.push( 
					CollisionTile::new(row, col, self.tiles[row][col].borrow().tile_type)
				);
			}
		}

		fail!("unimplemented.")
	}
}

impl sprite::Updatable for Map {
	fn update(&mut self, elapsed_time: sprite::Millis) {
		for row in self.tiles.iter() {
			for col in row.iter() {
				match col.borrow().sprite {
					Some(ref elem) => {
						let mut sprite = elem.borrow().borrow_mut();
						sprite.get().update(elapsed_time);
					}
					_ => {}
				};
			}
		}
	}

	#[allow(unused_variable)]
	fn set_position(&mut self, coords: (i32,i32)) {}
}

impl sprite::Drawable for Map {
	/// Draws current state to `display`
	fn draw(&self, graphics: &graphics::Graphics) {
		for a in range(0, self.tiles.len()) {
			for b in range(0, self.tiles[a].len()) {
				match self.tiles[a][b].borrow().sprite {
					Some(ref elem) => {
						// draw sprite at x,y coordinates.
						// a => row (y-axis)
						// b => col (x-axis)
						let mut sprite = elem.borrow().borrow_mut();
						sprite.get().set_position(
							(
								(b * sprite::TILE_SIZE as uint) as i32,
								(a * sprite::TILE_SIZE as uint) as i32
							)
						);

						sprite.get().draw(graphics);
					}
					_ => {}
				};
			}
		}
	}
}
