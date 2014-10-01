#![crate_id="rust-story#0.0.1"]

extern crate sdl2;
extern crate collections;

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
