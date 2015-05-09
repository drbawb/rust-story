use std::cmp;
use std::thread::sleep_ms;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};

use enemies;
use graphics::{self, Graphics};
use input;
use map;
use player;
use units;
use units::{AsGame};
use sprite::{self, Drawable};

use sdl2::{self, sdl};
use sdl2::event::Event;
use sdl2::keycode::KeyCode;

const  TARGET_FRAMERATE: units::Fps  =  60;
static MAX_FRAME_TIME: units::Millis =  units::Millis(5 * (1000 / TARGET_FRAMERATE as i64));

pub static SCREEN_WIDTH:  units::Tile = units::Tile(20);
pub static SCREEN_HEIGHT: units::Tile = units::Tile(15);

pub enum GameEvent {
	Panic,
}

enum GameMode {
	Running,
	GameOver,
}

/// An instance of the `rust-story` game with its own event loop.
pub struct Game<'engine> {
	quote:  player::Player,
	yatty:  enemies::CaveBat,
	map:    map::Map,

	context:     &'engine sdl2::Sdl,
	controller:  input::Input,
	display:     graphics::Graphics<'engine>,

	events_tx: Sender<GameEvent>,
	events_rx: Receiver<GameEvent>,
}

impl<'e> Game<'e> {
	/// Starts running this games event loop, note that this will block indefinitely.
	/// This function will return to the caller when the escape key is pressed.
	pub fn new(context: &'e sdl2::Sdl) -> Game<'e> {

		// initialize all major subsystems
		let controller   = input::Input::new();
		let mut display  = graphics::Graphics::new(context);

		let (gevent_tx, gevent_rx) = channel();

		Game {
			map: map::Map::create_test_map(&mut display, gevent_tx.clone()),
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

			events_tx: gevent_tx,
			events_rx: gevent_rx,
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
		let mut last_update_time = units::Millis(sdl::get_ticks() as i64);
	
		let mut event_pump   = self.context.event_pump();
		let mut game_state   = GameMode::Running;
		let mut running      = true;
		
		while running {
			let start_time_ms = units::Millis(sdl::get_ticks() as i64);
			self.controller.begin_new_frame();

			// drain sdl event queue once per frame
			// ideally should do in separate task
			for event in event_pump.poll_iter() {
				match event {
					Event::KeyDown { keycode, .. } => {
						self.controller.key_down_event(keycode);
					},
					Event::KeyUp { keycode, .. } => {
						self.controller.key_up_event(keycode);
					},
					_ => {},
				}
			}

			// drain our event queue
			'drain: loop {
				match self.events_rx.try_recv() {
					Ok(GameEvent::Panic) => { game_state = GameMode::GameOver },
					_ => {},
				}

				break 'drain;
			}

			// Handle exit game
			if self.controller.was_key_released(KeyCode::Escape) {
				running = false;
			}
			
			self.display.clear_buffer(); // clear back-buffer
			match game_state {
				GameMode::Running => {
					// handle player input
					self.tick_player();

					// inform actors of how much time has passed since last frame
					let current_time_ms = units::Millis(sdl::get_ticks() as i64);
					let elapsed_time    = current_time_ms - last_update_time;
					
					self.update(cmp::min(elapsed_time, MAX_FRAME_TIME));
					last_update_time = current_time_ms;

					// draw
					self.draw();
				},

				GameMode::GameOver => {
					self.tick_menu(&mut game_state);

					let sprite_oxy  = (units::Tile(0), units::Tile(0));
					let sprite_owh  = (units::Tile(15), units::Tile(20));
					let sprite_path = "assets/chase/game-over.bmp".to_string();
					let mut sprite  = sprite::Sprite::new(&mut self.display, 
					                                      sprite_oxy, 
					                                      sprite_owh, 
					                                      sprite_path);

					// draw
					sprite.draw(&mut self.display, sprite_oxy);
				},
			}
			self.display.switch_buffers();

			// throttle event-loop based on iteration time vs frame deadline
			let iter_time = units::Millis(sdl::get_ticks() as i64) - start_time_ms;
			let next_frame_time: u64 = if frame_delay > iter_time { 
				let (units::Millis(fd), units::Millis(it)) = (frame_delay, iter_time);
				(fd - it) as u64
			} else { 0 as u64 };

			sleep_ms(next_frame_time as u32);
			
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


	fn tick_menu(&mut self, next_state: &mut GameMode) {
		if self.controller.was_key_pressed(KeyCode::R) {
			// reload level
			self.map = map::Map::create_test_map(&mut self.display, self.events_tx.clone());
			
			self.quote = player::Player::new(
				&mut self.display,
				(SCREEN_WIDTH  / units::Tile(2)).to_game(),
				(SCREEN_HEIGHT / units::Tile(2)).to_game(),
			);

			self.yatty = enemies::CaveBat::new(&mut self.display,
				(SCREEN_WIDTH / units::Tile(3)).to_game(),
				(units::Tile(10)).to_game(),
			);

			*next_state = GameMode::Running;
		}
	}

	fn tick_player(&mut self) {
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
	}
}
