extern crate sdl2;

use std::rc::Rc;

use sdl2::render;

use game::graphics;
use game::sprite;

pub struct FixedBackdrop {
	resource: Rc<~render::Texture>
}

impl sprite::Drawable for FixedBackdrop {
	fn draw(&self, graphics: &graphics::Graphics) {
		println!("draw() FixedBackdrop");
	}
}

impl FixedBackdrop {
	pub fn new(	path: ~str, 
				graphics: &mut graphics::Graphics) 
				-> FixedBackdrop {

		let asset = graphics.load_image(path);

		FixedBackdrop { resource: asset }
	}
}