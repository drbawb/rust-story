pub struct Rectangle {
	x: int, 
	y: int,
	width: int, 
	height: int
}

impl Rectangle {
	pub fn new(width: int, height: int) -> Rectangle {
		Rectangle {
			x: 0, y: 0,
			width: width, height: height
		}
	}

	pub fn left(&self) 		-> int { self.x }
	pub fn right(&self) 	-> int { self.x + self.width }
	pub fn top(&self) 		-> int { self.y }
	pub fn bottom(&self) 	-> int { self.y + self.height}

	pub fn width(&self) 	-> int { self.width }
	pub fn height(&self) 	-> int { self.height }
}