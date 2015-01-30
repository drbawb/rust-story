use std::cmp;
use std::old_io::Timer;
use std::time::Duration;

use enemies;
use graphics;
use input;
use map;
use player;
use units;
use units::{AsGame};

use sdl2::sdl;
use sdl2::event::{self, Event};
use sdl2::keycode::{self, KeyCode};

const TARGET_FRAMERATE: units::Fps  =  60;
static MAX_FRAME_TIME: units::Millis =  units::Millis(5 * (1000 / TARGET_FRAMERATE as i64));

pub static SCREEN_WIDTH:  units::Tile = units::Tile(20);
pub static SCREEN_HEIGHT: units::Tile = units::Tile(15);

/// An instance of the `rust-story` game with its own event loop.
pub struct Game {
	quote:  player::Player,
	yatty:  enemies::CaveBat,
	map:    map::Map,

	display:     graphics::Graphics,
	controller:  input::Input,
}

impl Game {
	/// Starts running this games event loop, note that this will block indefinitely.
	/// This function will return to the caller when the escape key is pressed.
	pub fn new() -> Game {
		println!("initalizing sdl ...");
		
		// initialize all major subsystems
		// hide the mouse cursor in our drawing context
		sdl::init(sdl::INIT_EVERYTHING);
		let mut display = graphics::Graphics::new();
		let controller  = input::Input::new();

		Game {
			map: map::Map::create_test_map(&mut display),
			quote: player::Player::new(
				&mut display,
				(SCREEN_WIDTH  / units::Tile(2)).to_game(),
				(SCREEN_HEIGHT / units::Tile(2)).to_game(),
			),

			yatty: enemies::CaveBat::new( &mut display,
				(SCREEN_WIDTH / units::Tile(3)).to_game(),
				(units::Tile(10)).to_game(),
			),

			display:     display,
			controller:  controller
		}
	}

	pub fn start(&mut self) {
		self.event_loop();
		sdl::quit();
	}

	/// Polls current input events & dispatches them to the engine.
	///
	/// Then renders a snapshot of the world-state and then waits
	/// until its next frame deadline.
	fn event_loop(&mut self) {
		// event loop control
		let frame_delay          = units::Millis(1000 / TARGET_FRAMERATE as i64);
		let mut last_update_time = units::Millis(sdl::get_ticks() as i64);
		
		let mut running = true;
		let mut timer   = Timer::new().unwrap();
		
		while running {
			let start_time_ms = units::Millis(sdl::get_ticks() as i64);
			self.controller.begin_new_frame();

			// drain event queue once per frame
			// ideally should do in separate task
			match event::poll_event() {
				Event::KeyDown(_,_,key_cap,_,_,_) => {
					self.controller.key_down_event(key_cap);
				},
				Event::KeyUp(_,_,key_cap,_,_,_) => {
					self.controller.key_up_event(key_cap);
				},
				_ => {},
			}

			// Handle exit game
			if self.controller.was_key_released(KeyCode::Escape) {
				running = false;
			}

			// Handle player movement
			if self.controller.is_key_held(KeyCode::Left)
				&& self.controller.is_key_held(KeyCode::Right) {

				self.quote.stop_moving();
			} else if self.controller.is_key_held(KeyCode::Left) {
				self.quote.start_moving_left();
			} else if self.controller.is_key_held(KeyCode::Right) {
				self.quote.start_moving_right();
			} else {
				self.quote.stop_moving();
			}

			// Handle player looking
			if self.controller.is_key_held(KeyCode::Up)
				&& self.controller.is_key_held(KeyCode::Down) {

				self.quote.look_horizontal();
			} else if self.controller.is_key_held(KeyCode::Up) {
				self.quote.look_up();
			} else if self.controller.is_key_held(KeyCode::Down) {
				self.quote.look_down();
			} else {
				self.quote.look_horizontal();
			}

			// Handle player jump
			if self.controller.was_key_pressed(KeyCode::Z) {
				self.quote.start_jump();
			} else if self.controller.was_key_released(KeyCode::Z) {
				self.quote.stop_jump();
			}

			// inform actors of how much time has passed since last frame
			let current_time_ms = units::Millis(sdl::get_ticks() as i64);
			let elapsed_time    = current_time_ms - last_update_time;
			
			self.update(cmp::min(elapsed_time, MAX_FRAME_TIME));
			last_update_time = current_time_ms;

			// draw
			self.display.clear_buffer(); // clear back-buffer
			self.draw();
			self.display.switch_buffers();

			// throttle event-loop based on iteration time vs frame deadline
			let iter_time = units::Millis(sdl::get_ticks() as i64) - start_time_ms;
			let next_frame_time: u64 = if frame_delay > iter_time { 
				let (units::Millis(fd), units::Millis(it)) = (frame_delay, iter_time);
				(fd - it) as u64
			} else { 0 as u64 };
			timer.sleep(Duration::milliseconds(next_frame_time as i64));

			
			/* Print current FPS to stdout
			let units::Millis(start_time) = start_time_ms;
			let seconds_per_frame =  (sdl::get_ticks() as int - start_time) as f64 / 1000.0;
			let fps = 1.0 / (seconds_per_frame);

			println!("fps: {}", fps);
			*/
			
		}

	}

	/// Instructs our actors to draw their current state to the screen.
	fn draw(&mut self) {
		// background
		self.map.draw_background(&self.display);
		self.map.draw_sprites(&self.display);

		// foreground
		self.quote.draw(&self.display);
		self.yatty.draw(&self.display);
		self.map.draw(&self.display);

		// ui
		self.quote.draw_hud(&self.display);
	}

	/// Passes the current time in milliseconds to our underlying actors.
	fn update(&mut self, elapsed_time: units::Millis) {
		self.map.update(elapsed_time);
		self.quote.update(elapsed_time, &self.map);
		self.yatty.update(elapsed_time, self.quote.center_x());

		let collided =
			self.yatty.damage_rectangle()
			    .collides_with(&self.quote.damage_rectangle());

		if collided {
			self.quote.take_damage();
		}
	}
}
