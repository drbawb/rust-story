use std::num::Float;
use std::ops::{Add, Sub, Mul, Div};

static TILE_SIZE: i32          =  32;
static SCALE: f64              = 1.0;

pub trait AsGame  { fn to_game(&self)   -> Game;  }
pub trait AsTile  { fn to_tile(&self)  -> Tile;  }
pub trait AsPixel { fn to_pixel(&self) -> Pixel; }

/// A `Game` unit represents a density-independent distance in pixels.
/// Converting a `Game` to pixels will round it to the nearest coordinate,
/// scaled based on the desired tile size & resolution.
#[derive(Clone, Copy, PartialEq,PartialOrd)]
pub struct Game(pub f64);

impl AsGame for Game {
	#[inline]
	fn to_game(&self) -> Game { *self }
}

impl AsTile for Game {
	#[inline]
	fn to_tile(&self) -> Tile {
		let Game(a) = *self;
		Tile((a / TILE_SIZE as f64) as usize)
	}
}

impl AsPixel for Game {
	#[inline]
	fn to_pixel(&self) -> Pixel { let Game(a) = *self; Pixel((a / SCALE).round() as i32) }
}

// Allow `+` operator for anything which can be converted `#to_game()` 
impl<T: AsGame>  Add<T> for Game {
	type Output = Game;

	#[inline]
	fn add(self, rhs: T) -> Game {
		let (Game(a), Game(b)) = (self, rhs.to_game());
		Game(a + b)
	}
}

// Allow `-` operator for anything which can be converted `#to_game()`
impl <T: AsGame> Sub<T> for Game {
	type Output = Game;
	
	#[inline]
	fn sub(self, rhs: T) -> Game {
		let (Game(a), Game(b)) = (self, rhs.to_game());
		Game(a - b)
	}
}

// Allow `*` operator for anything which can be converted `#to_game()`
impl <T: AsGame> Mul<T> for Game {
	type Output = Game;

	#[inline]
	fn mul(self, rhs: T) -> Game {
		let (Game(a), Game(b)) = (self, rhs.to_game());
		Game(a * b)
	}
}

// Allow `/` operator for anything which can be converted `#to_game()`
impl <T: AsGame> Div<T> for Game {
	type Output = Game;

	#[inline]
	fn div(self, rhs: T) -> Game {
		let (Game(a), Game(b)) = (self, rhs.to_game());
		Game(a / b)
	}
}

/// A `Pixel` represents an absolute coordinate on a surface.
#[derive(Clone, Copy, PartialEq,Eq,PartialOrd,Ord)]
pub struct Pixel(pub i32);

impl AsPixel for Pixel {
	#[inline]
	fn to_pixel(&self) -> Pixel { *self }
}

// Allow `+` operator for anything which can be converted `#to_pixel()`
impl<T: AsPixel> Add<T> for Pixel {
	type Output = Pixel;

	#[inline]
	fn add(self, rhs: T) -> Pixel {
		let (Pixel(a), Pixel(b)) = (self, rhs.to_pixel());
		Pixel(a + b)
	}
}

/// A `HalfTile` represents half of the game's base tile size.
/// 
/// (This will ultimately be 16 Games, or some scaled number of
/// pixels.)
#[derive(Clone, Copy, PartialEq,Eq,PartialOrd,Ord)]
pub struct HalfTile(pub u64);

impl AsGame for HalfTile {
	#[inline]
	fn to_game(&self) -> Game {
		let HalfTile(a) = *self;
		Game((a * (TILE_SIZE as u64 / 2)) as f64)
	}
}

/// A `Tile` represents a single square drawn in the game's
/// _base tile-size_ (32 pixels).
///
/// This may ultimately be scaled if converted to `Games` or `Pixels`
#[derive(Clone, Copy, PartialEq,Eq,PartialOrd,Ord)]
pub struct Tile(pub usize);

impl AsGame for Tile {
	#[inline]
	fn to_game(&self) -> Game {
		let Tile(a) = *self;
		Game((a * (TILE_SIZE as usize)) as f64)
	}
}

impl AsTile for Tile {
	#[inline]
	fn to_tile(&self) -> Tile { *self }
}

impl AsPixel for Tile {
	#[inline]
	fn to_pixel(&self) -> Pixel { self.to_game().to_pixel() }
}

// Allow `+` operator for anything which can be converted `#to_tile()`
impl<T: AsTile> Add<T> for Tile {
	type Output = Tile;

	#[inline]
	fn add(self, rhs: T) -> Tile {
		let (Tile(a), Tile(b)) = (self, rhs.to_tile());
		Tile(a + b)
	}
}

// Allow `-` operator for anything which can be converted to `#to_tile()`
impl<T: AsTile> Sub<T> for Tile {
	type Output = Tile;

	#[inline]
	fn sub(self, rhs: T) -> Tile {
		let (Tile(a), Tile(b)) = (self, rhs.to_tile());
		Tile(a - b)
	}
}

// Allow `*` operator for anything which can be converted `#to_tile()`
impl<T: AsTile> Mul<T> for Tile {
	type Output = Tile;

	#[inline]
	fn mul(self, rhs: T) -> Tile {
		let (Tile(a), Tile(b)) = (self, rhs.to_tile());
		Tile(a * b)
	}
}

// Allow `/` operator for anything which can be converted `#to_tile()`
impl<T: AsTile> Div<T> for Tile {
	type Output = Tile;

	#[inline]
	fn div(self, rhs: T) -> Tile {
		let (Tile(a), Tile(b)) = (self, rhs.to_tile());
		Tile(a / b)
	}
}
