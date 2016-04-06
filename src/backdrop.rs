use game;
use graphics;
use units;
use units::{AsPixel};

use sdl2::rect::Rect;

static BACKGROUND_SIZE: units::Tile = units::Tile(4);

pub struct FixedBackdrop {
	surface_id: String,
}

impl FixedBackdrop {
	pub fn new(path: String,
	               graphics: &mut graphics::Graphics) -> FixedBackdrop {
		graphics.load_image(path.clone(), false);
		FixedBackdrop { surface_id: path }
	}

	/// Repeatedly paints the asset across the entire screen.
	/// Moving the destination rectangle `BACKGROUND_SIZE` pixels
	/// in either direction as it progresses.
	pub fn draw(&mut self, graphics: &mut graphics::Graphics) {
		let (mut x, mut y) = (0i32,0i32);
		let units::Pixel(tile_size) = BACKGROUND_SIZE.to_pixel();

		while units::Pixel(x) < game::SCREEN_WIDTH.to_pixel() {
			while units::Pixel(y) < game::SCREEN_HEIGHT.to_pixel() {
				let src  = Rect::new(0, 0, tile_size as u32, tile_size as u32);
				let dest = Rect::new(x, y, tile_size as u32, tile_size as u32);

				graphics.blit_surface(&self.surface_id[..], &src, &dest);
				y+= tile_size as i32;
			}

			x += tile_size as i32;
			y = 0;
		}
	}
}
