#[crate_id = "ruststory#0.1"];

extern mod sdl;
pub mod game;

pub fn main() {
	let story = ::game::Game{sprite: 0};
	story.start();
}
