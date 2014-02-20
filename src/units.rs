use std::f64;

static TILE_SIZE: i32 	= 32;
static SCALE: f64		= 1.0;

pub type Game 	= f64;	// Resolution independent units of position.
pub type Pixel 	= i32;	// An absolute position on the screen.
pub type Tile 	= uint;	
pub type Frame 	= uint;

pub type Fps = uint;	// Hz or (1 / Second)
pub type Millis = int;

pub type Velocity 		= f64; 	// Game / Millis()
pub type Acceleration 	= f64;	// Game / Millis() / Millis()

#[inline(always)]
pub fn game_to_pixel(gunit: Game) -> Pixel {
	return f64::round(gunit / SCALE) as Pixel
}

#[inline(always)]
pub fn game_to_tile(gunit: Game) -> Tile {
	(gunit / TILE_SIZE as f64) as Tile
}

#[inline(always)]
pub fn tile_to_game(tunit: Tile) -> Game {
	(tunit * (TILE_SIZE as uint)) as Game
}

#[inline(always)]
pub fn tile_to_pixel(tunit: Tile) -> Pixel {
	game_to_pixel(tile_to_game(tunit))
}
