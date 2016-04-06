extern crate num;
extern crate sdl2;

pub mod backdrop;
pub mod collisions;
pub mod enemies;
pub mod game;
pub mod graphics;
pub mod input;
pub mod map;
pub mod number_sprite;
pub mod player;
pub mod sprite;
pub mod units;

pub fn main() {
	println!("initalizing sdl ...");
	let sdl_context = sdl2::init().unwrap();

	println!("let me tell you a story ...");
	let mut story = ::game::Game::new(&sdl_context);
	story.start();
}
