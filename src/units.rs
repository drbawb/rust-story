use std::f64;

static TILE_SIZE: i32 	= 32;
static SCALE: f64		= 1.0;

pub trait AsGame 	{fn to_game(&self) -> Game;}
pub trait AsTile 	{fn to_tile(&self) -> Tile;}
pub trait AsPixel 	{fn to_pixel(&self) -> Pixel;}

#[deriving(Eq,Ord)]
pub struct Game(f64);

impl AsGame for Game {
	#[inline(always)]
	fn to_game(&self) -> Game { *self }
}

/// A `Game` divided by the current `TILE_SIZE` (32 | 16) expressed
/// as an unsigned integer.
impl AsTile for Game {
	#[inline(always)]	
	fn to_tile(&self) -> Tile {
		let Game(a) = *self;	
		Tile((a / TILE_SIZE as f64) as uint)
	}
}

/// A `Game` is simply a more precise `Pixel`, it must simply be rounded and returned
/// as a signed integer.
impl AsPixel for Game {
	#[inline(always)]
	fn to_pixel(&self) -> Pixel { let Game(a) = *self; Pixel(f64::round(a) as i32) }
}

// Allow `+` operator for anything which can be converted `#to_game()` 
impl<T: AsGame>  Add<T, Game> for Game {
	#[inline(always)]	
	fn add(&self, rhs: &T) -> Game {
		let (Game(a), Game(b)) = (*self, rhs.to_game());
		Game(a + b)
	}
}

// Allow `-` operator for anything which can be converted `#to_game()`
impl <T: AsGame> Sub<T, Game> for Game {
	#[inline(always)]
	fn sub(&self, rhs: &T) -> Game {
		let (Game(a), Game(b)) = (*self, rhs.to_game());
		Game(a - b)
	}
}

// Allow `*` operator for anything which can be converted `#to_game()`
impl <T: AsGame> Mul<T, Game> for Game {
	#[inline(always)]
	fn mul(&self, rhs: &T) -> Game {
		let (Game(a), Game(b)) = (*self, rhs.to_game());
		Game(a * b)
	}
}

// Allow `/` operator for anything which can be converted `#to_game()`
impl <T: AsGame> Div<T, Game> for Game {
	#[inline(always)]
	fn div(&self, rhs: &T) -> Game {
		let (Game(a), Game(b)) = (*self, rhs.to_game());
		Game(a / b)
	}
}

#[deriving(Eq,Ord)]
pub struct Pixel(i32);

/// A `Pixel` merely dereferences itself, as it is already a `Pixel`.
impl AsPixel for Pixel {
	#[inline(always)]
	fn to_pixel(&self) -> Pixel { *self }
}

// Allow `+` operator for anything which can be converted `#to_pixel()`
impl<T: AsPixel> Add<T, Pixel> for Pixel {
	#[inline(always)]
	fn add(&self, rhs: &T) -> Pixel {
		let (Pixel(a), Pixel(b)) = (*self, rhs.to_pixel());
		Pixel(a + b)
	}
}

#[deriving(Clone,Eq,Ord)]
pub struct Tile(uint);

/// A single `Tile` represents `TILE_SIZE` game units.
/// The conversion is a simple multiplication.
impl AsGame for Tile {
	#[inline(always)]	
	fn to_game(&self) -> Game {
		let Tile(a) = *self;	
		Game((a * (TILE_SIZE as uint)) as f64)
	}
}

/// A `Tile` merely dereferences itself, as it is already a `Tile`.
impl AsTile for Tile {
	#[inline(always)]
	fn to_tile(&self) -> Tile { *self }
}

/// A `Tile` must first be converted to `Game` units, which can then be expressed
/// in terms of `Pixel`'s on the screen.
impl AsPixel for Tile {
	#[inline(always)]	
	fn to_pixel(&self) -> Pixel { self.to_game().to_pixel() }
}

// Allow `+` operator for anything which can be converted `#to_tile()`
impl<T: AsTile> Add<T, Tile> for Tile {
	#[inline(always)]	
	fn add(&self, rhs: &T) -> Tile {
		let (Tile(a), Tile(b)) = (*self, rhs.to_tile());
		Tile(a + b)
	}
}

// Allow `/` operator for anything which can be converted `#to_tiel()`
impl<T: AsTile> Div<T, Tile> for Tile {
	#[inline(always)]
	fn div(&self, rhs: &T) -> Tile {
		let (Tile(a), Tile(b)) = (*self, rhs.to_tile());
		Tile(a / b)
	}
}

pub type Frame = uint;
pub type Fps = uint;

pub type Millis = int;
pub type Velocity = f64;
pub type Acceleration = f64;
