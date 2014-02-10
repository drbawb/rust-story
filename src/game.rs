extern mod sdl;
use std::io::Timer;

pub mod graphics;
pub mod input;
pub mod sprite;
pub mod player;

static TARGET_FRAMERATE: int = 60;

/// An instance of the `rust-story` game with its own event loop.
pub struct Game {
	sprite: int
}

impl Drop for Game {
	/// Cleanly shuts down the SDL rendering context.
	fn drop(&mut self) {
		println!("quitting sdl ...");
		sdl::quit();
	}
}

impl Game {
	/// Starts running this games event loop, note that this will block indefinitely.
	/// This function will return to the caller when the escape key is pressed.
	pub fn start(&self) {
		println!("initalizing sdl ...");
		
		// initialize all major subsystems
		// hide the mouse cursor in our drawing context
		sdl::init([sdl::InitEverything]);
		sdl::mouse::set_cursor_visible(false);

		self.event_loop();
	}


	/// Polls current input events & dispatches them to the engine.
	///
	/// Then renders a snapshot of the world-state and then waits
	/// until its next frame deadline.
	fn event_loop(&self) {
		let mut display = graphics::Graphics::new();
		let mut input = input::Input::new();
		
		// event loop control
		let frame_delay = (1000 / TARGET_FRAMERATE) as uint;
		let mut last_update_time = sdl::sdl::get_ticks();
		let mut running = true;
		let mut timer = Timer::new().unwrap();


		// load quote's sprite
		let mut quote= player::Player::new(&mut display, 320,240);

		while running {
			let start_time_ms = sdl::sdl::get_ticks();
			input.begin_new_frame();

			// drain event queue once per frame
			// ideally should do in separate task
			match sdl::event::poll_event() {
				sdl::event::KeyEvent(keyCap,pressed,_,_) => {
					if pressed {
						input.key_down_event(keyCap);
					} else {
						input.key_up_event(keyCap);
					}
				}
				_ => {}
			}

			// Handle exit game
			if input.was_key_released(sdl::event::EscapeKey) {
				running = false;
			}

			// Handle player movement
			if input.is_key_held(sdl::event::LeftKey)
				&& input.is_key_held(sdl::event::RightKey) {

				quote.stop_moving();
			} else if input.is_key_held(sdl::event::LeftKey) {
				quote.start_moving_left();
			} else if input.is_key_held(sdl::event::RightKey) {
				quote.start_moving_right();
			} else {
				quote.stop_moving();
			}

			// Handle player jump
			if input.was_key_pressed(sdl::event::ZKey) {
				quote.start_jump();
			} else if input.was_key_released(sdl::event::ZKey) {
				quote.stop_jump();
			}

			// update
			let current_time_ms = sdl::sdl::get_ticks();
			self.update(&mut quote, current_time_ms - last_update_time);
			last_update_time = current_time_ms;


			// draw
			display.clear_buffer(); // clear back-buffer
			self.draw(&quote, &display);
			display.switch_buffers();


			// throttle event-loop
			let iter_time = sdl::sdl::get_ticks() - start_time_ms;	// time in ms that this iteration of event loop took
			let next_frame_time: u64 = if frame_delay > iter_time {	// if we did not miss our deadline: adjust delay accordingly
				(frame_delay - iter_time) as u64
			} else { 0 as u64 };									// otherwise missed frame-deadline, skip waiting period
			timer.sleep(next_frame_time);

			/*
			// Print current FPS to stdout
			let seconds_per_frame =  (sdl::sdl::get_ticks() - start_time_ms) as f64 / 1000.0;
			let fps = 1.0 / (seconds_per_frame);

			println!("fps: {}", fps);
			*/
		}

	}

	/// Draws current state of sprites to the screen
	fn draw<T: sprite::Drawable>(&self, actor: &T, display: &graphics::Graphics) {
		actor.draw(display);
	}

	/// Updates an actor's concept of time.
	/// Then instructs them to mutate their state accordingly.
	fn update<T: sprite::Updatable>(&self, actor: &mut T, elapsed_time: uint) {
		actor.update(sprite::Millis(elapsed_time));
	}
}
