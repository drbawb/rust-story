extern crate sdl2;

use std::rc::Rc;

use sdl2::rect::Rect;
use sdl2::render;

use game::graphics;

static BACKGROUND_SIZE: i32 = 128; //px

pub struct FixedBackdrop {
	resource: Rc<~render::Texture>
}

impl FixedBackdrop {
	pub fn new(	path: ~str, 
				graphics: &mut graphics::Graphics) 
				-> FixedBackdrop {

		let asset = graphics.load_image(path);

		FixedBackdrop { resource: asset }
	}

	pub fn draw(&self, graphics: &graphics::Graphics) {
		let width = 640;
		let height = 480;

		let mut x = 0;
		let mut y = 0;
		while x < width {
			while y < height {
				// draw background tile to screen
				let src = Rect::new(
					0, 0, 
					BACKGROUND_SIZE, BACKGROUND_SIZE
				);

				let dest = Rect::new(
					x, y, 
					BACKGROUND_SIZE, BACKGROUND_SIZE
				);

				graphics.blit_surface(
					*(self.resource.borrow()), 
					&src, &dest
				);

				// repeat
				y+= BACKGROUND_SIZE;
			}

			x += BACKGROUND_SIZE;
			y = 0;
		}

	}
}