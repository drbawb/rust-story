extern mod extra;
extern mod sdl;


use self::extra::arc::Arc;
use game::graphics;

/// Milliseconds expressed as a large positive integer
/// This will be used at module boundaries in place of raw types.
pub struct Millis(uint);

pub trait Drawable { 
	fn draw(&self, display: &graphics::Graphics); 
}

pub trait Updatable { fn update(&mut self); }
pub trait Animatable : Updatable {
	fn step_time(&mut self, elapsed_time: Millis);
}

/// Represents a 32x32 2D character
/// This sprite will implm. a `Drawable` trait
pub struct Sprite {
	source_rect: sdl::sdl::Rect,
	sprite_sheet: Arc<~sdl::video::Surface>, 

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
	//! Reads current time-deltas and mutates state accordingly.
	fn update(&mut self) {
		self.source_rect = sdl::sdl::Rect::new((self.current_frame * 32) as i16, 0, 32, 32);
	}
}

impl Sprite {
	/// Loads character sprites from `assets/MyChar.bmp`
	/// `source_rect` acts as a viewport of this sprite-sheet.
	///
	/// Returns an error message if sprite-sheet could not be loaded.
	pub fn new(graphics: &mut graphics::Graphics, sheet_path: ~str, num_frames: int, fps: int) -> Result<Sprite, ~str> {
		// attempt to load sprite-sheet from `assets/MyChar.bmp`
		let origin = sdl::sdl::Rect::new(0, 0, 32, 32);
		let sheet = graphics.load_image(sheet_path); // request graphics subsystem cache this sprite.
		let sprite = Sprite{
			current_frame: 0, 
			elapsed_time: Millis(0),
			num_frames: (num_frames -1), 	// our frames are drawin w/ a 0-idx'd window.
			fps: fps,
			sprite_sheet: sheet, 	// "i made this" -- we own this side of the Arc()
			source_rect: origin
		};

		return Ok(sprite);

	}

	/// Draws this sprite at `x`, and `y` on `display`.
	pub fn draw_at(&self, display: &graphics::Graphics, x: i16, y: i16) {
		let dest_rect = sdl::sdl::Rect::new(x, y, 32, 32);
		display.blit_surface(*(self.sprite_sheet.get()), &self.source_rect, &dest_rect);
	}
}