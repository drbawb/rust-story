use std::rc::Rc;

use backdrop;
use graphics;
use sprite;
use units;

use collisions::Rectangle;
use units::{AsGame,AsTile};

#[deriving(PartialEq,Eq,Clone)]
pub enum TileType {
	Air,
	Wall
}

pub struct CollisionTile {
	pub tile_type:  TileType,
	pub row:        units::Tile,
	pub col:        units::Tile
}

impl CollisionTile {
	pub fn new(row: units::Tile, col: units::Tile, 
	           tile_type: TileType) -> CollisionTile {
		CollisionTile { tile_type: tile_type, row: row, col: col }
	}
}

// TODO: Conflicts w/ units::Tile, should probably have a different name.
#[deriving(Clone)]
struct Tile {
	tile_type:  TileType,
	sprite:     Option<Rc<Box<sprite::Updatable<units::Game>>>>
}

impl Tile {
	/// Creates n air tile w/ no sprite.
	fn new() -> Tile {
		Tile { tile_type: Air, sprite: None }
	}

	/// Creates a tile of `tile_type` initialized w/ its optional sprite.
	fn from_sprite(sprite: Rc<Box<sprite::Updatable<units::Game>>>,
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
		static ROWS: uint = 15; // 480
		static COLS: uint = 20; // 640

		let map_path =  format!("assets/base/Stage/PrtCave.bmp");
		let sprite   =  Rc::new(
			box sprite::Sprite::new(
				graphics,
				(units::Tile(1) , units::Tile(0)),
				(units::Tile(1), units::Tile(1)),
				map_path.clone()
			) as Box<sprite::Updatable<_>>
		);

		let chain_top = Rc::new(
			box sprite::Sprite::new(
				graphics,
				(units::Tile(11), units::Tile(2)),
				(units::Tile(1), units::Tile(1)),
				map_path.clone()
			) as Box<sprite::Updatable<_>>
		);

		let chain_middle = Rc::new(
			box sprite::Sprite::new(
				graphics,
				(units::Tile(12), units::Tile(2)),
				(units::Tile(1), units::Tile(1)),
				map_path.clone()
			) as Box<sprite::Updatable<_>>
		);

		let chain_bottom = Rc::new(
			box sprite::Sprite::new(
				graphics, 
				(units::Tile(13), units::Tile(2)),
				(units::Tile(1), units::Tile(1)),
				map_path.clone()
			) as Box<sprite::Updatable<_>>
		);

		let blank_tile = Tile::new();
		let wall_tile = Tile::from_sprite(sprite, Wall);
		let ct_tile = Tile::from_sprite(chain_top, Air);
		let cm_tile = Tile::from_sprite(chain_middle, Air);
		let cb_tile = Tile::from_sprite(chain_bottom, Air);

		let mut map = Map {
			background: backdrop::FixedBackdrop::new(
				format!("assets/base/bkBlue.bmp"), graphics
			),
			sprites: Vec::from_elem(ROWS,
				 Vec::from_elem(COLS, blank_tile.clone())),
			tiles: Vec::from_elem(ROWS,
			       Vec::from_elem(COLS, blank_tile.clone()))
		};
	
		// init `floor`
		for i in range(0, COLS) {
			*(map.tiles
			     .get_mut(ROWS - 1)
			     .get_mut(i)) = wall_tile.clone(); // store a reference
		}

		// "safety wall"
		for i in range (0, ROWS) {
			*(map.tiles.get_mut(i).get_mut(0))        = wall_tile.clone();
			*(map.tiles.get_mut(i).get_mut(COLS - 1)) = wall_tile.clone();
		}


		*(map.tiles.get_mut(ROWS - 2).get_mut(3)) = wall_tile.clone();
		*(map.tiles.get_mut(ROWS - 2).get_mut(5)) = wall_tile.clone();

		*(map.tiles.get_mut(ROWS - 3).get_mut(4)) = wall_tile.clone();
		*(map.tiles.get_mut(ROWS - 4).get_mut(3)) = wall_tile.clone();
		*(map.tiles.get_mut(ROWS - 5).get_mut(2)) = wall_tile.clone();

		*(map.sprites.get_mut(ROWS - 4).get_mut(2)) = ct_tile.clone();
		*(map.sprites.get_mut(ROWS - 3).get_mut(2)) = cm_tile.clone();
		*(map.sprites.get_mut(ROWS - 2).get_mut(2)) = cb_tile.clone();
	
		map
	}

	pub fn draw_background(&self, graphics: &graphics::Graphics) {
		self.background.draw(graphics);
	}

	pub fn draw_sprites(&self, graphics: &graphics::Graphics) {
		for a in range(0, self.sprites.len()) {
			for b in range(0, self.sprites[a].len()) {
				match self.sprites[a][b].sprite {
					Some(ref sprite) => {
						sprite.draw(graphics, 
						            (units::Tile(b).to_game(),
						             units::Tile(a).to_game()));
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
				match self.tiles[a][b].sprite {
					Some(ref sprite) => {
						sprite.draw(graphics,
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
	#[allow(unused_variable)]
	pub fn update(&mut self, elapsed_time: units::Millis) {
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

		for row in range(first_row, last_row + 1) {
			for col in range(first_col, last_col + 1) {
				collision_tiles.push( 
					CollisionTile::new(units::Tile(row), units::Tile(col), self.tiles[row][col].tile_type)
				);
			}
		}

		collision_tiles
	}
}
