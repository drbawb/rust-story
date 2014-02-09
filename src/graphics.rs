extern mod sdl;

use std::hashmap::HashMap;

static SCREEN_WIDTH: 	int 	 	= 1280;
static SCREEN_HEIGHT: 	int 	 	= 1024;
static BITS_PER_PIXEL: 	int 	 	= 32;

/// Acts as a buffer to the underlying display
pub struct Graphics {
	sprite_cache: HashMap<~str, sdl::video::Surface>,
	priv screen: ~sdl::video::Surface
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
				graphics = Graphics{screen: surface, sprite_cache: HashMap::<~str, sdl::video::Surface>::new()};
			}
			Err(_) => {fail!("oh my")}
		}

		return graphics;
	}

	// TODO: return [borrowed?] pointer which is valid as long as `graphics` is in scope
	pub fn load_image<'a>(&'a mut self, file_path: ~str) -> &'a mut sdl::video::Surface {
		self.sprite_cache.find_or_insert_with(file_path, |key| {
			let sprite_sheet = Path::new((*key).clone());
			let sprite_window = sdl::video::Surface::from_bmp(&sprite_sheet);

			*(sprite_window.unwrap()) // TODO: match & fail where
		})
	}

	pub fn blit_surface(&self, src: &sdl::video::Surface, src_rect: &sdl::sdl::Rect, dest_rect: &sdl::sdl::Rect) {
		self.screen.blit_rect(src, Some(*src_rect), Some(*dest_rect));
	}

	pub fn switch_buffers(&self) -> bool {
		self.screen.flip()
	}

	pub fn clear_buffer(&self) {
		self.screen.clear();
	}
}