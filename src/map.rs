use std::cell::RefCell;
use std::iter::repeat;
use std::rc::Rc;
use std::sync::mpsc::Sender;

use backdrop;
use graphics;
use sprite::{self, Drawable, Updatable};
use units;

use collisions::Rectangle;
use game::GameEvent;
use units::{AsGame,AsTile};

static ROWS: usize = 15; // 480
static COLS: usize = 20; // 640

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum TileType {
	Exit,
	Air,
	GDown,
	GUp,
	
	Destructible,
	Wall,
	
	Spikes,
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
	pub tile_type: TileType,
	pub ofs_x:     units::Game,

	hp:     i64,
	sprite: Option<TileSprite>,	
}

impl Tile {
	/// Creates n air tile w/ no sprite.
	fn new() -> Tile {
		Tile {
			tile_type: TileType::Air,
			sprite:    None,
			ofs_x:     units::Game(0.0),
			hp:        1,
		}
	}

	/// Creates a tile of `tile_type` initialized w/ its optional sprite.
	fn from_sprite(sprite: TileSprite,
	               tile_type: TileType) -> Tile {
		Tile {
			tile_type: tile_type,
			sprite:    Some(sprite.clone()),
			ofs_x:     units::Game(0.0),
			hp:        1,
		}
	}

	/// HP delta to be applied to actors on collision
	pub fn do_damage(&self) -> i64 {
		match self.tile_type {
			TileType::Spikes => { 100 },
			_ => { 0 },
		}
	}

	/// Yields damage to tile, if applicable
	fn take_damage(&mut self) {
		match self.tile_type {
			TileType::Destructible => { self.hp -= 1 },
			_ => {},
		};

		if self.hp <= 0 {
			println!("tile destroyed holy shit!");
		}
	}
}

pub struct Map {
	background:  backdrop::FixedBackdrop,
	sprites:     Vec<Vec<Tile>>,
	tiles:       Vec<Vec<Tile>>,

	event_chan: Sender<GameEvent>,
	spawn: (units::Tile, units::Tile),
}

impl Map {
	pub fn new(display: &mut graphics::Graphics,
	           events:   Sender<GameEvent>,
	           level_bg: &str,
	           level_fg: &str) -> Map {

		let header = level_bg.lines()
		                     .take(1)
		                     .next()
		                     .expect("map background path missing")
		                     .trim();

      	// loader assets
      	let sheet = format!("assets/base/Stage/PrtJail.bmp");
      	let wall = Rc::new(RefCell::new(
			box sprite::Sprite::new(
				display,
				(units::Tile(1) , units::Tile(0)),
				(units::Tile(1), units::Tile(1)),
				sheet.clone()
			) as Box<sprite::Updatable<_>>
  		));

		let dwall = Rc::new(RefCell::new(
			box sprite::Sprite::new(
				display,
				(units::Tile(3) , units::Tile(0)),
				(units::Tile(1), units::Tile(1)),
				sheet.clone()
			) as Box<sprite::Updatable<_>>
  		));

		let spikes_up = Rc::new(RefCell::new(
			box sprite::Sprite::new(
				display,
				(units::Tile(12) , units::Tile(4)),
				(units::Tile(1), units::Tile(1)),
				sheet.clone()
			) as Box<sprite::Updatable<_>>
		));

		let spikes_down = Rc::new(RefCell::new(
			box sprite::Sprite::new(
				display,
				(units::Tile(12) , units::Tile(5)),
				(units::Tile(1), units::Tile(1)),
				sheet.clone()
			) as Box<sprite::Updatable<_>>
		));

		let sheet = format!("assets/base/Stage/PrtAlmond.bmp");
		let gdown_brick = Rc::new(RefCell::new(
			box sprite::Sprite::new(
				display,
				(units::Tile(5) , units::Tile(3)),
				(units::Tile(1), units::Tile(1)),
				sheet.clone()
			) as Box<sprite::Updatable<_>>
		));

		let gup_brick = Rc::new(RefCell::new(
			box sprite::Sprite::new(
				display,
				(units::Tile(5) , units::Tile(3)),
				(units::Tile(1), units::Tile(1)),
				sheet.clone()
			) as Box<sprite::Updatable<_>>
		));

		let exit = Rc::new(RefCell::new(
			box sprite::Sprite::new(
				display,
				(units::Tile(13) , units::Tile(1)),
				(units::Tile(1), units::Tile(1)),
				sheet.clone()
			) as Box<sprite::Updatable<_>>
		));

		gup_brick.borrow_mut().flip(false, true);

		let mut spawn_pos = (units::Tile(0), units::Tile(0));
		let mut tile_rows = vec![];
		for (row_no,line) in level_fg.lines().enumerate() {
			let line = line.trim();
			let mut col = vec![];

			for (col_no, tile_ty) in line.chars().enumerate() {
				col.push(match tile_ty {
					'w' => { Tile::from_sprite(wall.clone(),          TileType::Wall) },
					'b' => { Tile::from_sprite(dwall.clone(), TileType::Destructible) },
					
					'^' => { Tile::from_sprite(spikes_up.clone(),     TileType::Spikes) },
					'v' => { Tile::from_sprite(spikes_down.clone(),   TileType::Spikes) },

					's' => {
						spawn_pos = (units::Tile(col_no), units::Tile(row_no));
						Tile::new()
					}
					
					'd' => { Tile::from_sprite(gdown_brick.clone(), TileType::GDown) },
					'u' => { Tile::from_sprite(gup_brick.clone(), TileType::GUp) },

					'x' => { Tile::from_sprite(exit.clone(), TileType::Exit) },

					'.' => { Tile::new() },
					_ => { Tile::new() },
					//any   => { panic!("unknown tile type in map fg {}", any); },
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
					_   => { Tile::new() },
				});
			}

			sprite_rows.push(col);
		}

		let header_path = header.to_string();
		println!("loading backgroudn: {}", header_path);

		Map {
			background: backdrop::FixedBackdrop::new(header_path, display),
			sprites: sprite_rows,
			tiles:   tile_rows,

			event_chan: events,
			spawn: spawn_pos,
		}

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
				if self.tiles[a][b].hp <= 0 { self.tiles[a][b] = Tile::new() }
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

	pub fn spawn_pos(&self) -> (units::Tile, units::Tile) { self.spawn }

	/// Does a fast-check to see if a rectangle overlaps another rectangle
	/// Scans the entire row.
	pub fn hit_scan(&mut self, rectangle: &Rectangle) -> Vec<CollisionTile> {
		let units::Tile(row_no) = rectangle.top().to_tile();

	    // check tiles at delta position
	    let mut collision_tiles = vec![];
		for col_no in (0..self.tiles[row_no].len()) {
			// compute tile's real position
			let tile = &self.tiles[row_no][col_no];				
			let mut d_rect = Rectangle::new(
				units::Tile(1).to_game(), 
				units::Tile(1).to_game()
			);

			d_rect.x = tile.ofs_x + units::Tile(col_no);
			d_rect.y = units::Tile(row_no).to_game();

			if d_rect.collides_with(rectangle) {
				collision_tiles.push(
					CollisionTile::new(units::Tile(row_no), 
					                   units::Tile(col_no), 
					                   tile)
				);
			}
		}

		collision_tiles
	}

	/// Transfers damage to the tile at (row,col)
	pub fn take_damage(&mut self, row: usize, col: usize) {
		self.tiles[row][col].take_damage();
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
