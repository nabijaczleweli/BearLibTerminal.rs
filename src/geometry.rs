use std::fmt;


/// Represents a single on-screen point/coordinate pair
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point {
	pub x: i32,
	pub y: i32,
}

impl Point {
	/// Creates a new point on the specified non-negative coordinates
	pub fn new(x: i32, y: i32) -> Self {
		assert!(x >= 0);
		assert!(y >= 0);

		Point{x: x, y: y}
	}
}


/// A 2D size representation
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Size {
	pub width: i32,
	pub height: i32,
}

impl Size {
	/// Creates a new non-negative size
	pub fn new(width: i32, height: i32) -> Self {
		assert!(width >= 0);
		assert!(height >= 0);

		Size{width: width, height: height}
	}
}

impl fmt::Display for Size {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "{}x{}", self.width, self.height)
	}
}


/// A rectangle, described by its four corners and a size
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Rect {
	pub top_left: Point,
	pub top_right: Point,
	pub bottom_right: Point,
	pub bottom_left: Point,
	pub size: Size,
}

impl Rect {
	pub fn from_size(origin: Point, size: Size) -> Self {
		let top_right    = Point::new(origin.x + size.width - 1, origin.y);
		let bottom_left  = Point::new(origin.x, origin.y + size.height - 1);
		let bottom_right = Point::new(top_right.x, bottom_left.y);

		Rect {
			top_left: origin,
			top_right: top_right,
			bottom_left: bottom_left,
			bottom_right: bottom_right,
			size: size
		}
	}

	pub fn from_points(top_left: Point, bottom_right: Point) -> Self {
		assert!(bottom_right.x >= top_left.x);
		assert!(bottom_right.y >= top_left.y);

		let size = Size::new(bottom_right.x - top_left.x, bottom_right.y - top_left.y);
		Self::from_size(top_left, size)
	}

	pub fn from_values(x: i32, y: i32, width: i32, height: i32) -> Self {
		let origin = Point::new(x, y);
		let size = Size::new(width, height);
		Self::from_size(origin, size)
	}

	pub fn from_point_values(top_left_x: i32, top_left_y: i32, bottom_right_x: i32, bottom_right_y: i32) -> Self {
		let top_left = Point::new(top_left_x, top_left_y);
		let bottom_right = Point::new(bottom_right_x, bottom_right_y);
		Self::from_points(top_left, bottom_right)
	}
}
