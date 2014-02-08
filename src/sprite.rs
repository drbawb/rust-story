extern mod sdl;

pub mod graphics;

/// Represents a 32x32 2D character
/// This sprite will implm. a `Drawable` trait
pub struct Sprite {
	source_rect: sdl::sdl::Rect,
	sprite_sheet: sdl::video::Surface
}

impl Sprite {
	pub fn new() -> Result<~Sprite, ~str> {
		return Err(~"sprite not impl.");
	}
}