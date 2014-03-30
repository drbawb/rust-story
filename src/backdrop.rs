use std::rc::Rc;

use game;
use game::graphics;
use game::units;
use game::units::{AsPixel};

use sdl2::rect::Rect;
use sdl2::render::Texture;


static BACKGROUND_SIZE: units::Tile = units::Tile(4);

pub struct FixedBackdrop {
	surface: Rc<~Texture>
}

impl FixedBackdrop {
	pub fn new(path: ~str, graphics: &mut graphics::Graphics) -> FixedBackdrop {
		let asset = graphics.load_image(path, false);
		FixedBackdrop { surface: asset }
	}

	/// Repeatedly paints the asset across the entire screen.
	/// Moving the destination rectangle `BACKGROUND_SIZE` pixels
	/// in either direction as it progresses.
	pub fn draw(&self, graphics: &graphics::Graphics) {
		let (mut x, mut y) = (0i32,0i32);
		let units::Pixel(tile_size) = BACKGROUND_SIZE.to_pixel();

		while units::Pixel(x) < game::SCREEN_WIDTH.to_pixel() {
			while units::Pixel(y) < game::SCREEN_HEIGHT.to_pixel() {
				let src  = Rect::new(0, 0, tile_size, tile_size);
				let dest = Rect::new(x, y, tile_size, tile_size);

				graphics.blit_surface(*self.surface, &src, &dest);
				y+= tile_size;
			}

			x += tile_size;
			y = 0;
		}
	}
}
