#![feature(box_syntax, std_misc, thread_sleep)]

extern crate sdl2;

use sdl2::sdl;

pub mod backdrop;
pub mod collisions;
pub mod enemies;
pub mod game;
pub mod graphics;
pub mod input;
pub mod map;
pub mod player;
pub mod sprite;
pub mod units;

pub fn main() {
	println!("initalizing sdl ...");
	let sdl_context = sdl::init(sdl::INIT_EVERYTHING).unwrap();

	println!("initializing rendering context ...");
	let renderer = graphics::Graphics::init_renderer();

	println!("let me tell you a story ...");
	let mut story = ::game::Game::new(&renderer, &sdl_context);
	story.start();
}
