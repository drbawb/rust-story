extern crate sdl2;

use sync::Arc;

use sdl2::rect::Rect;
use sdl2::render::Texture;

use game;
use game::graphics;
use game::units;

static BACKGROUND_SIZE: units::Tile = 4;

pub struct FixedBackdrop {
	surface: Arc<~Texture>
}

impl FixedBackdrop {
	pub fn new(	path: ~str, 
				graphics: &mut graphics::Graphics) 
				-> FixedBackdrop {

		let asset = graphics.load_image(path, false);

		FixedBackdrop { surface: asset }
	}

	/// Repeatedly paints the asset across the entire screen.
	/// Moving the destination rectangle `BACKGROUND_SIZE` pixels
	/// in either direction as it progresses.
	pub fn draw(&self, graphics: &graphics::Graphics) {
		let (mut x, mut y) = (0i32,0i32);
		while x < units::tile_to_pixel(game::SCREEN_WIDTH) {
			while y < units::tile_to_pixel(game::SCREEN_HEIGHT) {
				let src = Rect::new(
					0, 0, 
					units::tile_to_pixel(BACKGROUND_SIZE), 
					units::tile_to_pixel(BACKGROUND_SIZE));

				let dest = Rect::new(
					x, y,
					units::tile_to_pixel(BACKGROUND_SIZE),
					units::tile_to_pixel(BACKGROUND_SIZE)
				);

				graphics.blit_surface(*(self.surface.get()), &src, &dest);
				y+= units::tile_to_pixel(BACKGROUND_SIZE);
			}

			x += units::tile_to_pixel(BACKGROUND_SIZE);
			y = 0;
		}
	}
}
