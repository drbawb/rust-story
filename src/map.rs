use std::vec;
use std::rc::Rc;
use std::cell::RefCell;

use game::graphics;
use game::sprite;

pub struct Map {
	priv foreground_sprites: ~[~[Option<Rc<RefCell<~sprite::Updatable:> > >]]
}




fn new_tile(graphics: &mut graphics::Graphics) -> Option<Rc<RefCell<~sprite::Updatable:>>> {
	Some(
		Rc::new(
			RefCell::new(
				~sprite::Sprite::new(
					graphics, 
					(0,0), 
					(2,0),
					~"assets/PrtCave.bmp"
				) as ~sprite::Updatable:
			)
		)
	)
}

impl Map {
	pub fn create_test_map(graphics: &mut graphics::Graphics) -> Map {
		static num_rows: uint = 15; // 480
		static num_cols: uint = 20; // 640

		let mut map = Map {
			foreground_sprites: vec::from_elem(num_rows,
				vec::from_elem(num_cols, None)
			)
		};
	
		// init very top row
		for i in range(0, num_cols) {
			map.foreground_sprites[11][i] = new_tile(graphics); // store a reference
		}

		map.foreground_sprites[10][5] 	= new_tile(graphics);
		map.foreground_sprites[9][4] 	= new_tile(graphics);
		map.foreground_sprites[8][3] 	= new_tile(graphics);
		map.foreground_sprites[7][2] 	= new_tile(graphics);
		map.foreground_sprites[10][3] 	= new_tile(graphics);


		map
	}
}

impl sprite::Updatable for Map {
	fn update(&mut self, elapsed_time: sprite::Millis) {
		for x in range(0, self.foreground_sprites.len()) {
			for y in range(0, self.foreground_sprites[x].len()) {
				match self.foreground_sprites[x][y] {
					Some(ref elem) => {
						let mut sprite = elem.borrow().borrow_mut();
						sprite.get().update(elapsed_time);
						sprite.get().set_position(
							(
								(y * sprite::TILE_SIZE as uint) as i32, 
								(x * sprite::TILE_SIZE as uint) as i32
							)
						);
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
		for row in self.foreground_sprites.iter() {
			for col in row.iter() {
				match *col {
					Some(ref elem) => {
						let mut sprite = elem.borrow().borrow_mut();
						sprite.get().draw(graphics);
					}
					_ => {}
				};
			}
		}
	}
}
