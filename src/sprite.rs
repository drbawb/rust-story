extern mod sdl;

use std::path::posix::Path;
use game::graphics;

// todo: units library?

pub struct Millis(uint);

pub trait Drawable { fn draw(&self, display: &graphics::Graphics); }
pub trait Updatable { fn update(&mut self); }
pub trait Animatable : Updatable {
	fn step_time(&mut self, elapsed_time: Millis);
}

/// Represents a 32x32 2D character
/// This sprite will implm. a `Drawable` trait
pub struct Sprite {
	source_rect: sdl::sdl::Rect,
	sprite_sheet: ~sdl::video::Surface,

	priv current_frame: int,
	priv num_frames: int,
	priv fps: int,

	priv elapsed_time: Millis
}

impl Animatable for Sprite {
	fn step_time(&mut self, elapsed_time: Millis) {
		let frame_time = (1000 /self.fps) as uint;
		
		// unpack milliseconds to do integer math
		// then store the result
		let Millis(world_elapsed) = elapsed_time;
		let Millis(mut last_elapsed) = self.elapsed_time;
		last_elapsed += world_elapsed;
		self.elapsed_time = Millis(last_elapsed);


		if (last_elapsed > frame_time) {
			// reset timer when we move frame-pointer
			self.elapsed_time = Millis(0); 
			
			// increment frame if it doesn't overflow num_frames
			self.current_frame = if self.current_frame >= self.num_frames {
				0
			} else {
				self.current_frame + 1
			};
		}
	}
}

impl Updatable for Sprite {
	/// Reads current time-deltas and mutates state accordingly.
	fn update(&mut self) {
		self.source_rect = sdl::sdl::Rect::new((self.current_frame * 32) as i16, 0, 32, 32);
	}
}

impl Drawable for Sprite {
	/// Draws current state to `display`
	fn draw(&self, display: &graphics::Graphics) {
		display.blit_surface(self.sprite_sheet, &self.source_rect);
	}
}

impl Sprite {
	/// Loads character sprites from `assets/MyChar.bmp`
	/// `source_rect` acts as a viewport of this sprite-sheet.
	///
	/// Returns an error message if sprite-sheet could not be loaded.
	pub fn new(num_frames: int, fps: int) -> Result<Sprite, ~str> {
		// attempt to load sprite-sheet from `assets/MyChar.bmp`
		let sprite_sheet = Path::new("assets/MyChar.bmp");
		if !(sprite_sheet.is_file()) {
			return Err(~"sprite file does not appear to be a regular file.");
		}

		let sprite_window = sdl::video::Surface::from_bmp(&sprite_sheet);
		match sprite_window {
			Ok(sheet) => {
				let origin = sdl::sdl::Rect::new(0, 0, 32, 32);
				let sprite = Sprite{
					current_frame: 0, 
					elapsed_time: Millis(0),
					num_frames: (num_frames -1), // our frames are drawin w/ a 0-idx'd window.
					fps: fps,
					sprite_sheet: sheet, 
					source_rect: origin
				};
				return Ok(sprite);
			}
			Err(msg) => {return Err(msg);}
		}
	}
}