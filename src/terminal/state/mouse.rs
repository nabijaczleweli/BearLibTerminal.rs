//! Checking the state of mouse-related properties, namely the mouse cursor's position, `n`-clicks and scrolling.


use geometry::Point;
use bear_lib_terminal_sys as ffi;


/// Amount of steps the mouse wheel scrolled in the last [`Event::MouseScroll`](../../enum.Event.html#variant.MouseScroll).
///
/// Negative values indicate an "up" scroll.
///
/// Positive values indicate a "down" scroll.
pub fn scroll() -> i32 {
	ffi::state(ffi::TK_MOUSE_WHEEL)
}

/// Get the mouse cursor's position in cells.
pub fn position() -> Point {
	Point::new(ffi::state(ffi::TK_MOUSE_X), ffi::state(ffi::TK_MOUSE_Y))
}

/// Get the mouse cursor's position in pixels.
pub fn pixel_position() -> Point {
	Point::new(ffi::state(ffi::TK_MOUSE_PIXEL_X), ffi::state(ffi::TK_MOUSE_PIXEL_Y))
}

/// Amount of fast consecutive clicks for the [`Event::KeyPressed`](../../enum.Event.html#variant.KeyPressed)
/// with [`key: Mouse*`](../../enum.Event.html#variant.KeyPressed).
pub fn clicks() -> i32 {
	ffi::state(ffi::TK_MOUSE_CLICKS)
}
