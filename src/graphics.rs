extern mod sdl;

static SCREEN_WIDTH: 	int 	 	= 1280;
static SCREEN_HEIGHT: 	int 	 	= 1024;
static BITS_PER_PIXEL: 	int 	 	= 32;

/// Acts as a buffer to the underlying display
pub struct Graphics {
	screen: ~sdl::video::Surface
}




/// Prepare the display for rendering
pub fn Graphics() -> Graphics {
	let current_mode = sdl::video::set_video_mode(
		SCREEN_WIDTH, 
		SCREEN_HEIGHT, 
		BITS_PER_PIXEL, 
		[sdl::video::HWSurface],
		[sdl::video::Fullscreen]
	);
	
	
	let graphics: Graphics;
	match current_mode {
		Ok(surface) => {graphics = Graphics{screen: surface};}
		Err(_) => {fail!("oh my")}
	}

	return graphics;
}

impl Graphics {
	pub fn blit_surface(&self, src: &sdl::video::Surface, src_rect: &sdl::sdl::Rect) {
		let dest_rect = sdl::sdl::Rect::new(320, 240, 32, 32);
		self.screen.blit_rect(src, Some(*src_rect), Some(dest_rect));
	}

	pub fn switch_buffers(&self) -> bool {
		self.screen.flip()
	}
}