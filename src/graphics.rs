use game;
use units;
use units::{AsPixel};

use sdl2::rect;
use sdl2::pixels::{self, Color};
use sdl2::surface;
use sdl2::render::{self, RenderDriverIndex};
use sdl2::video::{self, WindowPos};
use sdl2::mouse;

/// Acts as a buffer to the underlying display
pub struct Graphics {
	screen: render::Renderer,
}

impl Graphics {
	/// Prepare the display for rendering
	pub fn new() -> Graphics {
		let (units::Pixel(w), units::Pixel(h)) = 
			(game::SCREEN_WIDTH.to_pixel(), game::SCREEN_HEIGHT.to_pixel());
		
		let current_mode = video::Window::new(
			"rust-story v0.0",                       // title
			WindowPos::PosCentered, WindowPos::PosCentered,  // position (x,y)
			w, h,
			video::INPUT_GRABBED
		);

		let window_context = match current_mode {
			Ok(ctx)  => ctx,
			Err(msg) => panic!(msg),
		};

		let render_context = render::Renderer::from_window(
			window_context,
			RenderDriverIndex::Auto,
			render::SOFTWARE,
		);

		let graphics: Graphics = match render_context {
			Ok(renderer) => {
				Graphics{ screen: renderer, }
			},
			Err(msg) => {panic!(msg)},
		};
		
		mouse::show_cursor(false);
		return graphics;
	}

	/// Loads a bitmap which resides at `file_path` and returns a handle
	/// This handle can safely be used in any of the graphics subsystem's rendering
	/// contexts.
	pub fn load_image(&mut self, 
	                  file_path: String, 
	                  transparent_black: bool) -> render::Texture {
		
		// Retrieve a handle or generate a new one if it exists already.
		// Load sprite
		let sprite_path = Path::new(file_path);
		let sprite_window = surface::Surface::from_bmp(&sprite_path);

		// Store sprite
		let sprite_surface = match sprite_window {
			Ok(surface) => surface,
			Err(msg) => panic!("sprite could not be loaded to a surface: {}", msg),
		};

		// wrap surface in texture and store it
		if transparent_black {
			match sprite_surface.set_color_key(true, Color::RGB(0,0,0)) {
				Ok(_) => {},
				Err(msg) => panic!("Failed to key sprite: {}", msg),
			}
		}

		match self.screen.create_texture_from_surface(&sprite_surface) {
			Ok(texture) => texture,
			Err(msg) => panic!("sprite could not be rendered: {}", msg)
		}
	}

	pub fn blit_surface(&self,
	                    src: &mut render::Texture,
	                    src_rect:  &rect::Rect,
	                    dest_rect: &rect::Rect) {
		
		let _ = self.screen.copy(src, Some(*src_rect), Some(*dest_rect));
	}

	pub fn switch_buffers(&self) {
		self.screen.present();
	}

	pub fn clear_buffer(&self) {
		let _ = self.screen.clear();
	}
}
