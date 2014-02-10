extern mod extra;
extern mod sdl;


use self::extra::arc::Arc;
use game::graphics;

static TILE_SIZE: i16 = 32;


#[deriving(IterBytes,Eq)]
pub enum Motion {
	Walking,
	Standing
}

#[deriving(IterBytes,Eq)]
pub enum Facing {
	North,
	West,
	South,
	East
}

/// Milliseconds expressed as a large positive integer
/// This will be used at module boundaries in place of raw types.
#[deriving(Ord)]
pub struct Millis(uint);


/// Any object which can be represented in 2D space
pub trait Drawable { 
	fn draw(&self, display: &graphics::Graphics); 
}

/// Any object which understands time and placement in 2D space.
pub trait Updatable : Drawable { 
	fn update(&mut self, elapsed_time: Millis); 
	fn set_position(&mut self, coords: (i16,i16));
}

/// Represents a static 32x32 2D character
pub struct Sprite {
	sprite_sheet: Arc<~sdl::video::Surface>, 
	source_rect: sdl::sdl::Rect,
	coords: (i16,i16)
}

impl Drawable for Sprite {
	/// Draws selfs @ coordinates provided by 
	fn draw(&self, display: &graphics::Graphics) {
		let (x,y) = self.coords;
		let dest_rect = sdl::sdl::Rect::new(x, y, 32, 32);
		display.blit_surface(*(self.sprite_sheet.get()), &self.source_rect, &dest_rect);
	}
}

#[allow(unused_variable)]
impl Updatable for Sprite {
	fn update(&mut self, elapsed_time: Millis) {
		// no-op for static sprite.
	}

	fn set_position(&mut self, coords: (i16,i16)) {
		self.coords = coords;
	}
}

impl Sprite {
	/// A new sprite which will draw itself at `coords`
	/// `sprite_at` is the index (row) where the sprite starts in `file_name`
	pub fn new(
		graphics: &mut graphics::Graphics, 
		coords: (i16,i16), 
		offset: (i16,i16), 
		file_name: ~str
	) -> Sprite {
		let (a,b) = offset;
		let origin = sdl::sdl::Rect::new(a * TILE_SIZE, b * TILE_SIZE, 32, 32);

		let sheet = graphics.load_image(file_name); // request graphics subsystem cache this sprite.

		let sprite = Sprite{
			sprite_sheet: sheet,
			source_rect: origin,
			coords: coords
		};

		sprite
	}
}

/// Represents a 32x32 2D character w/ a number of frames
/// Frames will be selected based on time-deltas supplied through update
pub struct AnimatedSprite {
	source_rect: sdl::sdl::Rect,
	sprite_sheet: Arc<~sdl::video::Surface>, 

	priv coords: (i16, i16),
	priv offset: (i16,i16),
	priv current_frame: i16,
	priv num_frames: i16,
	priv fps: int,

	priv elapsed_time: Millis
}

impl Updatable for AnimatedSprite {
	//! Reads current time-deltas and mutates state accordingly.
	fn update(&mut self, elapsed_time: Millis) {
		let frame_time = (1000 /self.fps) as uint;
		
		// unpack milliseconds to do integer math
		// then store the result
		let Millis(world_elapsed) = elapsed_time;
		let Millis(mut last_elapsed) = self.elapsed_time;
		last_elapsed += world_elapsed;
		self.elapsed_time = Millis(last_elapsed);


		// determine next frame
		if (last_elapsed > frame_time) {
			self.elapsed_time = Millis(0); // reset timer
			self.current_frame += 1;
			if self.current_frame > self.num_frames {
				self.current_frame = 0;
			}
		}

		let (ox, oy) = self.offset;
		self.source_rect = sdl::sdl::Rect::new(ox + (self.current_frame * TILE_SIZE), oy * TILE_SIZE, 32, 32)
	}

	fn set_position(&mut self, coords: (i16,i16)) {
		self.coords = coords;
	}
}

impl Drawable for AnimatedSprite {
	/// Draws selfs @ coordinates provided by 
	fn draw(&self, display: &graphics::Graphics) {
		let (x,y) = self.coords;
		let dest_rect = sdl::sdl::Rect::new(x, y, 32, 32);
		display.blit_surface(*(self.sprite_sheet.get()), &self.source_rect, &dest_rect);
	}
}

impl AnimatedSprite {
	/// Loads character sprites from `assets/MyChar.bmp`
	/// `source_rect` acts as a viewport of this sprite-sheet.
	///
	/// Returns an error message if sprite-sheet could not be loaded.
	pub fn new(
		graphics: &mut graphics::Graphics, 
		sheet_path: ~str, 
		offset: (i16,i16),
		num_frames: i16, 
		fps: int
	) -> Result<AnimatedSprite, ~str> {
		// attempt to load sprite-sheet from `assets/MyChar.bmp`
		let (x,y) = offset;
		let origin = sdl::sdl::Rect::new(x * TILE_SIZE, y * TILE_SIZE, 32, 32);
		
		let sheet = graphics.load_image(sheet_path); // request graphics subsystem cache this sprite.
		let sprite = AnimatedSprite{
			offset: offset,
			coords: (0,0),
			current_frame: 0, 
			elapsed_time: Millis(0),
			num_frames: (num_frames -1), 	// our frames are drawin w/ a 0-idx'd window.
			fps: fps,
			sprite_sheet: sheet, 	// "i made this" -- we own this side of the Arc()
			source_rect: origin
		};

		return Ok(sprite);
	}
}