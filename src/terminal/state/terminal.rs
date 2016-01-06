use Color;
use terminal::{self, Event};
use std::char;
use geometry::Size;
use bear_lib_terminal_sys as ffi;


pub fn size() -> Size {
	Size::new(ffi::state(ffi::TK_WIDTH), ffi::state(ffi::TK_HEIGHT))
}

pub fn cell_size() -> Size {
	Size::new(ffi::state(ffi::TK_CELL_WIDTH), ffi::state(ffi::TK_CELL_HEIGHT))
}

pub fn foreground() -> Color {
	terminal::from_color_t(ffi::state(ffi::TK_COLOR) as ffi::ColorT)
}

pub fn background() -> Color {
	terminal::from_color_t(ffi::state(ffi::TK_BKCOLOR) as ffi::ColorT)
}

pub fn layer() -> i32 {
	ffi::state(ffi::TK_LAYER)
}

pub fn char() -> char {
	char::from_u32(ffi::state(ffi::TK_WCHAR) as u32).unwrap()
}

pub fn event() -> Option<Event> {
	terminal::to_event(ffi::state(ffi::TK_EVENT))
}

pub fn fullscreen() -> bool {
	ffi::check(ffi::TK_FULLSCREEN)
}
