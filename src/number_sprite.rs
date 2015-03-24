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
	remainder: i32,
}

impl DigitIter {
	fn new(number: i32) -> DigitIter {
		DigitIter { remainder: number }
	}
}

impl Iterator for DigitIter {
	type Item = i32;
	fn next(&mut self) -> Option<i32> {
		if self.remainder <= 0 { return None; }

		let digit = self.remainder % 10; // grab last digit
		self.remainder /= 10;       // truncate power of 10
		Some(digit)                 // yield the truncated digit
	}
}

pub struct NumberSprite {
	sprite: Sprite,
}

impl NumberSprite {
	pub fn new(graphics: &mut Graphics, number: i32) -> NumberSprite {
		let digit = Sprite::new(
			graphics,
			(units::HalfTile(4), units::HalfTile(7)),
			(units::HalfTile(1), units::HalfTile(1)),
			format!("assets/base/TextBox.bmp"),
		);

		NumberSprite { sprite: digit }
	}
}

impl<C: AsGame> Drawable<C> for NumberSprite {
	fn draw(&mut self, display: &mut Graphics, coords: (C, C)) {
		self.sprite.draw(display, coords);
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

