use std::f64;

static TILE_SIZE: i32 	= 32;
static SCALE: f64	= 1.0; // This is a divider of TILE_SIZE, 1.0 for 32, 2.0 for 16, etc.

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
	fn to_pixel(&self) -> Pixel { let Game(a) = *self; Pixel(f64::round(a / SCALE) as i32) }
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

#[deriving(Eq,Ord)]
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

// Allow `-` operator for anything which can be converted to `#to_tile()`
impl<T: AsTile> Sub<T, Tile> for Tile {
	#[inline(always)]	
	fn sub(&self, rhs: &T) -> Tile {
		let (Tile(a), Tile(b)) = (*self, rhs.to_tile());
		Tile(a - b)
	}
}

// Allow `*` operator for anything which can be converted `#to_tile()`
impl<T: AsTile> Mul<T, Tile> for Tile {
	#[inline(always)]	
	fn mul(&self, rhs: &T) -> Tile {
		let (Tile(a), Tile(b)) = (*self, rhs.to_tile());
		Tile(a * b)
	}
}

// Allow `/` operator for anything which can be converted `#to_tile()`
impl<T: AsTile> Div<T, Tile> for Tile {
	#[inline(always)]
	fn div(&self, rhs: &T) -> Tile {
		let (Tile(a), Tile(b)) = (*self, rhs.to_tile());
		Tile(a / b)
	}
}

#[deriving(Eq,Ord)]
pub struct Millis(int);

impl Add<Millis,Millis> for Millis {
	#[inline(always)]
	fn add(&self, rhs: &Millis) -> Millis {
		let (Millis(t0), Millis(t1)) = (*self, *rhs);
		Millis(t0 + t1)
	}
}

impl Sub<Millis,Millis> for Millis {
	#[inline(always)]
	fn sub(&self, rhs: &Millis) -> Millis {
		let (Millis(t0), Millis(t1)) = (*self, *rhs);
		Millis(t0 - t1)
	}
}

#[deriving(Eq,Ord)]
pub struct Velocity(f64);

impl Neg<Velocity> for Velocity {
	#[inline(always)]
	fn neg(&self) -> Velocity {
		let Velocity(v0) = *self;
		Velocity(-v0)
	}
}

impl Add<Velocity, Velocity> for Velocity {
	#[inline(always)]
	fn add(&self, rhs: &Velocity) -> Velocity {
		let (Velocity(v0), Velocity(v1)) = (*self, *rhs);
		Velocity(v0 + v1)
	}
}

impl Sub<Velocity, Velocity> for Velocity {
	#[inline(always)]
	fn sub(&self, rhs: &Velocity) -> Velocity {
		let (Velocity(v0), Velocity(v1)) = (*self, *rhs);
		Velocity(v0 - v1)
	}
}

impl Mul<Millis,Game> for Velocity {
	#[inline(always)]
	fn mul(&self, rhs: &Millis) -> Game {
		let (Velocity(v0), Millis(t)) = (*self, *rhs);
		Game(v0 * t as f64)
	}
}

#[deriving(Eq,Ord)]
pub struct Acceleration(f64);

impl Mul<Millis, Velocity> for Acceleration {
	#[inline(always)]
	fn mul(&self, rhs: &Millis) -> Velocity {
		let (Acceleration(a), Millis(t)) = (*self, *rhs);
		Velocity(a * t as f64)
	}
}

impl Neg<Acceleration> for Acceleration {
	#[inline(always)]
	fn neg(&self) -> Acceleration {
		let Acceleration(a) = *self;
		Acceleration(-a)
	}
}

pub type Frame = uint;
pub type Fps = uint;
