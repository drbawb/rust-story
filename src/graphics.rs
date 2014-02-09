extern mod sdl;
extern mod extra;


use self::extra::arc::Arc;
use std::hashmap::HashMap;

static SCREEN_WIDTH: 	int 	 	= 1280;
static SCREEN_HEIGHT: 	int 	 	= 1024;
static BITS_PER_PIXEL: 	int 	 	= 32;

/// Acts as a buffer to the underlying display
pub struct Graphics {
	priv screen: ~sdl::video::Surface,
	sprite_cache: HashMap<~str, Arc<~sdl::video::Surface>>,
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
					sprite_cache: HashMap::<~str, Arc<~sdl::video::Surface>>::new()
				};
			}
			Err(_) => {fail!("oh my")}
		}

		return graphics;
	}

	/// Loads a bitmap which resides at `file_path` and returns a handle
	/// This handle can safely be used in any of the graphics subsystem's rendering
	/// contexts.
	pub fn load_image(&mut self, file_path: ~str) -> Arc<~sdl::video::Surface> {
		// Retrieve a handle or generate a new one if it exists already.
		let sprite_handle = self.sprite_cache.find_or_insert_with(file_path, |key| {
			// Load sprite
			let sprite_path = Path::new((*key).clone());
			let sprite_window = sdl::video::Surface::from_bmp(&sprite_path);

			// Store sprite
			match sprite_window {
				Ok(sprite) => {Arc::new(sprite)},
				Err(msg) => {fail!("sprite could not be loaded: {}", msg)}
			}
		});

		sprite_handle.clone()
		
	}

	pub fn remove_image(&mut self, file_path: ~str) {
		self.sprite_cache.remove(&file_path);
	}
	

	pub fn blit_surface(&self, src: &sdl::video::Surface, src_rect: &sdl::sdl::Rect, dest_rect: &sdl::sdl::Rect) {
		//let src_surface = self.sprite_cache.get(&src.id);
		self.screen.blit_rect(src, Some(*src_rect), Some(*dest_rect));
	}

	pub fn switch_buffers(&self) -> bool {
		self.screen.flip()
	}

	pub fn clear_buffer(&self) {
		self.screen.clear();
	}
}