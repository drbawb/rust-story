use std::cell::RefCell;
use std::iter::repeat;
use std::rc::Rc;

use backdrop;
use graphics;
use sprite;
use units;

use collisions::Rectangle;
use units::{AsGame,AsTile};

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum TileType {
	Air,
	Wall
}

#[derive(Clone,Copy)]
pub struct CollisionTile<'c> {
	pub tile: 		&'c Tile,
	pub row:        units::Tile,
	pub col:        units::Tile
}

impl<'c> CollisionTile<'c> {
	pub fn new(row:  units::Tile, 
	           col:  units::Tile, 
	           tile: &'c Tile) -> CollisionTile<'c> {
		CollisionTile { tile: tile, row: row, col: col }
	}
}

type TileSprite = Rc<RefCell<Box<sprite::Updatable<units::Game>>>>;

// TODO: Conflicts w/ units::Tile, should probably have a different name.
#[derive(Clone)]
struct Tile {
	tile_type:  TileType,
	sprite:     Option<TileSprite>
}

impl Tile {
	/// Creates n air tile w/ no sprite.
	fn new() -> Tile {
		Tile { tile_type: TileType::Air, sprite: None }
	}

	/// Creates a tile of `tile_type` initialized w/ its optional sprite.
	fn from_sprite(sprite: TileSprite,
	               tile_type: TileType) -> Tile {
		Tile { tile_type: tile_type, sprite: Some(sprite.clone()) }
	}
}

pub struct Map {
	background:  backdrop::FixedBackdrop,
	sprites:     Vec<Vec<Tile>>,
	tiles:       Vec<Vec<Tile>>,
}

impl Map {
	/// Will initialize a map (20 * 15) tiles:
	///
	/// * Most of these tiles will be `Air` tiles.
	/// * There are 15-tile high walls in the first and last columns. 
	/// * A small "obstacle course", 5-tiles wide, is placed about 2 tiles in.
	/// * A 3-tile high chain is placed on the left-side of this obstacle course.
	pub fn create_test_map(graphics: &mut graphics::Graphics) -> Map {
		static ROWS: usize = 15; // 480
		static COLS: usize = 20; // 640

		let map_path =  format!("assets/base/Stage/PrtCave.bmp");
		let sprite   =  Rc::new(RefCell::new(
			box sprite::Sprite::new(
				graphics,
				(units::Tile(1) , units::Tile(0)),
				(units::Tile(1), units::Tile(1)),
				map_path.clone()
			) as Box<sprite::Updatable<_>>
		));

		let chain_top = Rc::new(RefCell::new(
			box sprite::Sprite::new(
				graphics,
				(units::Tile(11), units::Tile(2)),
				(units::Tile(1), units::Tile(1)),
				map_path.clone()
			) as Box<sprite::Updatable<_>>
		));

		let chain_middle = Rc::new(RefCell::new(
			box sprite::Sprite::new(
				graphics,
				(units::Tile(12), units::Tile(2)),
				(units::Tile(1), units::Tile(1)),
				map_path.clone()
			) as Box<sprite::Updatable<_>>
		));

		let chain_bottom = Rc::new(RefCell::new(
			box sprite::Sprite::new(
				graphics, 
				(units::Tile(13), units::Tile(2)),
				(units::Tile(1), units::Tile(1)),
				map_path.clone()
			) as Box<sprite::Updatable<_>>
		));

		let blank_tile = Tile::new();
		let wall_tile  = Tile::from_sprite(sprite, TileType::Wall);
		let ct_tile    = Tile::from_sprite(chain_top, TileType::Air);
		let cm_tile    = Tile::from_sprite(chain_middle, TileType::Air);
		let cb_tile    = Tile::from_sprite(chain_bottom, TileType::Air);

		let blank_row: Vec<Tile> = repeat(blank_tile).take(COLS).collect();

		let mut map = Map {
			background: backdrop::FixedBackdrop::new(
				format!("assets/base/bkBlue.bmp"), graphics
			),
			sprites: repeat(blank_row.clone()).take(ROWS).collect(),
			tiles: repeat(blank_row.clone()).take(ROWS).collect(),
		};

	
		// init `floor`
		for i in (0..COLS) {
			map.tiles[ROWS - 1][i] = wall_tile.clone();
		}

		// "safety wall"
		for i in (0..ROWS) {
			map.tiles[i][0]        = wall_tile.clone();
			map.tiles[i][COLS - 1] = wall_tile.clone();
		}


		map.tiles[ROWS - 2][3] = wall_tile.clone();
		map.tiles[ROWS - 2][5] = wall_tile.clone();

		map.tiles[ROWS - 3][4] = wall_tile.clone();
		map.tiles[ROWS - 4][3] = wall_tile.clone();
		map.tiles[ROWS - 5][2] = wall_tile.clone();

		map.sprites[ROWS - 4][2] = ct_tile.clone();
		map.sprites[ROWS - 3][2] = cm_tile.clone();
		map.sprites[ROWS - 2][2] = cb_tile.clone();
	
		map
	}

	pub fn draw_background(&mut self, graphics: &mut graphics::Graphics) {
		self.background.draw(graphics);
	}

	pub fn draw_sprites(&mut self, graphics: &mut graphics::Graphics) {
		for a in (0..self.sprites.len()) {
			for b in (0..self.sprites[a].len()) {
				match self.sprites[a][b].sprite {
					Some(ref sprite) => {
						sprite.borrow_mut()
						      .draw(graphics, 
						            (units::Tile(b).to_game(),
						             units::Tile(a).to_game()));
					}
					_ => {}
				};
			}
		}
	}

	/// Draws current state to `display`
	pub fn draw(&mut self, graphics: &mut graphics::Graphics) {
		for a in (0..self.tiles.len()) {
			for b in (0..self.tiles[a].len()) {
				match self.tiles[a][b].sprite {
					Some(ref sprite) => {
						sprite.borrow_mut()
						      .draw(graphics,
						            (units::Tile(b).to_game(),
						             units::Tile(a).to_game()));
;
					}
					_ => {}
				};
			}
		}
	}


	/// no-op for demo map
	pub fn update(&mut self, _elapsed_time: units::Millis) {
		/* 
		 * This was effectively unused and IMHO does not warrant the
		 * complexity introduced by using dynamic borrow-ck'ing.
		 * 
		 * As most background sprites are shared [in this demo map] any
		 * animations would look really goofy as all tiles would
		 * advance their frames in perfect sync.
		 */
	}

	/// Checks if `Rectangle` is colliding with any tiles in the foreground.
	/// 
	/// NOTE: Checking a Rectangle which would be placed outside the tile-map
	/// results in a runtime failure!
	/// 
	/// NOTE: This is a simple check of the _outside bounds_ of the
	/// rectangle & tile. -- This method may claim that the player is 
	/// colliding w/ the edge of a tile that _appears to be_ empty space.
	pub fn get_colliding_tiles(&self, rectangle: &Rectangle) -> Vec<CollisionTile> {
		let mut collision_tiles: Vec<CollisionTile> = Vec::new();
		
		let units::Tile(first_row) =  rectangle.top().to_tile();
		let units::Tile(last_row)  =  rectangle.bottom().to_tile();
		let units::Tile(first_col) =  rectangle.left().to_tile();
		let units::Tile(last_col)  =  rectangle.right().to_tile();

		for row in (first_row..(last_row + 1)) {
			for col in (first_col..(last_col + 1)) {
				collision_tiles.push( 
					CollisionTile::new(units::Tile(row), units::Tile(col), self.tiles[row][col].tile_type)
				);
			}
		}

		collision_tiles
	}
}
