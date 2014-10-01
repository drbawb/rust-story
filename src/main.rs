extern crate sdl2;

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
	let mut story = ::game::Game::new();
	story.start();
}
