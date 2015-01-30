#![feature(box_syntax, core, hash, io, path, std_misc)]

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
	sdl::init(sdl::INIT_EVERYTHING);

	println!("initializing rendering context ...");
	let renderer = graphics::Graphics::init_renderer();

	println!("let me tell you a story ...");
	let mut story = ::game::Game::new(&renderer);
	story.start();
}
