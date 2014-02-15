use std::vec;
use std::rc::Rc;
use std::cell::RefCell;

use game::graphics;
use game::sprite;

pub struct Map {
	priv foreground_sprites: ~[~[Option<Rc<RefCell<~sprite::Updatable:> > >]]
}




impl Map {
	pub fn create_test_map(graphics: &mut graphics::Graphics) -> Map {
		static num_rows: uint = 15; // 480
		static num_cols: uint = 20; // 640

		let tile = Rc::new(
			RefCell::new(
				~sprite::Sprite::new(
					graphics, 
					(0,0), 
					(1,0),
					~"assets/PrtCave.bmp"
				) as ~sprite::Updatable:
			)
		);
		
		let mut map = Map {
			foreground_sprites: vec::from_elem(num_rows,
				vec::from_elem(num_cols, None)
			)
		};
	
		// init very top row
		for i in range(0, num_cols) {
			map.foreground_sprites[11][i] = Some(tile.clone()); // store a reference
		}

		map.foreground_sprites[10][3] 	= Some(tile.clone());
		map.foreground_sprites[10][5] 	= Some(tile.clone());
		
		map.foreground_sprites[9][4] 	= Some(tile.clone());
		map.foreground_sprites[8][3] 	= Some(tile.clone());
		map.foreground_sprites[7][2] 	= Some(tile.clone());
		
		map
	}
}

impl sprite::Updatable for Map {
	fn update(&mut self, elapsed_time: sprite::Millis) {
		for row in self.foreground_sprites.iter() {
			for col in row.iter() {
				match *col {
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
		for a in range(0, self.foreground_sprites.len()) {
			for b in range(0, self.foreground_sprites[a].len()) {
				match self.foreground_sprites[a][b] {
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
