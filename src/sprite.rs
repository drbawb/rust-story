extern mod extra;
extern mod sdl;


use self::extra::arc::Arc;
use game::graphics;

static TILE_SIZE: i16 = 32;


pub enum Motion {
	Walking,
	Standing
}

pub enum Facing {
	North,
	West,
	South,
	East
}

pub struct SpriteState(Motion, Facing);

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

	state: SpriteState,

	priv current_frame: (i16,i16),
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


		// determine next frame
		if (last_elapsed > frame_time) {
			self.elapsed_time = Millis(0); // reset frame timer
			let SpriteState(action, direction) = self.state;

			let facing = match direction {
				West => { 0 }
				East => { 1 }
				_ => {println!("dont know how to face dir"); 0}
			};

			self.current_frame = match action {
				Standing => { (0, facing) } // sprite col 1
				Walking => {
					let (action, _) = self.current_frame;
					if action + 1 > self.num_frames as i16 {
						(0, facing)
					} else {
						(action + 1, facing)
					}
				}
			};

			println!("frame: {:?}, window: {:?}", self.state, self.current_frame);

		}


	}
}

impl Updatable for Sprite {
	//! Reads current time-deltas and mutates state accordingly.
	fn update(&mut self) {
		let (x,y) = self.current_frame;
		self.source_rect = sdl::sdl::Rect::new(x * TILE_SIZE, y * TILE_SIZE, 32, 32);
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
			current_frame: (0,0), 
			elapsed_time: Millis(0),
			num_frames: (num_frames -1), 	// our frames are drawin w/ a 0-idx'd window.
			state: SpriteState(Standing, West),
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