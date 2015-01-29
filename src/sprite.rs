use sdl2::rect;
use sdl2::render;

use std::rc::Rc;

use graphics;

use units;
use units::{AsGame,AsPixel};

#[derive(Hash,PartialEq,Eq)]
pub enum Motion {
	Walking,
	Standing,
	Interacting,
	Jumping,
	Falling
}
pub static MOTIONS: [Motion; 5] = [Motion::Walking,
                                   Motion::Standing,
                                   Motion::Interacting,
                                   Motion::Jumping,
                                   Motion::Falling];

#[derive(Hash,PartialEq,Eq)]
pub enum Facing {
	West,
	East
}
pub static FACINGS: [Facing; 2] = [Facing::West, Facing::East];

#[derive(Hash,PartialEq,Eq)]
pub enum Looking {
	Up,
	Down,
	Horizontal
}
pub static LOOKINGS: [Looking; 3] = [Looking::Up, Looking::Down, Looking::Horizontal];

/// Any object which can be represented in 2D space
/// Coord represents the unit which describes this object's
/// position in 3D space.
///
/// Said unit must be expressible in terms of `Game` units.
pub trait Drawable<Coord> : 'static { 
	fn draw(&self, display: &graphics::Graphics, coords: (Coord,Coord));
}

/// Any object which understands time and placement in 2D space.
pub trait Updatable<T> : Drawable<T> { 
	fn update(&mut self, elapsed_time: units::Millis);
}

/// Represents a static 32x32 2D character
pub struct Sprite {
	sprite_sheet:  Rc<render::Texture>,
	source_rect:   rect::Rect,
	size:    (units::Game, units::Game),
}

#[old_impl_check]
impl<O:AsGame, S:AsGame> Sprite {
	/// A new sprite which will draw itself at `coords`
	/// `sprite_at` is the index (row) where the sprite starts in `file_name`
	pub fn new(
		graphics: &mut graphics::Graphics, 
		offset:  (O,O),  // source_x, source_ys
		size:    (S,S),  // width, height
		file_name: String,
	) -> Sprite {
		let (w,h) = size;
		let (x,y) = offset;

		// convert from AsGame trait
		let (norm_w,norm_h) = (w.to_game(), h.to_game());
		let (norm_x,norm_y) = (x.to_game(), y.to_game());

		let (units::Pixel(wi), units::Pixel(hi)) = 
			(norm_w.to_pixel(), norm_h.to_pixel());
		let (units::Pixel(xi), units::Pixel(yi)) = 
			(norm_x.to_pixel(), norm_y.to_pixel());

		let origin  = rect::Rect::new(xi,yi,wi,hi);
		let sheet   = graphics.load_image(file_name, true);  // request graphics subsystem cache this sprite.

		return Sprite {
			sprite_sheet:  sheet,
			source_rect:   origin,
			size:          (norm_w,norm_h),
		};
	}
}

impl<C: AsGame> Drawable<C> for Sprite {
	/// Draws selfs @ coordinates provided by 
	fn draw (&self, display: &graphics::Graphics, coords: (C,C)) {
		let (w,h) = self.size;
		let (x,y) = coords;
		
		let (units::Pixel(wi), units::Pixel(hi)) = (w.to_pixel(), h.to_pixel());
		let (units::Pixel(xi), units::Pixel(yi)) = 
			(x.to_game().to_pixel(), y.to_game().to_pixel());
	
		let dest_rect = rect::Rect::new(xi, yi, wi, hi);

		display.blit_surface(&*self.sprite_sheet, &self.source_rect, &dest_rect);
	}
}

#[allow(unused_variable)]
impl<C: AsGame> Updatable<C> for Sprite {
	fn update(&mut self, elapsed_time: units::Millis) {
		// no-op for static sprite.
	}
}

/// Represents a 32x32 2D character w/ a number of frames
/// Frames will be selected based on time-deltas supplied through update
pub struct AnimatedSprite {
	pub source_rect:   rect::Rect,
	pub sprite_sheet:  Rc<render::Texture>,

	offset:  (units::Tile, units::Tile),
	size:    (units::Tile, units::Tile),

	current_frame:  units::Frame,
	num_frames:     units::Frame,
	fps:            units::Fps,

	last_update: units::Millis,
}

impl AnimatedSprite {
	/// Loads character sprites from `assets/MyChar.bmp`
	/// `source_rect` acts as a viewport of this sprite-sheet.
	///
	/// Returns an error message if sprite-sheet could not be loaded.
	pub fn new(
		graphics:    &mut graphics::Graphics,
		sheet_path:  String,

		offset:  (units::Tile, units::Tile),
		size:    (units::Tile, units::Tile),

		num_frames:  units::Frame,
		fps:         units::Fps
	) -> Result<AnimatedSprite, String> {
		// attempt to load sprite-sheet from `assets/MyChar.bmp`
		let (w,h) = size;
		let (x,y) = offset;
	
		let (units::Pixel(wi), units::Pixel(hi)) = (w.to_pixel(), h.to_pixel());
		let (units::Pixel(xi), units::Pixel(yi)) = (x.to_pixel(), y.to_pixel());
		
		let origin = rect::Rect::new(xi, yi, wi, hi);
		
		let sheet = graphics.load_image(sheet_path, true); // request graphics subsystem cache this sprite.
		let sprite = AnimatedSprite{
			offset:  offset,
			size:    size,
			
			fps: fps,
			current_frame: 0,
			num_frames:   num_frames,        // our frames are drawin w/ a 0-idx'd window.
			last_update:  units::Millis(0),
			
			sprite_sheet:  sheet,
			source_rect:   origin,
		};

		return Ok(sprite);
	}
}

impl<C: AsGame> Updatable<C> for AnimatedSprite {
	/// Reads current time-deltas and mutates state accordingly.
	fn update(&mut self, elapsed_time: units::Millis) {
		let frame_time = units::Millis(1000 / self.fps as int);
		self.last_update = self.last_update + elapsed_time;

		// if we have missed drawing a frame
		if self.last_update > frame_time {
			self.last_update = units::Millis(0);  // reset timer
			self.current_frame += 1;              // increment frame counter

			if self.current_frame < self.num_frames {
				self.source_rect.x += self.source_rect.w;
			} else {
				self.current_frame  = 0;
				self.source_rect.x -= self.source_rect.w * (self.num_frames - 1) as i32;
			}
		}
	}
}

impl<C: AsGame> Drawable<C> for AnimatedSprite {
	/// Draws selfs @ coordinates provided by `Updatable` trait
	fn draw(&self, display: &graphics::Graphics, coords: (C,C)) {
		let (w,h) = self.size;
		let (x,y) = coords;
		
		let (units::Pixel(wi), units::Pixel(hi)) = (w.to_pixel(), h.to_pixel());
		let (units::Pixel(xi), units::Pixel(yi)) = 
			(x.to_game().to_pixel(), y.to_game().to_pixel());

		let dest_rect = rect::Rect::new(xi, yi, wi, hi);
		display.blit_surface(&*self.sprite_sheet, &self.source_rect, &dest_rect);
	}
}
