use game;
use units;
use units::{AsPixel};

use sdl2::{self, mouse, rect, surface};
use sdl2::pixels::Color;
use sdl2::render::{self, Renderer, RenderDriverIndex, Texture};
use sdl2::video::{self, WindowPos};

use std::collections::hash_map::{HashMap, Entry};
use std::path::Path;

/// Acts as a buffer to the underlying display
pub struct Graphics<'g> {
	cache:  HashMap<String, Texture>,
	screen: Renderer<'g>,
}

impl<'g> Graphics<'g> {
	/// Prepare the display for rendering
	pub fn new(context: &sdl2::Sdl) -> Graphics<'g> {
		// boot the renderer
		let (units::Pixel(w), units::Pixel(h)) = 
			(game::SCREEN_WIDTH.to_pixel(), game::SCREEN_HEIGHT.to_pixel());
		
		let current_mode = video::Window::new(
			context,
			"rust-story v0.0",                               // title
			WindowPos::PosCentered, WindowPos::PosCentered,  // position (x,y)
			w, h,
			video::INPUT_GRABBED
		);

		let window_context = match current_mode {
			Ok(ctx)  => ctx,
			Err(msg) => panic!(msg),
		};

		 let renderer = Renderer::from_window(
			window_context,
			RenderDriverIndex::Auto,
			render::SOFTWARE,
		).unwrap();

		// strap it to graphics subsystem
		let graphics = Graphics {
			cache:  HashMap::new(),
			screen: renderer,
		};

		mouse::show_cursor(true);
		return graphics;
	}


	/// Caches the bitmap found at `file_path` ...
	/// The filename can then be used to fetch a handle to the loaded
	/// texture at a later occasion.
	///
	/// Panics if the resource cannot be loaded for any reason ...
	pub fn load_image(&mut self, 
	                  file_path: String, 
	                  transparent_black: bool) {
		
		// Retrieve a handle or generate a new one if it exists already.
		// Load sprite
		let sprite_path = Path::new(&file_path[..]);
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

		match self.cache.entry(file_path.clone()) {
			Entry::Vacant(entry) => {
				match self.screen.create_texture_from_surface(&sprite_surface) {
					Ok(texture) => { entry.insert(texture); },
					Err(msg) => panic!("sprite could not be rendered: {}", msg)
				}
			},

			_ => {},
		};
	}

	pub fn borrow_res(&mut self, file_path: String) -> &mut Texture {
		self.cache.get_mut(&file_path).unwrap()
	}

	pub fn draw_rect(&mut self, rect: &rect::Rect, color: Color) {
		let _ = self.screen.drawer().set_draw_color(color);
		let _ = self.screen.drawer().fill_rect(*rect);
	}

	pub fn blit_surface(&mut self,
	                    src_id: &str,
	                    src_rect:  &rect::Rect,
	                    dest_rect: &rect::Rect) {
	
		let src = &mut self.cache.get_mut(src_id).unwrap();
		let _ = self.screen.drawer().copy(src, Some(*src_rect), Some(*dest_rect));
	}

	pub fn blit_ex(&mut self,
	                    src_id: &str,
	                    src_rect:  &rect::Rect,
	                    dest_rect: &rect::Rect,
	                    flip: (bool, bool)) {
	
		let src = &mut self.cache.get_mut(src_id).unwrap();
		let _ = self.screen.drawer().copy_ex(
			src, 
			Some(*src_rect), 
			Some(*dest_rect), 
			
			0.0, 
			None, 
			flip
		);
	}

	pub fn switch_buffers(&mut self) {
		self.screen.drawer().present();
	}

	pub fn clear_buffer(&mut self) {
		let _ = self.screen.drawer().clear();
	}
}
