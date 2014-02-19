use std::f64;

static TILE_SIZE: i32 	= 32;
static SCALE: f64		= 1.0;

pub struct Game(f64);
pub struct Pixel(i32);
pub struct Tile(uint);

pub struct Frame(uint);
pub struct Fps(uint);

pub struct Millis(int);
pub struct Velocity(f64);
pub struct Acceleration(f64);


trait AsGame 	{fn to_game(&self) -> Game;}
trait AsTile 	{fn to_tile(&self) -> Tile;}
trait AsPixel 	{fn to_pixel(&self) -> Pixel;}

/// Game's merely dereference themselves. 
impl AsGame for Game {
	#[inline(always)]
	fn to_game(&self) -> Game { *self }
}

/// A `Game` divided by the current `TILE_SIZE` (32 | 16) expressed
/// as an unsigned integer.
///
/// TODO: Perhaps I should round here as well
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

/// A `Pixel` merely dereferences itself, as it is already a `Pixel`.
impl AsPixel for Pixel {
	#[inline(always)]
	fn to_pixel(&self) -> Pixel { *self }
}

// Allow `+` operator for anything which can be converted `#to_game()` 
impl<T: AsGame>  Add<T, Game> for Game {
	#[inline(always)]	
	fn add(&self, rhs: &T) -> Game {
		let (Game(a), Game(b)) = (*self, rhs.to_game());
		Game(a + b)
	}
}

// Allow `*` operator for anything which can be converted `#to_game()`
impl <T: AsGame> Mul<T, Game> for Game {
	#[inline(always)]
	fn mul(&self, rhs: &T) -> Game {
		let (Game(a), Game(b)) = (*self, rhs.to_game());
		Game (a * b)
	}
}

// Allow `+` operator for anything which can be converted `#to_tile()`
impl<T: AsTile> Add<T, Tile> for Tile {
	#[inline(always)]	
	fn add(&self, rhs: &T) -> Tile {
		let (Tile(a), Tile(b)) = (*self, rhs.to_tile());
		Tile(a + b)
	}
}

// Allow `+` operator for anything which can be converted `#to_pixel()`
impl<T: AsPixel> Add<T, Pixel> for Pixel {
	#[inline(always)]
	fn add(&self, rhs: &T) -> Pixel {
		let (Pixel(a), Pixel(b)) = (*self, rhs.to_pixel());
		Pixel(a + b)
	}
}

fn main() {
	let mut gs = Game(325.0) * Game(2.0); // simple wraps
	println!("gs: {:?}", gs);	
	
	gs = gs + (Tile(1) + Tile(2)); 		
	println!("gs: {:?}", gs);
	
	let pixels = Pixel(1) + gs;
	println!("px: {:?}", pixels);
}
