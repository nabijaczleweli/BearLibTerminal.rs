//! Wherein are contained functions for checking the terminal's properties: its size, whether cetrain keys are pressed, mouse position, etc.
//!
//! # Examples
//!
//! ```ignore
//! # // Ignore here, because on CI all `open()` calls will fail, as there's no graphical env. there
//! use bear_lib_terminal::terminal;
//! use bear_lib_terminal::geometry::Size;
//!
//! terminal::open("terminal::state example", 60, 25);
//! assert!(terminal::state::size() == Size::new(60, 25));
//! terminal::close();
//! ```


pub mod mouse;

use Color;
use std::char;
use geometry::Size;
use terminal::{self, Event, KeyCode};
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

/// Check, whether a [`KeyCode`](../enum.KeyCode.html)-specified key is currently pressed.
pub fn key_pressed(key: KeyCode) -> bool {
	ffi::check(match key {
		KeyCode::A            => ffi::TK_A,
		KeyCode::B            => ffi::TK_B,
		KeyCode::C            => ffi::TK_C,
		KeyCode::D            => ffi::TK_D,
		KeyCode::E            => ffi::TK_E,
		KeyCode::F            => ffi::TK_F,
		KeyCode::G            => ffi::TK_G,
		KeyCode::H            => ffi::TK_H,
		KeyCode::I            => ffi::TK_I,
		KeyCode::J            => ffi::TK_J,
		KeyCode::K            => ffi::TK_K,
		KeyCode::L            => ffi::TK_L,
		KeyCode::M            => ffi::TK_M,
		KeyCode::N            => ffi::TK_N,
		KeyCode::O            => ffi::TK_O,
		KeyCode::P            => ffi::TK_P,
		KeyCode::Q            => ffi::TK_Q,
		KeyCode::R            => ffi::TK_R,
		KeyCode::S            => ffi::TK_S,
		KeyCode::T            => ffi::TK_T,
		KeyCode::U            => ffi::TK_U,
		KeyCode::V            => ffi::TK_V,
		KeyCode::W            => ffi::TK_W,
		KeyCode::X            => ffi::TK_X,
		KeyCode::Y            => ffi::TK_Y,
		KeyCode::Z            => ffi::TK_Z,
		KeyCode::Row1         => ffi::TK_1,
		KeyCode::Row2         => ffi::TK_2,
		KeyCode::Row3         => ffi::TK_3,
		KeyCode::Row4         => ffi::TK_4,
		KeyCode::Row5         => ffi::TK_5,
		KeyCode::Row6         => ffi::TK_6,
		KeyCode::Row7         => ffi::TK_7,
		KeyCode::Row8         => ffi::TK_8,
		KeyCode::Row9         => ffi::TK_9,
		KeyCode::Row0         => ffi::TK_0,
		KeyCode::Enter        => ffi::TK_ENTER,
		KeyCode::Escape       => ffi::TK_ESCAPE,
		KeyCode::Backspace    => ffi::TK_BACKSPACE,
		KeyCode::Tab          => ffi::TK_TAB,
		KeyCode::Space        => ffi::TK_SPACE,
		KeyCode::Minus        => ffi::TK_MINUS,
		KeyCode::Equals       => ffi::TK_EQUALS,
		KeyCode::LeftBracket  => ffi::TK_LBRACKET,
		KeyCode::RightBracket => ffi::TK_RBRACKET,
		KeyCode::Backslash    => ffi::TK_BACKSLASH,
		KeyCode::Semicolon    => ffi::TK_SEMICOLON,
		KeyCode::Apostrophe   => ffi::TK_APOSTROPHE,
		KeyCode::Grave        => ffi::TK_GRAVE,
		KeyCode::Comma        => ffi::TK_COMMA,
		KeyCode::Period       => ffi::TK_PERIOD,
		KeyCode::Slash        => ffi::TK_SLASH,
		KeyCode::F1           => ffi::TK_F1,
		KeyCode::F2           => ffi::TK_F2,
		KeyCode::F3           => ffi::TK_F3,
		KeyCode::F4           => ffi::TK_F4,
		KeyCode::F5           => ffi::TK_F5,
		KeyCode::F6           => ffi::TK_F6,
		KeyCode::F7           => ffi::TK_F7,
		KeyCode::F8           => ffi::TK_F8,
		KeyCode::F9           => ffi::TK_F9,
		KeyCode::F10          => ffi::TK_F10,
		KeyCode::F11          => ffi::TK_F11,
		KeyCode::F12          => ffi::TK_F12,
		KeyCode::Pause        => ffi::TK_PAUSE,
		KeyCode::Insert       => ffi::TK_INSERT,
		KeyCode::Home         => ffi::TK_HOME,
		KeyCode::PageUp       => ffi::TK_PAGEUP,
		KeyCode::Delete       => ffi::TK_DELETE,
		KeyCode::End          => ffi::TK_END,
		KeyCode::PageDown     => ffi::TK_PAGEDOWN,
		KeyCode::Right        => ffi::TK_RIGHT,
		KeyCode::Left         => ffi::TK_LEFT,
		KeyCode::Down         => ffi::TK_DOWN,
		KeyCode::Up           => ffi::TK_UP,
		KeyCode::NumDivide    => ffi::TK_KP_DIVIDE,
		KeyCode::NumMultiply  => ffi::TK_KP_MULTIPLY,
		KeyCode::NumMinus     => ffi::TK_KP_MINUS,
		KeyCode::NumPlus      => ffi::TK_KP_PLUS,
		KeyCode::NumEnter     => ffi::TK_KP_ENTER,
		KeyCode::Num1         => ffi::TK_KP_1,
		KeyCode::Num2         => ffi::TK_KP_2,
		KeyCode::Num3         => ffi::TK_KP_3,
		KeyCode::Num4         => ffi::TK_KP_4,
		KeyCode::Num5         => ffi::TK_KP_5,
		KeyCode::Num6         => ffi::TK_KP_6,
		KeyCode::Num7         => ffi::TK_KP_7,
		KeyCode::Num8         => ffi::TK_KP_8,
		KeyCode::Num9         => ffi::TK_KP_9,
		KeyCode::Num0         => ffi::TK_KP_0,
		KeyCode::NumPeriod    => ffi::TK_KP_PERIOD,
		KeyCode::MouseLeft    => ffi::TK_MOUSE_LEFT,
		KeyCode::MouseRight   => ffi::TK_MOUSE_RIGHT,
		KeyCode::MouseMiddle  => ffi::TK_MOUSE_MIDDLE,
		KeyCode::MouseFourth  => ffi::TK_MOUSE_X1,
		KeyCode::MouseFifth   => ffi::TK_MOUSE_X2,
	})
}
