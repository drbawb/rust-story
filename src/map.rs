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

		let mut test_map = Map {
			foreground_sprites: vec::from_elem(num_rows,
				vec::from_elem(num_cols, None)
			)
		};
	
		let cave_tile = Rc::new(
				RefCell::new(
				~sprite::Sprite::new(
					graphics, 
					(0,0), 
					(0,1),
					~"assets/PrtCave.bmp"
				) as ~sprite::Updatable:
			)
		);

		// init very top row
		for i in range(0, num_cols) {
			test_map.foreground_sprites[0][i] = Some(cave_tile.clone()); // store a reference
		}

		test_map
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

	fn set_position(&mut self, coords: (i32,i32)) {
		for row in self.foreground_sprites.iter() {
			for col in row.iter() {
				match *col {
					Some(ref elem) => {
						let mut sprite = elem.borrow().borrow_mut();
						sprite.get().set_position(coords);
					}
					_ => {}
				};
			}
		}
	}
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
