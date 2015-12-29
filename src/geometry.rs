//! Helper `struct`s for abstracting on-screen geometry.


use std::fmt;


/// Represents a single on-screen point/coordinate pair.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point {
	pub x: i32,
	pub y: i32,
}

impl Point {
	/// Creates a new point on the specified non-negative coordinates
	pub fn new(x: i32, y: i32) -> Point {
		assert!(x >= 0);
		assert!(y >= 0);

		Point{
			x: x,
			y: y
		}
	}
}


/// A 2D size representation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Size {
	pub width: i32,
	pub height: i32,
}

impl Size {
	/// Creates a new non-negative size.
	pub fn new(width: i32, height: i32) -> Size {
		assert!(width >= 0);
		assert!(height >= 0);

		Size{
			width: width,
			height: height,
		}
	}
}

impl fmt::Display for Size {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "{}x{}", self.width, self.height)
	}
}


/// A rectangle, described by its four corners and a size.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Rect {
	/// The top-left corner.
	pub top_left: Point,
	/// The top-right corner.
	pub top_right: Point,
	/// The bottom-right corner.
	pub bottom_right: Point,
	/// The bottom-left corner.
	pub bottom_left: Point,
	/// The `Rect`angle's size.
	pub size: Size,
}

impl Rect {
	/// Construct a `Rect` from its top-left corner and its size.
	///
	/// # Examples
	///
	/// ```
	/// # use bear_lib_terminal::geometry::{Rect, Point, Size};
	/// let rect = Rect::from_size(Point::new(10, 20), Size::new(30, 40));
	/// assert_eq!(rect.top_left, Point::new(10, 20));
	/// assert_eq!(rect.top_right, Point::new(40, 20));
	/// assert_eq!(rect.bottom_left, Point::new(10, 60));
	/// assert_eq!(rect.bottom_right, Point::new(40, 60));
	/// assert_eq!(rect.size, Size::new(30, 40));
	/// ```
	pub fn from_size(origin: Point, size: Size) -> Rect {
		let top_right    = Point::new(origin.x + size.width, origin.y);
		let bottom_left  = Point::new(origin.x, origin.y + size.height);
		let bottom_right = Point::new(top_right.x, bottom_left.y);

		Rect{
			top_left: origin,
			top_right: top_right,
			bottom_left: bottom_left,
			bottom_right: bottom_right,
			size: size
		}
	}

	/// Construct a `Rect` from its top-left and bottom-right corners.
	///
	/// # Examples
	///
	/// ```
	/// # use bear_lib_terminal::geometry::{Rect, Point, Size};
	/// let rect = Rect::from_points(Point::new(10, 20), Point::new(30, 40));
	/// assert_eq!(rect.top_left, Point::new(10, 20));
	/// assert_eq!(rect.top_right, Point::new(30, 20));
	/// assert_eq!(rect.bottom_left, Point::new(10, 40));
	/// assert_eq!(rect.bottom_right, Point::new(30, 40));
	/// assert_eq!(rect.size, Size::new(20, 20));
	/// ```
	pub fn from_points(top_left: Point, bottom_right: Point) -> Rect {
		assert!(bottom_right.x >= top_left.x);
		assert!(bottom_right.y >= top_left.y);

		let size = Size::new(bottom_right.x - top_left.x, bottom_right.y - top_left.y);
		Rect::from_size(top_left, size)
	}

	/// Construct a `Rect` from its top-left corner and its size, values unwrapped.
	///
	/// # Examples
	///
	/// ```
	/// # use bear_lib_terminal::geometry::{Rect, Point, Size};
	/// assert_eq!(Rect::from_values(10, 20, 30, 40), Rect::from_size(Point::new(10, 20), Size::new(30, 40)));
	/// ```
	pub fn from_values(x: i32, y: i32, width: i32, height: i32) -> Rect {
		let origin = Point::new(x, y);
		let size = Size::new(width, height);
		Rect::from_size(origin, size)
	}


	/// Construct a `Rect` from its top-left and bottom-right corners, values unwrapped.
	///
	/// # Examples
	///
	/// ```
	/// # use bear_lib_terminal::geometry::{Rect, Point, Size};
	/// assert_eq!(Rect::from_point_values(10, 20, 30, 40), Rect::from_points(Point::new(10, 20), Point::new(30, 40)));
	/// ```
	pub fn from_point_values(top_left_x: i32, top_left_y: i32, bottom_right_x: i32, bottom_right_y: i32) -> Rect {
		let top_left = Point::new(top_left_x, top_left_y);
		let bottom_right = Point::new(bottom_right_x, bottom_right_y);
		Rect::from_points(top_left, bottom_right)
	}
}
