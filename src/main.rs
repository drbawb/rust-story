#[crate_id="rust-story#0.0.1"];

extern mod sdl2;
pub mod game;

pub fn main() {
	let story = ::game::Game{sprite: 0};
	story.start();
}
