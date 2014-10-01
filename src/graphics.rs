use std::rc::Rc;

use std::collections::HashMap;

use game;
use units;
use units::{AsPixel};

use sdl2::rect;
use sdl2::pixels;
use sdl2::surface;
use sdl2::render;
use sdl2::video;
use sdl2::mouse;

/// Acts as a buffer to the underlying display
pub struct Graphics {
	screen:            render::Renderer<video::Window>,
	pub sprite_cache:  HashMap<String, Rc<render::Texture>>,
}

impl Graphics {
	/// Prepare the display for rendering
	pub fn new() -> Graphics {
		let (units::Pixel(w), units::Pixel(h)) = 
			(game::SCREEN_WIDTH.to_pixel(), game::SCREEN_HEIGHT.to_pixel());
		
		let current_mode = video::Window::new(
			"rust-story v0.0",                       // title
			video::PosCentered, video::PosCentered,  // position (x,y)
			w as int, h as int,
			[video::InputGrabbed]
		);

		let window_context = match current_mode {
			Ok(ctx)  => ctx,
			Err(msg) => fail!(msg),
		};

		let render_context = render::Renderer::from_window(
			window_context,
			render::DriverAuto,
			render::Software,
		);

		let graphics: Graphics = match render_context {
			Ok(renderer) => {
				Graphics{
					screen:        renderer,
					sprite_cache:  HashMap::<String, Rc<render::Texture>>::new(),
				}
			},
			Err(msg) => {fail!(msg)},
		};
		
		mouse::show_cursor(false);
		return graphics;
	}

	/// Loads a bitmap which resides at `file_path` and returns a handle
	/// This handle can safely be used in any of the graphics subsystem's rendering
	/// contexts.
	pub fn load_image(&mut self, 
	                  file_path: String, 
	                  transparent_black: bool) -> Rc<render::Texture> {
		
		// Retrieve a handle or generate a new one if it exists already.
		let borrowed_display = &self.screen;
		let handle = self.sprite_cache.find_or_insert_with(file_path, |key| {
			// Load sprite
			let sprite_path = Path::new((*key).clone());
			let sprite_window = surface::Surface::from_bmp(&sprite_path);

			// Store sprite
			let sprite_surface = match sprite_window {
				Ok(surface) => surface,
				Err(msg) => fail!("sprite could not be loaded to a surface: {}", msg),
			};

			// wrap surface in texture and store it
			if transparent_black {
				match sprite_surface.set_color_key(true, pixels::RGB(0,0,0)) {
					Ok(_) => {},
					Err(msg) => fail!("Failed to key sprite: {}", msg),
				}
			}

			match borrowed_display.create_texture_from_surface(&sprite_surface) {
				Ok(texture) => Rc::new(texture),
				Err(msg) => fail!("sprite could not be rendered: {}", msg)
			}
		});

		handle.clone()
	}

	pub fn remove_image(&mut self, file_path: String) {
		self.sprite_cache.remove(&file_path);
	}
	
	pub fn blit_surface(&self,
	                    src: &render::Texture,
	                    src_rect:  &rect::Rect,
	                    dest_rect: &rect::Rect) {
		
		self.screen.copy(src, Some(*src_rect), Some(*dest_rect));
	}

	pub fn switch_buffers(&self) {
		self.screen.present();
	}

	pub fn clear_buffer(&self) {
		self.screen.clear();
	}
}
