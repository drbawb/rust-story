cavestory-content
=================

Content used for https://www.youtube.com/playlist?list=PL006xsVEsbKjSKBmLu1clo85yLrwjY67X

IMPORTANT:
As of Episode 17 of Reconstructing CaveStory we switch to using the media from the original freeware version.
https://www.youtube.com/watch?v=RqAPmJuSMgk&list=PL006xsVEsbKjSKBmLu1clo85yLrwjY67X&index=19

You can get the Sprites (.pbm files) from there.

---

Notes on switching between assets:

If you'd like to use the assets from the original (320x240, 16x16 tiles) game:
* Open `src/units.rs` and set `static SCALE: f64 = 1.0;` to `= 2.0`.
* (All physics are calculated based on 32x32 tiles; but they will be drawn to scale.) 

If you'd like to use assets from CaveStory+ (available on Steam, 640x480, 32x32 tiles):
* Open `src/units.rs` and set `static SCALE: f64 = 1.0;` to `= 1.0`.

You may have to fiddle w/ paths to use the original freeware assets:
* I use the `.bmp` suffix, not `.pbm` (you only need to change the extension; they are the same format).
* My sprites are mostly organized into `base/`, `base/Npc/`, and `base/Stage/`
	* (This mirrors the high-def release of the game _NOT the original version._)

Failure to load a resource will print a message about `task failure` to your `STDOUT`.

This should tell you which asset could not be found. Simply find & move the asset to the expected
path and the game should run.

(As failure is immediate: you may have to repeat this cycle several times.)

