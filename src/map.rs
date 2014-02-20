use std::vec;
use std::rc::Rc;
use std::cell::RefCell;

use game::graphics;
use game::sprite;

use game::units;
use game::units::{AsGame,AsTile};

use game::backdrop;
use game::collisions::Rectangle;


#[deriving(Eq)]
pub enum TileType {
	Air,
	Wall
}

struct CollisionTile {
	pub tile_type: TileType,
	pub row: units::Tile,
	pub col: units::Tile
}

impl CollisionTile {
	pub fn new(row: units::Tile, col: units::Tile, tile_type: TileType) -> CollisionTile {
		CollisionTile {
			tile_type: tile_type,
			row: row, col: col
		}
	}
}

struct Tile {
	tile_type: TileType,
	sprite: Option<Rc<RefCell<~sprite::Updatable>>>
}

impl Tile {
	/// Creates n air tile w/ no sprite.
	fn new() -> Tile {
		Tile { tile_type: Air, sprite: None }
	}

	/// Creates a tile of `tile_type` with `sprite.`
	fn from_sprite(
		sprite: Rc<RefCell<~sprite::Updatable>>, tile_type: TileType
	) -> Tile {
		// Return tile with Some(sprite)
		Tile { tile_type: tile_type, sprite: Some(sprite) }
	}
}

pub struct Map {
	priv background: 	backdrop::FixedBackdrop,
	priv sprites:		~[~[Rc<Tile>]],
	priv tiles: 		~[~[Rc<Tile>]]
}

impl Map {
	pub fn create_test_map(graphics: &mut graphics::Graphics) -> Map {
		static num_rows: units::Tile = units::Tile(15); // 480
		static num_cols: units::Tile = units::Tile(20); // 640
		let (units::Tile(rows), units::Tile(cols)) = 
		(num_rows, num_cols);

		let sprite = Rc::new(
			RefCell::new(
				~sprite::Sprite::new(
					graphics, 
					(units::Game(0.0), units::Game(0.0)), 
					(units::Tile(1) , units::Tile(0)),
					(units::Tile(1), units::Tile(1)),
					~"assets/PrtCave.bmp"
				) as ~sprite::Updatable
			)
		);

		let chain_top = Rc::new(
			RefCell::new(
				~sprite::Sprite::new(
					graphics, 
					(units::Game(0.0), units::Game(0.0)), 
					(units::Tile(11), units::Tile(2)),
					(units::Tile(1), units::Tile(1)),
					~"assets/PrtCave.bmp"
				) as ~sprite::Updatable
			)
		);

		let chain_middle = Rc::new(
			RefCell::new(
				~sprite::Sprite::new(
					graphics, 
					(units::Game(0.0), units::Game(0.0)), 
					(units::Tile(12), units::Tile(2)),
					(units::Tile(1), units::Tile(1)),
					~"assets/PrtCave.bmp"
				) as ~sprite::Updatable
			)
		);

		let chain_bottom = Rc::new(
			RefCell::new(
				~sprite::Sprite::new(
					graphics, 
					(units::Game(0.0), units::Game(0.0)), 
					(units::Tile(13), units::Tile(2)),
					(units::Tile(1), units::Tile(1)),
					~"assets/PrtCave.bmp"
				) as ~sprite::Updatable
			)
		);

		let blank_tile = Rc::new(Tile::new());
		let wall_tile = Rc::new(Tile::from_sprite(sprite, Wall));
		let ct_tile = Rc::new(Tile::from_sprite(chain_top, Air));
		let cm_tile = Rc::new(Tile::from_sprite(chain_middle, Air));
		let cb_tile = Rc::new(Tile::from_sprite(chain_bottom, Air));

		let mut map = Map {
			background:	backdrop::FixedBackdrop::new(
				~"assets/bkBlue.bmp", graphics
			),
			sprites: vec::from_elem(rows,
				vec::from_elem(cols, blank_tile.clone())),
			tiles: vec::from_elem(rows,
				vec::from_elem(cols, blank_tile.clone()))
		};
	
		// init `floor`
		for i in range(0, cols) {
			map.tiles[rows - 1][i] = wall_tile.clone(); // store a reference
		}

		// "safety wall"
		for i in range (0, rows) {
			map.tiles[i][0] = wall_tile.clone();
			map.tiles[i][cols - 1] = wall_tile.clone();
		}


		map.tiles[rows - 2][3] 	= wall_tile.clone();
		map.tiles[rows - 2][5] 	= wall_tile.clone();
		
		map.tiles[rows - 3][4] 	= wall_tile.clone();
		map.tiles[rows - 4][3] 	= wall_tile.clone();
		map.tiles[rows - 5][2] 	= wall_tile.clone();

		map.sprites[rows - 4][2] = ct_tile.clone();
		map.sprites[rows - 3][2] = cm_tile.clone();
		map.sprites[rows - 2][2] = cb_tile.clone();
	
		map
	}

	pub fn draw_background(&self, graphics: &graphics::Graphics) {
		self.background.draw(graphics);
	}

	pub fn draw_sprites(&self, graphics: &graphics::Graphics) {
		for a in range(0, self.sprites.len()) {
			for b in range(0, self.sprites[a].len()) {
				match self.sprites[a][b].borrow().sprite {
					Some(ref elem) => {
						let mut sprite = elem.borrow().borrow_mut();
						let x = &units::Tile(b) as &units::AsGame;
						let y = &units::Tile(a) as &units::AsGame;

						sprite.get().set_position(
							(x.to_game(),
							 y.to_game()));

						sprite.get().draw(graphics);
					}
					_ => {}
				};
			}
		}
	}

	/// Draws current state to `display`
	pub fn draw(&self, graphics: &graphics::Graphics) {
		for a in range(0, self.tiles.len()) {
			for b in range(0, self.tiles[a].len()) {
				match self.tiles[a][b].borrow().sprite {
					Some(ref elem) => {
						let mut sprite = elem.borrow().borrow_mut();

						sprite.get().set_position(
							((&units::Tile(b) as &units::AsGame).to_game(),	
							 (&units::Tile(a) as &units::AsGame).to_game()));

						sprite.get().draw(graphics);
					}
					_ => {}
				};
			}
		}
	}

	pub fn update(&mut self, elapsed_time: units::Millis) {
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

	pub fn get_colliding_tiles(&self, rectangle: &Rectangle) -> ~[CollisionTile] {
		let first_row 	= rectangle.top().to_tile();
		let last_row 	= rectangle.bottom().to_tile();
		let first_col 	= rectangle.left().to_tile();
		let last_col 	= rectangle.right().to_tile();

		let mut collision_tiles: ~[CollisionTile] = ~[];
		let units::Tile(start_row) = first_row;
		let units::Tile(end_row) = last_row;

		let units::Tile(start_col) = first_col;
		let units::Tile(end_col) = last_col;

		for row in range(start_row, end_row + 1) {
			for col in range(start_col, end_col + 1) {
				collision_tiles.push( 
					CollisionTile::new(units::Tile(row), units::Tile(col), self.tiles[row][col].borrow().tile_type)
				);
			}
		}

		collision_tiles
	}
}
