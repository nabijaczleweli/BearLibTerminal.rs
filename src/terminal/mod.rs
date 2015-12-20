mod ffi;
mod input;

use geometry::{Rect, Point};
use colors::Color;
use self::ffi::ColorT;
pub use self::input::{Event, KeyCode};

pub fn open(title: &str, width: i32, height: i32) {
	let config = format!("
		window: title='{}', size={}x{};
		input: precise-mouse=false, filter=[keyboard, system];
		log: level=debug;
		font: DejaVuSansMono.ttf, size=14;
	", title, width, height);

	ffi::open();
	ffi::set(&config);
}

pub fn print(point: Point, value: &str) {
	let _ = ffi::print(point.x, point.y, value);
}

pub fn print_xy(x: i32, y: i32, value: &str) {
	print(Point::new(x, y), value);
}

pub fn put(point: Point, cell: char) {
	ffi::put(point.x, point.y, cell as i32);
}

pub fn refresh() {
	ffi::refresh()
}

pub fn clear(area: Option<Rect>) {
	match area {
		Some(rect) => ffi::clear_area(rect.top_left.x, rect.top_left.y, rect.bottom_right.x, rect.bottom_right.y),
		None => ffi::clear(),
	}
}

pub fn read_event() -> Option<Event> {
	if !has_event() {
		None
	} else {
		wait_event()
	}
}

pub fn wait_event() -> Option<Event> {
	to_event(ffi::read())
}

pub fn has_event() -> bool {
	ffi::peek() != 0
}

pub fn set_foreground(color: Color) {
	ffi::color(to_color_t(color));
}

pub fn with_foreground<F: Fn()>(color: Color, callback: F) {
	let current = ffi::state_color(ffi::TK_COLOR);
	set_foreground(color);
	callback();
	ffi::color(current);
}

pub fn set_background(color: Color) {
	ffi::bkcolor(to_color_t(color));
}

pub fn with_background<F: Fn()>(color: Color, callback: F) {
	let current = ffi::state_color(ffi::TK_BKCOLOR);
	set_background(color);
	callback();
	ffi::bkcolor(current);
}

pub fn set_colors(fg: Color, bg: Color) {
	set_foreground(fg);
	set_background(bg);
}

pub fn with_colors<F: Fn()>(fg: Color, bg: Color, callback: F) {
	with_foreground(fg, || {
		with_background(bg, || {
			callback();
		});
	});
}

fn from_color_t(color: ColorT) -> Color {
	let alpha = ((color >> 24) & 0xFF) as u8;
	let red   = ((color >> 16) & 0xFF) as u8;
	let green = ((color >> 8)  & 0xFF) as u8;
	let blue  = (color         & 0xFF) as u8;

	Color::from_rgba(red, green, blue, alpha)
}

fn to_color_t(color: Color) -> ColorT {
	(
		((color.alpha as ColorT) << 24) |
		((color.red   as ColorT) << 16) |
		((color.green as ColorT) << 8)  |
		(color.blue   as ColorT)
	)
}

fn to_keycode(code: i32) -> Option<KeyCode> {
	match code {
		ffi::TK_A            => Some(KeyCode::A),
		ffi::TK_B            => Some(KeyCode::B),
		ffi::TK_C            => Some(KeyCode::C),
		ffi::TK_D            => Some(KeyCode::D),
		ffi::TK_E            => Some(KeyCode::E),
		ffi::TK_F            => Some(KeyCode::F),
		ffi::TK_G            => Some(KeyCode::G),
		ffi::TK_H            => Some(KeyCode::H),
		ffi::TK_I            => Some(KeyCode::I),
		ffi::TK_J            => Some(KeyCode::J),
		ffi::TK_K            => Some(KeyCode::K),
		ffi::TK_L            => Some(KeyCode::L),
		ffi::TK_M            => Some(KeyCode::M),
		ffi::TK_N            => Some(KeyCode::N),
		ffi::TK_O            => Some(KeyCode::O),
		ffi::TK_P            => Some(KeyCode::P),
		ffi::TK_Q            => Some(KeyCode::Q),
		ffi::TK_R            => Some(KeyCode::R),
		ffi::TK_S            => Some(KeyCode::S),
		ffi::TK_T            => Some(KeyCode::T),
		ffi::TK_U            => Some(KeyCode::U),
		ffi::TK_V            => Some(KeyCode::V),
		ffi::TK_W            => Some(KeyCode::W),
		ffi::TK_X            => Some(KeyCode::X),
		ffi::TK_Y            => Some(KeyCode::Y),
		ffi::TK_Z            => Some(KeyCode::Z),
		ffi::TK_1            => Some(KeyCode::Row1),
		ffi::TK_2            => Some(KeyCode::Row2),
		ffi::TK_3            => Some(KeyCode::Row3),
		ffi::TK_4            => Some(KeyCode::Row4),
		ffi::TK_5            => Some(KeyCode::Row5),
		ffi::TK_6            => Some(KeyCode::Row6),
		ffi::TK_7            => Some(KeyCode::Row7),
		ffi::TK_8            => Some(KeyCode::Row8),
		ffi::TK_9            => Some(KeyCode::Row9),
		ffi::TK_0            => Some(KeyCode::Row0),
		ffi::TK_ENTER        => Some(KeyCode::Enter),
		ffi::TK_ESCAPE       => Some(KeyCode::Escape),
		ffi::TK_BACKSPACE    => Some(KeyCode::Backspace),
		ffi::TK_TAB          => Some(KeyCode::Tab),
		ffi::TK_SPACE        => Some(KeyCode::Space),
		ffi::TK_MINUS        => Some(KeyCode::Minus),
		ffi::TK_EQUALS       => Some(KeyCode::Equals),
		ffi::TK_LBRACKET     => Some(KeyCode::LeftBracket),
		ffi::TK_RBRACKET     => Some(KeyCode::RightBracket),
		ffi::TK_BACKSLASH    => Some(KeyCode::Backslash),
		ffi::TK_SEMICOLON    => Some(KeyCode::Semicolon),
		ffi::TK_APOSTROPHE   => Some(KeyCode::Apostrophe),
		ffi::TK_GRAVE        => Some(KeyCode::Grave),
		ffi::TK_COMMA        => Some(KeyCode::Comma),
		ffi::TK_PERIOD       => Some(KeyCode::Period),
		ffi::TK_SLASH        => Some(KeyCode::Slash),
		ffi::TK_F1           => Some(KeyCode::F1),
		ffi::TK_F2           => Some(KeyCode::F2),
		ffi::TK_F3           => Some(KeyCode::F3),
		ffi::TK_F4           => Some(KeyCode::F4),
		ffi::TK_F5           => Some(KeyCode::F5),
		ffi::TK_F6           => Some(KeyCode::F6),
		ffi::TK_F7           => Some(KeyCode::F7),
		ffi::TK_F8           => Some(KeyCode::F8),
		ffi::TK_F9           => Some(KeyCode::F9),
		ffi::TK_F10          => Some(KeyCode::F10),
		ffi::TK_F11          => Some(KeyCode::F11),
		ffi::TK_F12          => Some(KeyCode::F12),
		ffi::TK_PAUSE        => Some(KeyCode::Pause),
		ffi::TK_INSERT       => Some(KeyCode::Insert),
		ffi::TK_HOME         => Some(KeyCode::Home),
		ffi::TK_PAGEUP       => Some(KeyCode::PageUp),
		ffi::TK_DELETE       => Some(KeyCode::Delete),
		ffi::TK_END          => Some(KeyCode::End),
		ffi::TK_PAGEDOWN     => Some(KeyCode::PageDown),
		ffi::TK_RIGHT        => Some(KeyCode::Right),
		ffi::TK_LEFT         => Some(KeyCode::Left),
		ffi::TK_DOWN         => Some(KeyCode::Down),
		ffi::TK_UP           => Some(KeyCode::Up),
		ffi::TK_KP_DIVIDE    => Some(KeyCode::NumDivide),
		ffi::TK_KP_MULTIPLY  => Some(KeyCode::NumMultiply),
		ffi::TK_KP_MINUS     => Some(KeyCode::NumMinus),
		ffi::TK_KP_PLUS      => Some(KeyCode::NumPlus),
		ffi::TK_KP_ENTER     => Some(KeyCode::NumEnter),
		ffi::TK_KP_1         => Some(KeyCode::Num1),
		ffi::TK_KP_2         => Some(KeyCode::Num2),
		ffi::TK_KP_3         => Some(KeyCode::Num3),
		ffi::TK_KP_4         => Some(KeyCode::Num4),
		ffi::TK_KP_5         => Some(KeyCode::Num5),
		ffi::TK_KP_6         => Some(KeyCode::Num6),
		ffi::TK_KP_7         => Some(KeyCode::Num7),
		ffi::TK_KP_8         => Some(KeyCode::Num8),
		ffi::TK_KP_9         => Some(KeyCode::Num9),
		ffi::TK_KP_0         => Some(KeyCode::Num0),
		ffi::TK_KP_PERIOD    => Some(KeyCode::NumPeriod),
		ffi::TK_MOUSE_LEFT   => Some(KeyCode::MouseLeft),
		ffi::TK_MOUSE_RIGHT  => Some(KeyCode::MouseRight),
		ffi::TK_MOUSE_MIDDLE => Some(KeyCode::MouseMiddle),
		ffi::TK_MOUSE_X1     => Some(KeyCode::MouseFourth),
		ffi::TK_MOUSE_X2     => Some(KeyCode::MouseFifth),
		_                    => None,
	}
}

fn to_event(code: i32) -> Option<Event> {
	match code {
		ffi::TK_CLOSE        => Some(Event::Close),
		ffi::TK_RESIZED      => Some(Event::Resize),
		ffi::TK_MOUSE_MOVE   => Some(get_mouse_move()),
		ffi::TK_MOUSE_SCROLL => Some(get_mouse_scroll()),
		_                    => to_key_event(code),
	}
}

fn to_key_event(code: i32) -> Option<Event> {
	let key = code & !ffi::TK_KEY_RELEASED;
	let released = (code & ffi::TK_KEY_RELEASED) == ffi::TK_KEY_RELEASED;
	let ctrl = ffi::check(ffi::TK_CONTROL);
	let shift = ffi::check(ffi::TK_SHIFT);

	match to_keycode(key) {
		Some(converted) => Some(get_key(released, converted, ctrl, shift)),
		None            => None,
	}
}

fn get_mouse_move() -> Event {
	Event::MouseMove {
		x: ffi::state(ffi::TK_MOUSE_X),
		y: ffi::state(ffi::TK_MOUSE_Y),
	}
}

fn get_mouse_scroll() -> Event {
	Event::MouseScroll {
		delta: ffi::state(ffi::TK_MOUSE_WHEEL),
	}
}

fn get_key(released: bool, key: KeyCode, ctrl: bool, shift: bool) -> Event {
	if released {
		Event::KeyReleased {
			key: key,
			ctrl: ctrl,
			shift: shift,
		}
	} else {
		Event::KeyPressed {
			key: key,
			ctrl: ctrl,
			shift: shift,
		}
	}
}
