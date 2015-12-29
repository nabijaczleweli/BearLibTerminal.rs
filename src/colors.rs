/// An RGBA colour repr.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Color {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: u8,
}

impl Color {
	/// Equivalent to [`from_rgba()`](fn.from_rgba.html) with full opacity
	pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
		Color{
			red: red,
			green: green,
			blue: blue,
			alpha: 0xFF,
		}
	}

	pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
		Color{
			red: red,
			green: green,
			blue: blue,
			alpha: alpha,
		}
	}
}
