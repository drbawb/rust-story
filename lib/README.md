You must place a compiled version of [rust-sdl2](https://github.com/AngryLawyer/rust-sdl2) in 
this directory for `rust-story` to compile.

`rustc` should recognize both static [.rlib] & dynamic [.so] libraries.

The libraries have not been included due to size concerns. I rebuild `rust@master` nightly.
This requires a rebuild of rust-sdl2 which weighs in at ~6Mib a pop for a statically linked
library.

