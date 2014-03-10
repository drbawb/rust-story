In an effort to teach myself `Rust` I've decided to follow along with
the excellent youtube playlist [reconstructing cavestory](http://www.youtube.com/playlist?list=PL006xsVEsbKjSKBmLu1clo85yLrwjY67X).

This project uses [rust-sdl2](https://github.com/AngryLawyer/rust-sdl2) which binds to SDL2.

[![Build Status](https://travis-ci.org/drbawb/rust-story.png?branch=master)](https://travis-ci.org/drbawb/rust-story)

To run the project:
 * Install [`rust@master`](https://github.com/mozilla/rust)
 * Compile rust-sdl2 to an `.so` or `.rlib` and place the resulting library in `lib/`
 * `make && make run`: will run rustc on `src/main.rs` linking against `lib/**`

Make targets include:
 * `make deps`:		init & update the `lib/rust-sdl2` submodule; builds it; copies the dep to `lib/`
 * `make debug`: 	time the various `rustc` passes & forcibly include debug symbols for use w/ `gdb`.
 * `make veyron`: 	time the various `rustc` passes & run a more thorough LLVM optimization pass.
 * `make clean`: 	remove any artifacts generated by the previous build.
 * `make run`		execute `bin/rust-story` :-) 

The project is structured as follows:

	-- assets/ 	(assets linked to in the youtube playlist notes)
	-- bin/ 	(output executables)
	-- lib/		(stores rust-sdl2 `.so` file)
	-- src/ 	(.rs files used to build the game)


For the most part this program reads much like it's [`C++`][1] and [`C++11`][2] counterparts.

 * `Traits` are used instead of extending abstract classes.
 	* These are similar to `interfaces` in Go, Java, etc.
 	* Some implementors of the `sprite` traits hold references to memory which cannot
	  be shared between threads safely. (That is: in a way that the compiler can guarantee is safe.)
		* These implement a trait with no send bound: e.g `~Trait:`.
	* Concrete example: Sprite & AnimatedSprite implement `sprite::Updatable` & `sprite::Drawable`

 * Pattern matching is leveraged where appropriate
 	* For a fun head-scratcher: look at `Player::load_sprite()`
 		* The rust compiler requires exhaustive patterns, so:
 		* For every combination of `(Movement, Looking, Facing)` `load_sprite()` must
 		  return _some sprite._
		* (Obviously the compiler cannot guarantee that we load the _correct sprite._)

 * Most things are stack allocated; owned pointers are used for heap allocations.
 	* `rustc` makes sure that other classes may only _borrow references_ to this memory. 
 * `Arc<>` is used for reference counting shared references (which would be `"const"` in the C++ version).
 * `RWArc<>` is used for shared references that must be mutable. (For the time being: I'm using this in place of Rc<RefCell<>> as a means to 
   statically confirm the thread-safety of my code. This way I have some reasonable guarantees that I could share certain things in the event I add 
   multithreading.)

[1]: https://github.com/chebert/cavestory-screencast
[2]: https://github.com/JIghtuse/cavestory-sdl2

