use std::cmp;
use std::thread::sleep;
use std::time::Duration;

use enemies;
use graphics;
use input;
use map;
use player;
use units;
use units::{AsGame};

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const TARGET_FRAMERATE: units::Fps  =  60;
static MAX_FRAME_TIME: units::Millis =  units::Millis(5 * (1000 / TARGET_FRAMERATE as i64));

pub static SCREEN_WIDTH:  units::Tile = units::Tile(20);
pub static SCREEN_HEIGHT: units::Tile = units::Tile(15);

/// An instance of the `rust-story` game with its own event loop.
pub struct Game<'engine> {
	quote:  player::Player,
	yatty:  enemies::CaveBat,
	map:    map::Map,

	context:     &'engine sdl2::Sdl,
	controller:  input::Input,
	display:     graphics::Graphics<'engine>,
}

impl<'e> Game<'e> {
	/// Starts running this games event loop, note that this will block indefinitely.
	/// This function will return to the caller when the escape key is pressed.
	pub fn new(context: &'e sdl2::Sdl) -> Game<'e> {

		// initialize all major subsystems
		let controller   = input::Input::new();
		let mut display  = graphics::Graphics::new(context);

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
			controller:  controller,
			context:     context,
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
		let frame_delay          = units::Millis(1000 / TARGET_FRAMERATE as i64);
		let mut last_update_time = units::Millis(self.get_ticks() as i64);
	
		let mut event_pump = self.context.event_pump().unwrap();
		let mut running    = true;
		
		while running {
			let start_time_ms = units::Millis(self.get_ticks() as i64);
			self.controller.begin_new_frame();

			// drain event queue once per frame
			// ideally should do in separate task

			for event in event_pump.poll_iter() {
				match event {
					Event::KeyDown { keycode, .. } => {
						self.controller.key_down_event(keycode.unwrap());
					},
					Event::KeyUp { keycode, .. } => {
						self.controller.key_up_event(keycode.unwrap());
					},
					_ => {},
				}
			}

			// Handle exit game
			if self.controller.was_key_released(Keycode::Escape) {
				running = false;
			}

			// Handle player movement
			if self.controller.is_key_held(Keycode::Left)
				&& self.controller.is_key_held(Keycode::Right) {

				self.quote.stop_moving();
			} else if self.controller.is_key_held(Keycode::Left) {
				self.quote.start_moving_left();
			} else if self.controller.is_key_held(Keycode::Right) {
				self.quote.start_moving_right();
			} else {
				self.quote.stop_moving();
			}

			// Handle player looking
			if self.controller.is_key_held(Keycode::Up)
				&& self.controller.is_key_held(Keycode::Down) {

				self.quote.look_horizontal();
			} else if self.controller.is_key_held(Keycode::Up) {
				self.quote.look_up();
			} else if self.controller.is_key_held(Keycode::Down) {
				self.quote.look_down();
			} else {
				self.quote.look_horizontal();
			}

			// Handle player jump
			if self.controller.was_key_pressed(Keycode::Z) {
				self.quote.start_jump();
			} else if self.controller.was_key_released(Keycode::Z) {
				self.quote.stop_jump();
			}

			// inform actors of how much time has passed since last frame
			let current_time_ms = units::Millis(self.get_ticks() as i64);
			let elapsed_time    = current_time_ms - last_update_time;
			
			self.update(cmp::min(elapsed_time, MAX_FRAME_TIME));
			last_update_time = current_time_ms;

			// draw
			self.display.clear_buffer(); // clear back-buffer
			self.draw();
			self.display.switch_buffers();

			// throttle event-loop based on iteration time vs frame deadline
			let iter_time = units::Millis(self.get_ticks() as i64) - start_time_ms;
			let next_frame_time: u64 = if frame_delay > iter_time { 
				let (units::Millis(fd), units::Millis(it)) = (frame_delay, iter_time);
				(fd - it) as u64
			} else { 0 as u64 };

			sleep(Duration::from_millis(next_frame_time));
			
			/* Print current FPS to stdout
			let units::Millis(start_time) = start_time_ms;
			let seconds_per_frame =  (sdl::get_ticks() as int - start_time) as f64 / 1000.0;
			let fps = 1.0 / (seconds_per_frame);

			println!("fps: {}", fps);
			*/
			
		}

	}

    // TODO: use time::* etc?
    fn get_ticks(&mut self) -> u32 {
        self.context.timer().unwrap().ticks()
    }

	/// Instructs our actors to draw their current state to the screen.
	fn draw(&mut self) {
		// background
		self.map.draw_background(&mut self.display);
		self.map.draw_sprites(&mut self.display);

		// foreground
		self.quote.draw(&mut self.display);
		self.yatty.draw(&mut self.display);
		self.map.draw(&mut self.display);

		// ui
		self.quote.draw_hud(&mut self.display);
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
