use std::f64;

static TILE_SIZE: i32 	= 32;
static SCALE: f64	= 1.0; // This is a divider of TILE_SIZE, 1.0 for 32, 2.0 for 16, etc.

pub trait AsGame 	{fn to_game(&self) -> Game;}
pub trait AsTile 	{fn to_tile(&self) -> Tile;}
pub trait AsPixel 	{fn to_pixel(&self) -> Pixel;}

/// A `Game` unit represents a density-independent distance in pixels.
/// Converting a `Game` to pixels will round it to the nearest coordinate,
/// scaled based on the desired tile size & resolution.
#[deriving(Eq,Ord)]
pub struct Game(f64);

impl AsGame for Game {
	#[inline(always)]
	fn to_game(&self) -> Game { *self }
}

impl AsTile for Game {
	#[inline(always)]	
	fn to_tile(&self) -> Tile {
		let Game(a) = *self;	
		Tile((a / TILE_SIZE as f64) as uint)
	}
}

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

/// A `Tile` represents a single square drawn in the game's
/// _base tile-size_ (32 pixels).
#[deriving(Eq,Ord)]
pub struct Tile(uint);

impl AsGame for Tile {
	#[inline(always)]	
	fn to_game(&self) -> Game {
		let Tile(a) = *self;	
		Game((a * (TILE_SIZE as uint)) as f64)
	}
}

impl AsTile for Tile {
	#[inline(always)]
	fn to_tile(&self) -> Tile { *self }
}

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

/// Millis represents a length of time in milliseconds as a signed integer.
/// (NOTE: As `Millis` supports basic arithmetic: "negative time" is possible.)
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

/// Velocity represents the current speed of an object.
/// This speed is measured in Games/Millis, and is stored as a float.
///
/// (Note: this is actually `Pixels/ms`, but `Games` are used as
/// they are higher precision types, they will also automatically
/// scale the render distance when converted to pixels.)
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

/// Any velocity multiplied by some length in time `t`
/// results in a distance measured in `Games`
impl Mul<Millis,Game> for Velocity {
	#[inline(always)]
	fn mul(&self, rhs: &Millis) -> Game {
		let (Velocity(v0), Millis(t)) = (*self, *rhs);
		Game(v0 * t as f64)
	}
}

/// Acceleration is defined as `(Games/ms)/ms`
#[deriving(Eq,Ord)]
pub struct Acceleration(f64);

/// Acceleration `a` multipled by some time `t` results
/// in `Velocity(a * t)`
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

#[deriving(Eq,Ord)]
pub struct Degrees(f64);

impl Degrees {
	/// Degrees are converted to radians as follows: `Degrees * (PI / 180.0)`
	pub fn to_radians(&self) -> f64 {
		let Degrees(d) = *self;
		d * (f64::consts::PI / 180.0)
	}
}

impl Add<Degrees,Degrees> for Degrees {
	#[inline(always)]
	fn add(&self, rhs: &Degrees) -> Degrees {
		let (Degrees(d0), Degrees(d1)) = (*self, *rhs);
		Degrees(d0 + d1)
	}
}

/// Degrees `d` multiplied by Games `g` yields `Degrees(d * g)`
impl<T:AsGame> Mul<T, Degrees> for Degrees {
	#[inline(always)]
	fn mul(&self, rhs: &T) -> Degrees {
		let (Degrees(d), Game(g)) = (*self, rhs.to_game());
		Degrees(d * g)
	}
}

/// Some number of Degrees `d` divided by some time `t` yields
/// an AngularVelocity `av`
impl Div<Millis,AngularVelocity> for Degrees {
	#[inline(always)]
	fn div(&self, rhs: &Millis) -> AngularVelocity {
		let (Degrees(d), Millis(t)) = (*self, *rhs);
		AngularVelocity(d / t as f64)
	}
}

/// AngularVelocity is defined as `Degrees/Millis` and is stored in a float.
#[deriving(Eq,Ord)]
pub struct AngularVelocity(f64);

/// Some AngularVelocity `av` multiplied by some time `t` yields
/// a number of degrees `d`.
impl Mul<Millis, Degrees> for AngularVelocity {
	#[inline(always)]
	fn mul(&self, rhs: &Millis) -> Degrees {
		let (AngularVelocity(av), Millis(t)) = (*self, *rhs);
		Degrees(av * t as f64)
	}
}



pub type Frame = uint;
pub type Fps = uint;
