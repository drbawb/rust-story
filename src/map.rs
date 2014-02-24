use std::vec;
use sync::RWArc;

use game::graphics;
use game::sprite;

use game::backdrop;
use game::collisions::Rectangle;
use game::units;
use game::units::{AsGame,AsTile};



#[deriving(Eq,Clone)]
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
		CollisionTile { tile_type: tile_type, row: row, col: col }
	}
}

// TODO: Conflicts w/ units::Tile, should probably have a different name.
#[deriving(Clone)]
struct Tile {
	tile_type: TileType,
	sprite: Option<RWArc<~sprite::Updatable:Freeze+Send>>
}

impl Tile {
	/// Creates n air tile w/ no sprite.
	fn new() -> Tile {
		Tile { tile_type: Air, sprite: None }
	}

	/// Creates a tile of `tile_type` initialized w/ its optional sprite.
	fn from_sprite(	sprite: RWArc<~sprite::Updatable:Freeze+Send>, 
			tile_type: TileType) -> Tile {
		Tile { tile_type: tile_type, sprite: Some(sprite) }
	}
}

pub struct Map {
	priv background: 	backdrop::FixedBackdrop,
	priv sprites:		~[~[Tile]],
	priv tiles: 		~[~[Tile]]
}

impl Map {
	/// Will initialize a map (20 * 15) tiles:
	///	
	/// * Most of these tiles will be `Air` tiles.
	/// * There are 15-tile high walls in the first and last columns. 
	/// * A small "obstacle course", 5-tiles wide, is placed about 2 tiles in.
	/// * A 3-tile high chain is placed on the left-side of this obstacle course.
	pub fn create_test_map(graphics: &mut graphics::Graphics) -> Map {
		static rows: uint = 15; // 480
		static cols: uint = 20; // 640

		let map_path = ~"assets/base/Stage/PrtCave.bmp";
		let sprite = RWArc::new(
			~sprite::Sprite::new(
				graphics, 
				(units::Game(0.0), units::Game(0.0)), 
				(units::Tile(1) , units::Tile(0)),
				(units::Tile(1), units::Tile(1)),
				map_path.clone()
			) as ~sprite::Updatable:Freeze+Send
		);

		let chain_top = RWArc::new(
			~sprite::Sprite::new(
				graphics, 
				(units::Game(0.0), units::Game(0.0)), 
				(units::Tile(11), units::Tile(2)),
				(units::Tile(1), units::Tile(1)),
				map_path.clone()
			) as ~sprite::Updatable:Freeze+Send
		);

		let chain_middle = RWArc::new(
			~sprite::Sprite::new(
				graphics, 
				(units::Game(0.0), units::Game(0.0)), 
				(units::Tile(12), units::Tile(2)),
				(units::Tile(1), units::Tile(1)),
				map_path.clone()
			) as ~sprite::Updatable:Freeze+Send
		);

		let chain_bottom = RWArc::new(
			~sprite::Sprite::new(
				graphics, 
				(units::Game(0.0), units::Game(0.0)), 
				(units::Tile(13), units::Tile(2)),
				(units::Tile(1), units::Tile(1)),
				map_path.clone()
			) as ~sprite::Updatable:Freeze+Send
		);

		let blank_tile = Tile::new();
		let wall_tile = Tile::from_sprite(sprite, Wall);
		let ct_tile = Tile::from_sprite(chain_top, Air);
		let cm_tile = Tile::from_sprite(chain_middle, Air);
		let cb_tile = Tile::from_sprite(chain_bottom, Air);

		let mut map = Map {
			background: backdrop::FixedBackdrop::new(
				~"assets/base/bkBlue.bmp", graphics
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
				match self.sprites[a][b].sprite {
					Some(ref elem) => {
						elem.write(|sprite| {
							sprite.set_position(
								(units::Tile(b).to_game(),
								 units::Tile(a).to_game()));

							sprite.draw(graphics);
						});
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
					Some(ref elem) => {
						elem.write(|sprite| {
							sprite.set_position(
								(units::Tile(b).to_game(), units::Tile(a).to_game())
							);

							sprite.draw(graphics);
						});
					}
					_ => {}
				};
			}
		}
	}

	pub fn update(&mut self, elapsed_time: units::Millis) {
		for row in self.tiles.iter() {
			for col in row.iter() {
				match col.sprite {
					Some(ref elem) => {
						elem.write(|sprite| {
							sprite.update(elapsed_time);
						});
					}
					_ => {}
				};
			}
		}
	}

	/// Checks if `Rectangle` is colliding with any tiles in the foreground.
	/// 
	/// NOTE: Checking a Rectangle which would be placed outside the tile-map
	/// results in a runtime failure!
	/// 
	/// NOTE: This is a simple check of the _outside bounds_ of the
	/// rectangle & tile. -- This method may claim that the player is 
	/// colliding w/ the edge of a tile that _appears to be_ empty space.
	pub fn get_colliding_tiles(&self, rectangle: &Rectangle) -> ~[CollisionTile] {
		let mut collision_tiles: ~[CollisionTile] = ~[];
		
		let units::Tile(first_row) 	= rectangle.top().to_tile();
		let units::Tile(last_row) 	= rectangle.bottom().to_tile();
		let units::Tile(first_col) 	= rectangle.left().to_tile();
		let units::Tile(last_col) 	= rectangle.right().to_tile();

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
