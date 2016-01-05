use geometry::Point;
use bear_lib_terminal_sys as ffi;


pub fn scroll() -> i32 {
	ffi::state(ffi::TK_MOUSE_WHEEL)
}

pub fn position() -> Point {
	Point::new(ffi::state(ffi::TK_MOUSE_X), ffi::state(ffi::TK_MOUSE_Y))
}

pub fn pixel_position() -> Point {
	Point::new(ffi::state(ffi::TK_MOUSE_PIXEL_X), ffi::state(ffi::TK_MOUSE_PIXEL_Y))
}

pub fn clicks() -> i32 {
	ffi::state(ffi::TK_MOUSE_CLICKS)
}
