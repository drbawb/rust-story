use graphics::Graphics;
use sprite::{Drawable, Sprite};
use units::{self, AsGame};

/// Each call to `next` yields digits from successively _increasing
/// powers of 10._ (e.g: 10, 100, 1000, etc.)
///
/// This can be used, for e.g, to yield the digits from a whole number.
/// Useful for impl. counters, status bars, prompts, etc.
///
struct DigitIter {
	is_zero:   bool,
	remainder: i32,
}

impl DigitIter {
	fn new(number: i32) -> DigitIter {
		DigitIter { remainder: number, is_zero: (number == 0) }
	}
}

impl Iterator for DigitIter {
	type Item = i32;
	fn next(&mut self) -> Option<i32> {
		if self.is_zero { self.is_zero = false; return Some(0) };
		if self.remainder <= 0 { return None; }

		let digit = self.remainder % 10; // grab last digit
		self.remainder /= 10;            // truncate power of 10
		Some(digit)                      // yield the truncated digit
	}
}

pub struct NumberSprite {
	digit_sprites: Vec<Sprite>,
}

impl NumberSprite {
	pub fn new(graphics: &mut Graphics, number: i32) -> NumberSprite {
		// map digits into sprite
		let digits  = DigitIter::new(number);
		let sprites = digits.map(|digit| {
			Sprite::new(
				graphics,
				(units::HalfTile(digit as u64), units::HalfTile(7)),
				(units::HalfTile(1), units::HalfTile(1)),
				format!("assets/base/TextBox.bmp"),
			)
		}).collect();

		NumberSprite { digit_sprites: sprites }
	}
}

impl<C: AsGame> Drawable<C> for NumberSprite {
	fn draw(&mut self, display: &mut Graphics, coords: (C, C)) {
		let (x, y) = coords;
		let (x, y) = (x.to_game(), y.to_game());

		for (idx, digit) in self.digit_sprites.iter_mut()
		                                      .enumerate() {
			// shift left by 1 half tile for digit pos
			let d_x = x - (units::HalfTile(1 * idx as u64));
			digit.draw(display, (d_x, y));
		}
	}
}

#[test]
fn test_digit_iter() {
	let mut iter  = DigitIter::new(42);
	let digit_10  = iter.next();
	let digit_100 = iter.next();
	assert_eq!(digit_10,  Some(2));
	assert_eq!(digit_100, Some(4));
}

#[test]
fn test_zero_iter() {
	let mut iter  = DigitIter::new(0);
	let digit_10  = iter.next();
	let digit_100 = iter.next();
	assert_eq!(digit_10,  Some(0));
	assert_eq!(digit_100, None);
}
