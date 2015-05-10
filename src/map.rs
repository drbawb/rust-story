use std::cell::RefCell;
use std::iter::repeat;
use std::rc::Rc;
use std::sync::mpsc::Sender;

use backdrop;
use graphics;
use sprite;
use units;

use collisions::Rectangle;
use game::GameEvent;
use units::{AsGame,AsTile};

static ROWS: usize = 15; // 480
static COLS: usize = 20; // 640

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
pub struct Tile {
	pub tile_type:  TileType,
	pub ofs_x:      units::Game,
	sprite:         Option<TileSprite>,	
}

impl Tile {
	/// Creates n air tile w/ no sprite.
	fn new() -> Tile {
		Tile {
			tile_type: TileType::Air,
			sprite:    None,
			ofs_x:     units::Game(0.0),
		}
	}

	/// Creates a tile of `tile_type` initialized w/ its optional sprite.
	fn from_sprite(sprite: TileSprite,
	               tile_type: TileType) -> Tile {
		Tile {
			tile_type: tile_type,
			sprite:    Some(sprite.clone()),
			ofs_x:     units::Game(0.0),
		}
	}
}

pub struct Map {
	background:  backdrop::FixedBackdrop,
	sprites:     Vec<Vec<Tile>>,
	tiles:       Vec<Vec<Tile>>,

	event_chan: Sender<GameEvent>,
}

impl Map {
	pub fn new(display: &mut graphics::Graphics,
	           events:   Sender<GameEvent>,
	           level_bg: &str,
	           level_fg: &str) -> Map {

		let header = level_bg.lines()
		                     .take(1)
		                     .next()
		                     .expect("map background path missing");

      	// loader assets
      	let sheet = format!("assets/base/Stage/PrtCave.bmp");
		let wall = Rc::new(RefCell::new(
			box sprite::Sprite::new(
				display,
				(units::Tile(1) , units::Tile(0)),
				(units::Tile(1), units::Tile(1)),
				sheet.clone()
			) as Box<sprite::Updatable<_>>
		));

		let mut tile_rows = vec![];
		for line in level_fg.lines() {
			let line = line.trim();
			let mut col = vec![];

			for tile_ty in line.chars() {
				col.push(match tile_ty {
					'w' => { Tile::from_sprite(wall.clone(), TileType::Wall) },
					'.' => { Tile::new() },
					any   => { panic!("unknown tile type in map fg {}", any); },
				});
			}

			tile_rows.push(col);
		}

		let mut sprite_rows = vec![];
		for line in level_bg.lines().skip(1) {
			let line = line.trim();
			let mut col = vec![];

			for tile_ty in line.chars() {
				col.push(match tile_ty {
					'w' => { Tile::from_sprite(wall.clone(), TileType::Wall) },
					'.' => { Tile::new() },
					_   => { panic!("unknown tile type in map bg"); },
				});
			}

			sprite_rows.push(col);
		}

		Map {
			background: backdrop::FixedBackdrop::new(header.to_string(), display),
			sprites: sprite_rows,
			tiles:   tile_rows,

			event_chan: events,
		}

	}
	/// Will initialize a map (20 * 15) tiles:
	///
	/// * Most of these tiles will be `Air` tiles.
	/// * There are 15-tile high walls in the first and last columns. 
	/// * A small "obstacle course", 5-tiles wide, is placed about 2 tiles in.
	/// * A 3-tile high chain is placed on the left-side of this obstacle course.
	pub fn create_test_map(graphics: &mut graphics::Graphics, 
	                       events: Sender<GameEvent>) -> Map {

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

			event_chan: events,
		};

	
		// init `safety floors`
		for i in (0..COLS) {
			map.tiles[0][i]        = wall_tile.clone();
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
						            (units::Tile(b).to_game() + self.tiles[a][b].ofs_x,
						             units::Tile(a).to_game()));
;
					}
					_ => {}
				};
			}
		}
	}

	pub fn update(&mut self, elapsed_time: units::Millis) {
		let floor_velocity = units::Velocity(0.0);
		let delta_x = floor_velocity * elapsed_time;

		// shift last row of tiles by delta_x
		let last_row = self.tiles.len() - 1;
		for tile in (self.tiles[last_row].iter_mut()) {
			tile.ofs_x = tile.ofs_x + delta_x;
		}
	}

	/// Does a fast-check to see if a rectangle overlaps another rectangle
	pub fn hit_scan(&self, rectangle: &Rectangle) -> Vec<CollisionTile> {
		let units::Tile(first_row) =  rectangle.top().to_tile();
		let units::Tile(last_row) =  rectangle.bottom().to_tile();

	    // check tiles at delta position
	    let mut collision_tiles = vec![];
		for row_no in (first_row..last_row) {
			for col_no in (0..self.tiles[row_no].len()) {

				// compute tile's real position
				let tile = &self.tiles[row_no][col_no];
				let mut d_rect = Rectangle::new(
					units::Tile(1).to_game(), 
					units::Tile(1).to_game()
				);

				d_rect.x = tile.ofs_x + units::Tile(col_no);
				d_rect.y = units::Tile(row_no).to_game();

				if rectangle.collides_with(&d_rect) {
					collision_tiles.push( 
						CollisionTile::new(units::Tile(row_no), 
						                   units::Tile(col_no), 
						                   tile)
					);
				}
			}
		}

		collision_tiles


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
		
		// define player hitbox as tiles
		let units::Tile(first_row) =  rectangle.top().to_tile();
		let units::Tile(last_row)  =  rectangle.bottom().to_tile();
		let units::Tile(first_col) =  rectangle.left().to_tile();
		let units::Tile(last_col)  =  rectangle.right().to_tile();

		// bounds check for soft panic
		if    (first_row > ROWS) || (last_row+1 > ROWS)
		   || (first_col > COLS) || (last_col   > COLS) {

		   	let _ = self.event_chan.send(GameEvent::Panic);
		   	return vec![];
	    }

		// check tiles at delta position
		for row_no in (first_row..(last_row+1)) {
			for col_no in (0..self.tiles[row_no].len()) {

				// compute tile's real position
				let tile = &self.tiles[row_no][col_no];
				let mut d_rect = Rectangle::new(
					units::Tile(1).to_game(), 
					units::Tile(1).to_game()
				);

				d_rect.x = tile.ofs_x + units::Tile(col_no);
				d_rect.y = units::Tile(row_no).to_game();

				if rectangle.collides_with(&d_rect) {
					collision_tiles.push( 
						CollisionTile::new(units::Tile(row_no), 
						                   units::Tile(col_no), 
						                   tile)
					);
				}
			}
		}

		collision_tiles
	}
}
