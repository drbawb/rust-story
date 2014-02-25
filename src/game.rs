extern crate sdl2;

use std::cmp;
use std::io::Timer;

use game::units::{AsGame};

use sdl2::sdl;
use sdl2::event;
use sdl2::keycode;

pub mod backdrop;
pub mod collisions;
pub mod graphics;
pub mod input;
pub mod map;
pub mod player;
pub mod enemies;
pub mod sprite;
pub mod units;

static TARGET_FRAMERATE: units::Fps 	= 60;
static MAX_FRAME_TIME: units::Millis 	= units::Millis(5 * (1000 / TARGET_FRAMERATE) as int);

pub static SCREEN_WIDTH: 	units::Tile 	= units::Tile(20);
pub static SCREEN_HEIGHT:	units::Tile  	= units::Tile(15);

/// An instance of the `rust-story` game with its own event loop.
pub struct Game {
	priv quote: player::Player,
	priv yatty: enemies::CaveBat,
	priv map: 	map::Map,

	priv display: 		graphics::Graphics,
	priv controller: 	input::Input 
}

/// When the `Game` leaves scope SDL is instructed to `quit`.
impl Drop for Game {
	fn drop(&mut self) {
		println!("quitting sdl ...");
		sdl::quit();
	}
}

impl Game {
	/// Starts running this games event loop, note that this will block indefinitely.
	/// This function will return to the caller when the escape key is pressed.
	pub fn new() -> Game {
		println!("initalizing sdl ...");
		
		// initialize all major subsystems
		// hide the mouse cursor in our drawing context
		sdl::init([sdl::InitEverything]);
		let mut display = graphics::Graphics::new();
		let controller =  input::Input::new();		

		Game {
			map: 	map::Map::create_test_map(&mut display),
			quote: 	player::Player::new(
					&mut display, 
					(SCREEN_WIDTH / units::Tile(2)).to_game(),
					(SCREEN_HEIGHT / units::Tile(2)).to_game()
				),
			yatty:	enemies::CaveBat::new(
					&mut display,
					(SCREEN_WIDTH / units::Tile(3)).to_game(),
					(units::Tile(10)).to_game()	
				),
			display: display,
			controller: controller
		}
	}

	pub fn start(&mut self) {
		self.event_loop();
	}


	/// Polls current input events & dispatches them to the engine.
	///
	/// Then renders a snapshot of the world-state and then waits
	/// until its next frame deadline.
	fn event_loop(&mut self) {
		// event loop control
		let frame_delay = units::Millis(1000 / TARGET_FRAMERATE as int);
		let mut last_update_time = units::Millis(sdl::get_ticks() as int);
		let mut running = true;
		let mut timer = Timer::new().unwrap();
		
		while running {
			let start_time_ms = units::Millis(sdl::get_ticks() as int);
			self.controller.begin_new_frame();

			// drain event queue once per frame
			// ideally should do in separate task
			match event::poll_event() {
				event::KeyDownEvent(_,_,key_cap,_,_) => {
					self.controller.key_down_event(key_cap);
				}
				event::KeyUpEvent(_,_,key_cap,_,_) => {
					self.controller.key_up_event(key_cap);
				}
				_ => {}
			}

			// Handle exit game
			if self.controller.was_key_released(keycode::EscapeKey) {
				running = false;
			}

			// Handle player movement
			if self.controller.is_key_held(keycode::LeftKey)
				&& self.controller.is_key_held(keycode::RightKey) {

				self.quote.stop_moving();
			} else if self.controller.is_key_held(keycode::LeftKey) {
				self.quote.start_moving_left();
			} else if self.controller.is_key_held(keycode::RightKey) {
				self.quote.start_moving_right();
			} else {
				self.quote.stop_moving();
			}

			// Handle player looking
			if self.controller.is_key_held(keycode::UpKey)
				&& self.controller.is_key_held(keycode::DownKey) {

				self.quote.look_horizontal();
			} else if self.controller.is_key_held(keycode::UpKey) {
				self.quote.look_up();
			} else if self.controller.is_key_held(keycode::DownKey) {
				self.quote.look_down();
			} else {
				self.quote.look_horizontal();
			}

			// Handle player jump
			if self.controller.was_key_pressed(keycode::ZKey) {
				self.quote.start_jump();
			} else if self.controller.was_key_released(keycode::ZKey) {
				self.quote.stop_jump();
			}

			// update
			let current_time_ms = units::Millis(sdl::get_ticks() as int);
			let elapsed_time = current_time_ms - last_update_time;
			self.update(cmp::min(elapsed_time, MAX_FRAME_TIME));
			last_update_time = current_time_ms;

			// draw
			self.display.clear_buffer(); // clear back-buffer
			self.draw();
			self.display.switch_buffers();

			// throttle event-loop
			let iter_time = units::Millis(sdl::get_ticks() as int) - start_time_ms;
			let next_frame_time: u64 = if frame_delay > iter_time {	// if we did not miss our deadline: adjust delay accordingly
				let (units::Millis(fd), units::Millis(it)) = (frame_delay, iter_time);
				(fd - it) as u64
			} else { 0 as u64 };									// otherwise missed frame-deadline, skip waiting period
			timer.sleep(next_frame_time);

			
			/* Print current FPS to stdout
			let units::Millis(start_time) = start_time_ms;
			let seconds_per_frame =  (sdl::get_ticks() as int - start_time) as f64 / 1000.0;
			let fps = 1.0 / (seconds_per_frame);

			println!("fps: {}", fps);
			*/
			
		}

	}

	/// Instructs our actors to draw their current state to the screen. 
	fn draw(&self) {
		self.map.draw_background(&self.display);
		self.map.draw_sprites(&self.display);
		self.quote.draw(&self.display);
		self.yatty.draw(&self.display);
		self.map.draw(&self.display);
	}

	/// Passes the current time in milliseconds to our underlying actors.	
	fn update(&mut self, elapsed_time: units::Millis) {
		self.map.update(elapsed_time);
		self.quote.update(elapsed_time, &self.map);
		self.yatty.update(elapsed_time);
	}
}
