extern mod sdl2;
pub mod game;

pub fn main() {
	let story = ::game::Game{sprite: 0};
	story.start();
}
