use sdl2::rect;
use sdl2::render;

use sync::Arc;
use game::graphics;
use game::units;

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
	fn update(&mut self, elapsed_time: units::Millis); 
	fn set_position(&mut self, coords: (units::Game,units::Game));
}

/// Represents a static 32x32 2D character
pub struct Sprite {
	sprite_sheet: Arc<~render::Texture>, 
	source_rect: rect::Rect,
	coords: (units::Game,units::Game)
}

impl Drawable for Sprite {
	/// Draws selfs @ coordinates provided by 
	fn draw(&self, display: &graphics::Graphics) {
		let (x,y) = self.coords;
		let dest_rect = rect::Rect::new(
			units::game_to_pixel(x), units::game_to_pixel(y),
			32, 32
		);
		display.blit_surface(*(self.sprite_sheet.get()), &self.source_rect, &dest_rect);
	}
}

#[allow(unused_variable)]
impl Updatable for Sprite {
	fn update(&mut self, elapsed_time: units::Millis) {
		// no-op for static sprite.
	}

	fn set_position(&mut self, coords: (units::Game,units::Game)) {
		self.coords = coords;
	}
}

impl Sprite {
	/// A new sprite which will draw itself at `coords`
	/// `sprite_at` is the index (row) where the sprite starts in `file_name`
	pub fn new(
		graphics: &mut graphics::Graphics, 
		coords: (units::Game,units::Game), // position on screen
		offset: (units::Tile,units::Tile), // source_x, source_y
		file_name: ~str
	) -> Sprite {
		let (a,b) = offset;
		let origin = rect::Rect::new(units::tile_to_pixel(a), units::tile_to_pixel(b), 32, 32);

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

	priv coords: (units::Game, units::Game),
	priv offset: (units::Tile, units::Tile),
	priv current_frame: units::Frame,
	priv num_frames: units::Frame,
	priv fps: units::Fps,

	priv last_update: units::Millis
}

impl Updatable for AnimatedSprite {
	//! Reads current time-deltas and mutates state accordingly.
	fn update(&mut self, elapsed_time: units::Millis) {
		let frame_time = (1000 / self.fps);	
		self.last_update = self.last_update + elapsed_time;

		// determine next frame
		if self.last_update as uint > frame_time {
			let (ox,_) = self.offset; 

			self.last_update = 0; // reset timer
			self.current_frame += 1;
			if self.current_frame > self.num_frames + ox {
				self.current_frame = match self.offset {(ox,_) => {ox}};
			}
		}

		let (ox, oy) = self.offset;
		self.source_rect = rect::Rect::new(
			units::tile_to_pixel(ox + self.current_frame),
			units::tile_to_pixel(oy),
			32, 32
		)
	}

	fn set_position(&mut self, coords: (units::Game,units::Game)) {
		self.coords = coords;
	}
}

impl Drawable for AnimatedSprite {
	/// Draws selfs @ coordinates provided by 
	fn draw(&self, display: &graphics::Graphics) {
		let (x,y) = self.coords;
		let dest_rect = rect::Rect::new(
			units::game_to_pixel(x), units::game_to_pixel(y),
			 32, 32
		);
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
		offset: (units::Tile, units::Tile),
		num_frames: units::Frame,
		fps: units::Fps
	) -> Result<AnimatedSprite, ~str> {
		// attempt to load sprite-sheet from `assets/MyChar.bmp`
		let (x,y) = offset;
		let origin = rect::Rect::new(
			units::tile_to_pixel(x), units::tile_to_pixel(y) ,
			32, 32
		);
		
		let sheet = graphics.load_image(sheet_path, true); // request graphics subsystem cache this sprite.
		let sprite = AnimatedSprite{
			offset: offset,
			coords: (0.0, 0.0),
			current_frame: match offset {(ox, _) => {ox}}, 
			last_update: 0,
			num_frames: (num_frames -1), 	// our frames are drawin w/ a 0-idx'd window.
			fps: fps,
			sprite_sheet: sheet, 	// "i made this" -- we own this side of the Arc()
			source_rect: origin
		};

		return Ok(sprite);
	}
}
