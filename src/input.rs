extern mod sdl;

use std::hashmap::HashMap;

/// Responds to inquiries regarding three sets of keyboard input.
///
///- Pressed keys
///- Released keys
///- Held keys
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

	/// Resets the toggle states of pressed & released keys.
	pub fn beginNewFrame(&mut self) {
		self.pressed_keys.clear();
		self.released_keys.clear();
	}

	/// Handles a key down event
	pub fn keyDownEvent(&mut self, key: sdl::event::Key) {
		self.pressed_keys.insert(key as u32, true);
		self.held_keys.insert(key as u32, true);
	}

	/// Handles a key up event
	pub fn keyUpEvent(&mut self, key: sdl::event::Key) {
		self.released_keys.insert(key as u32, true);
		self.held_keys.insert(key as u32, false);
	}

	/// Responds true if key was pressed since last call to `beginNewFrame()`.
	/// Responds false otherwise.
	pub fn wasKeyPressed(&self, key: sdl::event::Key) -> bool {
		let key_cap = &(key as u32);
		match self.pressed_keys.find_copy(key_cap) {
			Some(is_pressed) => {is_pressed}
			None => false
		}
	}
	
	/// Responds true if key was released since last call to `beginNewFrame()`.
	/// Responds false otherwise.
	pub fn wasKeyReleased(&self, key: sdl::event::Key) -> bool {
		let key_cap = &(key as u32);
		match self.released_keys.find_copy(key_cap) {
			Some(is_pressed) => {is_pressed}
			None => false
		}
	}
	
	/// Responds true if key has been pressed since last call to `beginNewFrame()`
	/// but _has not yet been released._
	///
	/// Responds false otherwise.
	pub fn isKeyHeld(&self, key: sdl::event::Key) -> bool {
		let key_cap = &(key as u32);
		match self.held_keys.find_copy(key_cap) {
			Some(is_pressed) => {is_pressed}
			None => false
		}
	}
}