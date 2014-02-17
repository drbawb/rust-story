extern crate sdl2;

use std::rc::Rc;

use sdl2::rect::Rect;
use sdl2::render::Texture;

use game;
use game::graphics;

static BACKGROUND_SIZE: i32 = 128; //px

pub struct FixedBackdrop {
	surface: Rc<~Texture>
}

impl FixedBackdrop {
	pub fn new(	path: ~str, 
				graphics: &mut graphics::Graphics) 
				-> FixedBackdrop {

		let asset = graphics.load_image(path);

		FixedBackdrop { surface: asset }
	}

	/// Repeatedly paints the asset across the entire screen.
	/// Moving the destination rectangle `BACKGROUND_SIZE` pixels
	/// in either direction as it progresses.
	pub fn draw(&self, graphics: &graphics::Graphics) {
		let (mut x, mut y) = (0,0);
		while x < game::SCREEN_WIDTH {
			while y < game::SCREEN_HEIGHT {
				let src = Rect::new(0, 0, BACKGROUND_SIZE, BACKGROUND_SIZE);
				let dest = Rect::new(
					x as i32, y as i32, 
					BACKGROUND_SIZE, BACKGROUND_SIZE
				);

				graphics.blit_surface(*(self.surface.borrow()), &src, &dest);
				y+= BACKGROUND_SIZE as int;
			}

			x += BACKGROUND_SIZE as int;
			y = 0;
		}
	}
}