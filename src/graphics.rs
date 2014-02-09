extern mod sdl;

use std::hashmap::HashMap;

static SCREEN_WIDTH: 	int 	 	= 1280;
static SCREEN_HEIGHT: 	int 	 	= 1024;
static BITS_PER_PIXEL: 	int 	 	= 32;

/// Acts as a buffer to the underlying display
pub struct Graphics {
	priv screen: ~sdl::video::Surface,

	open_handles: HashMap<~str, Handle>,
	sprite_cache: HashMap<int, ~sdl::video::Surface>,
	priv next_handle: int
}

pub struct Handle {
	id: int
}

impl Graphics {
	/// Prepare the display for rendering
	pub fn new() -> Graphics {
		let current_mode = sdl::video::set_video_mode(
			SCREEN_WIDTH, 
			SCREEN_HEIGHT, 
			BITS_PER_PIXEL, 
			[sdl::video::HWSurface],
			[sdl::video::Fullscreen]
		);
		
		
		let graphics: Graphics;
		match current_mode {
			Ok(surface) => {
				graphics = Graphics{
					screen: surface, 

					open_handles: HashMap::<~str, Handle>::new(),
					sprite_cache: HashMap::<int, ~sdl::video::Surface>::new(),

					next_handle: 0
				};
			}
			Err(_) => {fail!("oh my")}
		}

		return graphics;
	}

	// TODO: return [borrowed?] pointer which is valid as long as `graphics` is in scope
	pub fn load_image(&mut self, file_path: ~str) -> Handle {
		// Retrieve a handle or generate a new one if it exists already.
		let a = self.open_handles.find_or_insert_with(file_path, |key| {
			// Assign handle
			let sprite_handle = Handle{id: self.next_handle};
			self.next_handle += 1; // incr. for next handle

			// Load sprite
			let sprite_path = Path::new((*key).clone());
			let sprite_window = sdl::video::Surface::from_bmp(&sprite_path);

			// Store sprite
			// TODO: check `Result<>`
			self.sprite_cache.insert(sprite_handle.id, sprite_window.unwrap());

			sprite_handle
		});

		*a
	}

	pub fn blit_surface(&self, src: Handle, src_rect: &sdl::sdl::Rect, dest_rect: &sdl::sdl::Rect) {
		let src_surface = self.sprite_cache.get(&src.id);
		self.screen.blit_rect(*src_surface, Some(*src_rect), Some(*dest_rect));
	}

	pub fn switch_buffers(&self) -> bool {
		self.screen.flip()
	}

	pub fn clear_buffer(&self) {
		self.screen.clear();
	}
}