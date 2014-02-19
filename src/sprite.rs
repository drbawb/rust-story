use sdl2::rect;
use sdl2::render;

use sync::Arc;
use game::graphics;
use game::units::Millis;
use game::units::TILE_SIZE;

#[deriving(IterBytes,Eq)]
pub enum Motion {
	Walking,
	Standing,
	Interacting,
	Jumping,
	Falling
}
pub static MOTIONS: [Motion, ..5] = [Walking, Standing, Interacting, Jumping, Falling];


#[deriving(IterBytes,Eq)]
pub enum Facing {
	West,
	East
}
pub static FACINGS: [Facing, ..2] = [West, East];

#[deriving(IterBytes,Eq)]
pub enum Looking {
	Up,
	Down,
	Horizontal
}
pub static LOOKINGS: [Looking, ..3] = [Up, Down, Horizontal];

/// Any object which can be represented in 2D space
pub trait Drawable { 
	fn draw(&self, display: &graphics::Graphics); 
}

/// Any object which understands time and placement in 2D space.
pub trait Updatable : Drawable { 
	fn update(&mut self, elapsed_time: Millis); 
	fn set_position(&mut self, coords: (i32,i32));
}

/// Represents a static 32x32 2D character
pub struct Sprite {
	sprite_sheet: Arc<~render::Texture>, 
	source_rect: rect::Rect,
	coords: (i32,i32)
}

impl Drawable for Sprite {
	/// Draws selfs @ coordinates provided by 
	fn draw(&self, display: &graphics::Graphics) {
		let (x,y) = self.coords;
		let dest_rect = rect::Rect::new(x, y, 32, 32);
		display.blit_surface(*(self.sprite_sheet.get()), &self.source_rect, &dest_rect);
	}
}

#[allow(unused_variable)]
impl Updatable for Sprite {
	fn update(&mut self, elapsed_time: Millis) {
		// no-op for static sprite.
	}

	fn set_position(&mut self, coords: (i32,i32)) {
		self.coords = coords;
	}
}

impl Sprite {
	/// A new sprite which will draw itself at `coords`
	/// `sprite_at` is the index (row) where the sprite starts in `file_name`
	pub fn new(
		graphics: &mut graphics::Graphics, 
		coords: (i32,i32), 
		offset: (i32,i32), 
		file_name: ~str
	) -> Sprite {
		let (a,b) = offset;
		let origin = rect::Rect::new(a * TILE_SIZE, b * TILE_SIZE, 32, 32);

		let sheet = graphics.load_image(file_name, true); // request graphics subsystem cache this sprite.

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
	source_rect: rect::Rect,
	sprite_sheet: Arc<~render::Texture>, 

	priv coords: (i32, i32),
	priv offset: (i32,i32),
	priv current_frame: i32,
	priv num_frames: i32,
	priv fps: i32,

	priv last_update: Millis
}

impl Updatable for AnimatedSprite {
	//! Reads current time-deltas and mutates state accordingly.
	fn update(&mut self, elapsed_time: Millis) {
		let frame_time = Millis((1000 /self.fps) as uint);
		
		// unpack milliseconds to do integer math
		// then store the result
		// let Millis(world_elapsed) = elapsed_time;
		// let Millis(mut last_elapsed) = self.elapsed_time;
		// last_elapsed += world_elapsed;

		self.last_update = self.last_update + elapsed_time;

		// determine next frame
		if self.last_update > frame_time {
			let (ox,_) = self.offset;

			self.last_update = Millis(0); // reset timer
			self.current_frame += 1;
			if self.current_frame > self.num_frames + ox {
				self.current_frame = match self.offset {(ox,_) => {ox}};
			}
		}

		let (ox, oy) = self.offset;
		self.source_rect = rect::Rect::new(ox + (self.current_frame * TILE_SIZE), oy * TILE_SIZE, 32, 32)
	}

	fn set_position(&mut self, coords: (i32,i32)) {
		self.coords = coords;
	}
}

impl Drawable for AnimatedSprite {
	/// Draws selfs @ coordinates provided by 
	fn draw(&self, display: &graphics::Graphics) {
		let (x,y) = self.coords;
		let dest_rect = rect::Rect::new(x, y, 32, 32);
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
		offset: (i32,i32),
		num_frames: i32, 
		fps: i32
	) -> Result<AnimatedSprite, ~str> {
		// attempt to load sprite-sheet from `assets/MyChar.bmp`
		let (x,y) = offset;
		let origin = rect::Rect::new(x * TILE_SIZE, y * TILE_SIZE, 32, 32);
		
		let sheet = graphics.load_image(sheet_path, true); // request graphics subsystem cache this sprite.
		let sprite = AnimatedSprite{
			offset: offset,
			coords: (0,0),
			current_frame: match offset {(ox, _) => {ox}}, 
			last_update: Millis(0),
			num_frames: (num_frames -1), 	// our frames are drawin w/ a 0-idx'd window.
			fps: fps,
			sprite_sheet: sheet, 	// "i made this" -- we own this side of the Arc()
			source_rect: origin
		};

		return Ok(sprite);
	}
}
