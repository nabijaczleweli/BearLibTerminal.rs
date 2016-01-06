use Color;
use terminal::{self, Event};
use std::char;
use geometry::Size;
use bear_lib_terminal_sys as ffi;


/// Get the terminal size in cells.
pub fn size() -> Size {
	Size::new(ffi::state(ffi::TK_WIDTH), ffi::state(ffi::TK_HEIGHT))
}

/// Get cell size in pixels.
pub fn cell_size() -> Size {
	Size::new(ffi::state(ffi::TK_CELL_WIDTH), ffi::state(ffi::TK_CELL_HEIGHT))
}

/// Get the currently selected foreground colour.
///
/// Foreground colours are changed by using the [`terminal::*_foreground()`](../index.html) function family.
pub fn foreground() -> Color {
	terminal::from_color_t(ffi::state(ffi::TK_COLOR) as ffi::ColorT)
}

/// Get the currently selected background colour.
///
/// Background colours are changed by using the [`terminal::*_background()`](../index.html) function family.
pub fn background() -> Color {
	terminal::from_color_t(ffi::state(ffi::TK_BKCOLOR) as ffi::ColorT)
}

/// Get the currently selected layer.
///
/// Layer is selected by using the [`terminal::layer()`](../fn.layer.html) function.
pub fn layer() -> i32 {
	ffi::state(ffi::TK_LAYER)
}

/// Most-recent-event-produced unicode character.
pub fn char() -> char {
	char::from_u32(ffi::state(ffi::TK_WCHAR) as u32).unwrap()
}

/// Get last dequeued event.
///
/// Returns `None` iff no events have been dequeued yet.
pub fn event() -> Option<Event> {
	terminal::to_event(ffi::state(ffi::TK_EVENT))
}

/// Check, whether the terminal is currently full-screen.
pub fn fullscreen() -> bool {
	ffi::check(ffi::TK_FULLSCREEN)
}
