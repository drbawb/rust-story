use game;
use units;
use units::{AsPixel};

use sdl2::{self, rect, surface};
use sdl2::pixels::Color;
use sdl2::render::{Renderer, Texture};

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
	
       
        let video            = context.video().unwrap();
        let mut window_proto = video.window("rust-story v0.0", w as u32, h as u32);
        let current_mode     = window_proto.position_centered()
                                           .input_grabbed()
                                           .build();

		let window_context = match current_mode {
			Ok(ctx)  => ctx,
			Err(msg) => panic!(msg),
		};

        let renderer = window_context.renderer()
            .software()
            .build()
            .unwrap();

		// strap it to graphics subsystem
		let graphics = Graphics {
			cache:  HashMap::new(),
			screen: renderer,
		};

        context.mouse().show_cursor(true);
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
		let sprite_window = surface::Surface::load_bmp(&sprite_path);

		// Store sprite
		let mut sprite_surface = match sprite_window {
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
					Err(msg) => panic!("sprite could not be rendered: {:?}", msg)
				}
			},

			_ => {},
		};
	}

	pub fn borrow_res(&mut self, file_path: String) -> &mut Texture {
		self.cache.get_mut(&file_path).unwrap()
	}

	pub fn blit_surface(&mut self,
	                    src_id: &str,
	                    src_rect:  &rect::Rect,
	                    dest_rect: &rect::Rect) {
	
		let src = &mut self.cache.get_mut(src_id).unwrap();
		let _ = self.screen.copy(src, Some(*src_rect), Some(*dest_rect));
	}

	pub fn switch_buffers(&mut self) {
		self.screen.present();
	}

	pub fn clear_buffer(&mut self) {
		let _ = self.screen.clear();
	}
}
