In an effort to teach myself `Rust` I've decided to follow along with
the excellent youtube playlist [reconstructing cavestory](http://www.youtube.com/playlist?list=PL006xsVEsbKjSKBmLu1clo85yLrwjY67X).

This project uses [rust-sdl2](https://github.com/AngryLawyer/rust-sdl2) which binds to SDL2.

[![Build Status](https://travis-ci.org/drbawb/rust-story.png?branch=master)](https://travis-ci.org/drbawb/rust-story)

### Building w/ Cargo

This project uses Cargo to fetch dependencies and drive compilation.
Make sure you have a recent version of `rustc` and `cargo`.

To build simply run: `cargo build` from the root of the project.


The project is structured as follows:

	-- assets/ 	(assets linked to in the youtube playlist notes)
	-- src/ 	(.rs files used to build the game)

For the most part this program reads much like it's [`C++`][1] and [`C++11`][2] counterparts.

 * `Traits` are used instead of extending abstract classes.
 	* These are similar to `interfaces` in Go, Java, etc.

 * Pattern matching is leveraged where appropriate
 	* For a fun head-scratcher: look at `Player::load_sprite()`
 		* The rust compiler requires exhaustive patterns, so:
 		* For every combination of `(Movement, Looking, Facing)` `load_sprite()` must
 		  return _some sprite._
		* (Obviously the compiler cannot guarantee that we load the _correct sprite._)

  * Reference counting is used for safely sharing pointers.

---

Some areas that could be improved:

  * Needless copying and allocation, espcially w.r.t using strings for map keys.
  * Remove unnecessary boxes
  * Re-evaluate usages of trait objects

[1]: https://github.com/chebert/cavestory-screencast
[2]: https://github.com/JIghtuse/cavestory-sdl2

