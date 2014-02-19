use std::f64;

pub static TILE_SIZE: i32 = 32;

pub type Game 	= f64;	// Resolution independent units of position.
pub type Pixel 	= int;	// An absolute position on the screen.
pub type Tile 	= uint;	

pub type Fps = uint;	// Hz or (1 / Second)

pub type Velocity 		= f64; 	// Game / Millis()
pub type Acceleration 	= f64;	// Game / Millis() / Millis()

#[inline]
pub fn game_to_pixel(gunit: Game) -> Pixel {
	// TODO: assuming 32px
	return f64::round(gunit) as Pixel
}

#[inline]
pub fn game_to_tile(gunit: Game) -> Tile {
	(gunit / TILE_SIZE as f64) as Tile
}

#[inline]
pub fn tile_to_game(tunit: Tile) -> Game {
	(tunit * (TILE_SIZE as uint)) as Game
}

#[inline]
pub fn tile_to_pixel(tunit: Tile) -> Pixel {
	game_to_pixel(tile_to_game(tunit))
}

/// Milliseconds expressed as a large positive integer
#[deriving(Ord,Eq)]
pub struct Millis(uint);

impl Add<Millis,Millis> for Millis {
	/// The `uint`s inside LHS & RHS will be added together and wrapped 
	/// inside a new `Millis()`
	fn add(&self, rhs: &Millis) -> Millis {
		let Millis(a) = *self;
		let Millis(b) = *rhs;

		Millis(a+b)
	}	
}

impl Mul<f64, f64> for Millis {
	/// The `uint` inside LHS will be cast to f64.
	/// Multiplication will then proceed as normal, returning
	/// an f64 as a result.
	fn mul(&self, rhs: &f64) -> f64 {
		let Millis(a) = *self;

		(*rhs) * (a as f64)
	}
}
