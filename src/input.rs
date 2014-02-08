extern mod sdl;

use std::hashmap::HashMap;

pub struct Input {
	priv pressed_keys: 	HashMap<u32, bool>,
	priv released_keys: HashMap<u32, bool>,
	priv held_keys: 	HashMap<u32, bool>

}

impl Input {
	pub fn new() -> Input {
		Input{
			pressed_keys: 	HashMap::<u32, bool>::new(),
			released_keys: 	HashMap::<u32, bool>::new(),
			held_keys: 		HashMap::<u32, bool>::new()
		}
	}

	pub fn beginNewFrame(&mut self) {
		self.pressed_keys.clear();
		self.released_keys.clear();
	}

	pub fn keyDownEvent(&mut self, key: sdl::event::Key) {
		self.pressed_keys.insert(key as u32, true);
		self.held_keys.insert(key as u32, true);
	}

	pub fn keyUpEvent(&mut self, key: sdl::event::Key) {
		self.released_keys.insert(key as u32, true);
		self.held_keys.insert(key as u32, false);
	}

	pub fn wasKeyPressed(&self, key: sdl::event::Key) -> bool {
		let key_cap = &(key as u32);
		match self.pressed_keys.find_copy(key_cap) {
			Some(is_pressed) => {is_pressed}
			None => false
		}
	}
	
	pub fn wasKeyReleased(&self, key: sdl::event::Key) -> bool {
		let key_cap = &(key as u32);
		match self.released_keys.find_copy(key_cap) {
			Some(is_pressed) => {is_pressed}
			None => false
		}
	}
	
	pub fn isKeyHeld(&self, key: sdl::event::Key) -> bool {
		let key_cap = &(key as u32);
		match self.held_keys.find_copy(key_cap) {
			Some(is_pressed) => {is_pressed}
			None => false
		}
	}
}