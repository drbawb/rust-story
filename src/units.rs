/// Milliseconds expressed as a large positive integer
/// This will be used at module boundaries in place of raw types.
#[deriving(Ord,Eq)]
pub struct Millis(uint);

impl Add<Millis,Millis> for Millis {
	fn add(&self, rhs: &Millis) -> Millis {
		let Millis(a) = *self;
		let Millis(b) = *rhs;

		Millis(a+b)
	}	
}

impl Mul<f64, f64> for Millis {
	fn mul(&self, rhs: &f64) -> f64 {
		let Millis(a) = *self;

		(*rhs) * (a as f64)
	}
}
