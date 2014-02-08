extern mod sdl;

use std::path::posix::Path;
use game::graphics;

/// Represents a 32x32 2D character
/// This sprite will implm. a `Drawable` trait
pub struct Sprite {
	source_rect: sdl::sdl::Rect,
	sprite_sheet: ~sdl::video::Surface
}

impl Sprite {
	/// Loads character sprites from `assets/MyChar.bmp`
	/// `source_rect` acts as a viewport of this sprite-sheet.
	///
	/// Returns an error message if sprite-sheet could not be loaded.
	pub fn new() -> Result<Sprite, ~str> {
		// attempt to load sprite-sheet from `assets/MyChar.bmp`
		let sprite_sheet = Path::new("assets/MyChar.bmp");
		if !(sprite_sheet.is_file()) {
			return Err(~"sprite file does not appear to be a regular file.");
		}

		let sprite_window = sdl::video::Surface::from_bmp(&sprite_sheet);
		match sprite_window {
			Ok(sheet) => {
				let origin = sdl::sdl::Rect::new(0, 0, 32, 32);
				let sprite = Sprite{sprite_sheet: sheet, source_rect: origin};
				return Ok(sprite);
			}
			Err(msg) => {return Err(msg);}
		}
	}

	pub fn draw(&self, display: &graphics::Graphics) {
		display.blit_surface(self.sprite_sheet, &self.source_rect);
	}
}