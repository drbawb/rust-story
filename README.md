In an effort to teach myself `Rust` I've decided to follow along with
the excellent youtube playlist [reconstructing cavestory](http://www.youtube.com/playlist?list=PL006xsVEsbKjSKBmLu1clo85yLrwjY67X).

This project uses [rust-sdl2](https://github.com/AngryLawyer/rust-sdl2) which binds to SDL2.

To run the project:
	* install rust-master
	* compile rust-sdl2 to a `.so` file, place this in `lib/`
	* `make && make run`: will run rustc on `src/main.rs` linking against `lib/**`

The project is structured as follows:

	-- assets/ 	(assets linked to in the youtube playlist notes)
	-- bin/ 	(output executables)
	-- lib/		(stores rust-sdl2 `.so` file)
	-- src/ 	(.rs files used to build the game)

I will not be uploading the assets here, though they're freely available if you'd like to build this yourself.

While I try to follow the general architecture described in the youtube series, you will see some
notable departures where more idiomatic rust code is appropriate.

To list a few:
	* Where polymorphism is required: I use ~Traits, as rust does not provide classes nor object inheritance.
		* (Had to use ~Trait: to clear the `Sendable` trait bound.)
		* (This means sprites cannot leave the `graphics` thread as they own a texture Rc<>)
	* Where C++ smart pointers would be used: I use built-in rust allocation techniques.
	* Pattern matching is leveraged where appropriate
	* Tuple structs are used to provide type-safe abstractions of units (such as time)

