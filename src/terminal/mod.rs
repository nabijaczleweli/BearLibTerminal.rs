//! Rusticized interface for the FFI.


mod input;
pub mod config;

use std::char;
use colors::Color;
use geometry::{Rect, Point, Size};
use self::config::{ConfigPart, Window};
use bear_lib_terminal_sys as ffi;
use bear_lib_terminal_sys::ColorT;

pub use self::input::{Event, KeyCode};


/// Creates the terminal window of the specified size with the specified title, without showing it.
/// To show the window use the [`refresh()`](fn.refresh.html) function.
///
/// Equivalent to the [`terminal_open()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#open) with a subsequent call to
/// the [`terminal_set()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#set) with the title.
pub fn open(title: &str, width: u32, height: u32) {
	ffi::open();
	set(Window::empty().size(Size::new(width as i32, height as i32)).title(title.to_string()));
}

/// Closes the terminal window, causing all subsequent functions from the module (apart from [`open()`](fn.open.html)) to fail
///
/// Equivalent to the [`terminal_close()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#close).
pub fn close() {
	ffi::close();
}

/// Invoke the [`terminal_set()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#set) with the argument's `config_str`.
///
/// Returns `false` iff the config string is malformed.
///
/// For build-in [`ConfigPart`](config/trait.ConfigPart.html)s see the [`config`](config/index.html) module.
pub fn set<T: ConfigPart>(cfg: T) -> bool {
	ffi::set(&*&cfg.to_config_str())
}

/// Flushes all changes made to the screen; also shows the window after the [`open()`](fn.open.html) call
///
/// Equivalent to the [`terminal_refresh()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#refresh).
pub fn refresh() {
	ffi::refresh();
}

/// Clears the screen (either partailly or fully)
///
/// If `area` is `None`, clears the entire screen, all layers
/// (identically to the [`terminal_clear()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#clear).
///
/// Otherwise clears specified area on the current layer, as would the
/// [`terminal_clear_area()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#clear_area).
pub fn clear(area: Option<Rect>) {
	match area {
		Some(rect) => ffi::clear_area(rect.top_left.x, rect.top_left.y, rect.bottom_right.x, rect.bottom_right.y),
		None       => ffi::clear(),
	}
}

/// Sets the current layer's crop area.
///
/// <sub>I don't get it either, refer the [`terminal_crop()` C API function's documentation](http://foo.wyrd.name/en:bearlibterminal:reference#crop).</sub>
pub fn crop(rect: Rect) {
	ffi::crop(rect.top_left.x, rect.top_left.y, rect.size.width, rect.size.height);
}

/// Selects the current layer.
///
/// The layer `index` must be between 0 and 255.
/// For more information consult the documentation for the [`terminal_layer()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#layer).
pub fn layer(index: i32) {
	ffi::layer(index);
}

/// Sets the current foreground color, which will affect all the output functions called later.
///
/// This is equivalent to the [`terminal_color()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#color).
pub fn set_foreground(color: Color) {
	ffi::color(to_color_t(color));
}

/// Sets the foreground color before calling the function and resets it afterwards.
pub fn with_foreground<F: Fn()>(color: Color, callback: F) {
	let current = ffi::state_color(ffi::TK_COLOR);
	set_foreground(color);
	callback();
	ffi::color(current);
}

/// Sets the current background color, which will affect all the output functions called later.
///
/// This is equivalent to the [`terminal_bkcolor()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#bkcolor).
pub fn set_background(color: Color) {
	ffi::bkcolor(to_color_t(color));
}

/// Sets the background color before calling the function and resets it afterwards.
pub fn with_background<F: Fn()>(color: Color, callback: F) {
	let current = ffi::state_color(ffi::TK_BKCOLOR);
	set_background(color);
	callback();
	ffi::bkcolor(current);
}

/// Sets the current foreground and background color, which will affect all the output functions called later.
///
/// This is equivalent to calling [`set_background()`](fn.set_background.html) and [`set_foreground()`](fn.set_foreground.html) in succession.
pub fn set_colors(fg: Color, bg: Color) {
	set_foreground(fg);
	set_background(bg);
}

/// Sets the foreground and background color before calling the function and resets them afterwards.
pub fn with_colors<F: Fn()>(fg: Color, bg: Color, callback: F) {
	with_foreground(fg, ||
		with_background(bg, ||
			callback()
		)
	);
}

/// Enable or disable composition, (dis)allowing for "stacking" tiles on top of each other in the same cell.
///
/// For details and other uses consult the documentation for the
/// [`terminal_composition()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#composition).
pub fn composition(enable: bool) {
	ffi::composition(enable);
}

/// Prints the specified character to the specified location.
///
/// Equivalent to the [`terminal_put()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#put).
pub fn put(point: Point, cell: char) {
	ffi::put(point.x, point.y, cell as i32);
}

/// Equivalent to [`put()`](fn.put.html) with a `Point` constructed from the first two arguments.
pub fn put_xy(x: i32, y: i32, cell: char) {
	ffi::put(x, y, cell as i32);
}

/// Prints the specified character to the specified pixel-offsetted location, gradient-colouring it from the corners.
///
/// For details see the docs for the [`terminal_put_ext()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#put_ext).
pub fn put_ext(pos: Point, offset: Point, cell: char, corners: &Vec<Color>) {
	ffi::put_ext(pos.x, pos.y, offset.x, offset.y, cell as i32, &corners.iter().cloned().map(to_color_t).collect::<Vec<_>>()[..]);
}

/// Get the character in the specified coordinates on the specified layer.
///
/// Returns 0 if the cell is empty on the specified layer.
///
/// Consult the documentation for the [`terminal_pick()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#pick) for more data.
pub fn pick(point: Point, index: i32) -> char {
	char::from_u32(ffi::pick(point.x, point.y, index) as u32).unwrap()
}

/// Get the color of the character in the specified coordinates on the specified layer.
///
/// Consult the documentation for the [`terminal_pick_color()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#pick_color),
/// despite its laconicity.
pub fn pick_foreground_color(point: Point, index: i32) -> Color {
	from_color_t(ffi::pick_color(point.x, point.y, index))
}

/// Get the background color in the specified coordinates.
///
/// Consult the documentation for the [`terminal_pick_bkcolor()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#pick_bkcolor)
/// for the same amount of information.
pub fn pick_background_color(point: Point) -> Color {
	from_color_t(ffi::pick_bkcolor(point.x, point.y))
}

/// Prints the specified string to the specified location, formatting it along the way.
///
/// For formatting spec see the docs for the [`terminal_print()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#print).
pub fn print(point: Point, value: &str) {
	let _ = ffi::print(point.x, point.y, value);
}

/// Equivalent to [`print()`](fn.print.html) with a `Point` constructed from the first two arguments.
pub fn print_xy(x: i32, y: i32, value: &str) {
	print(Point::new(x, y), value);
}

/// Calculate the argument's width/height without printing it.
///
/// Whether the function returns the width or the height depends on the presence of the `bbox` tag in the string.
///
/// Refer to the [docs for the `terminal_measure()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#measure), note,
/// that the return type therein is incorrect.
pub fn measure(value: &str) -> i32 {
	ffi::measure(value)
}

/// Check, whether the next [`read_event()`](fn.read_event.html) call will return `Some`.
///
/// Consult the [documentation for the `terminal_has_input()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#has_input).
pub fn has_input() -> bool {
	ffi::has_input()
}

/// Returns the next event, blocks until one's available.
///
/// This is equivalent to the [`terminal_read()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#read).
pub fn wait_event() -> Option<Event> {
	to_event(ffi::read())
}

/// Returns the next event in the queue if it's available, otherwise returns `None`.
///
/// If one intends on waiting for events, the [`wait_event()`](fn.wait_event.html) function is recommended.
///
/// This is equivalent to the behaviour mentioned in the
/// [docs for the `terminal_read()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#read), but not the function's behaviour itself.
pub fn read_event() -> Option<Event> {
	if !has_input() {
		None
	} else {
		wait_event()
	}
}

/// Returns the next event in the queue if it's available without popping it therefrom, otherwise returns `None`.
///
/// If one intends on waiting for events, the [`wait_event()`](fn.wait_event.html) function is recommended.
///
/// If one intends on popping the events, the [`read_event()`](fn.read_event.html) function is recommended.
///
/// If one intends on just checking if an event is ready, the [`has_input()`](fn.has_input.html) function is recommended.
///
/// This is equivalent to the [`terminal_peek()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#peek).
pub fn peek_event() -> Option<Event> {
	match ffi::peek() {
		0 => None,
		event => to_event(event),
	}
}

/// Reads up to `max` characters without parsing, starting from the specified coordinates.
///
/// Returns `None` if the user closed the window or pressed `Escape`,
/// `Some` containing the read string otherwise.
///
/// The read string will contain up to `max` characters.
///
/// The string being read will be kept on-screen only *during* the reading process, the scene will be restored before returning.
pub fn read_str(point: Point, max: i32) -> Option<String> {
	ffi::read_str(point.x, point.y, max)
}

/// Sleep for the specified amount of milliseconds.
///
/// See the [`terminal_delay()` C API function's documentation](http://foo.wyrd.name/en:bearlibterminal:reference#delay).
pub fn delay(period: i32) {
	ffi::delay(period)
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
	let key      = code & !ffi::TK_KEY_RELEASED;
	let released = (code & ffi::TK_KEY_RELEASED) == ffi::TK_KEY_RELEASED;

	match key {
		ffi::TK_SHIFT   => Some(if released {Event::ShiftReleased}   else {Event::ShiftPressed}),
		ffi::TK_CONTROL => Some(if released {Event::ControlReleased} else {Event::ControlPressed}),
		key             => {
			let ctrl  = ffi::check(ffi::TK_CONTROL);
			let shift = ffi::check(ffi::TK_SHIFT);

			match to_keycode(key) {
				Some(converted) => Some(get_key(released, converted, ctrl, shift)),
				None            => None,
			}
		}
	}
}

fn get_mouse_move() -> Event {
	Event::MouseMove{
		x: ffi::state(ffi::TK_MOUSE_X),
		y: ffi::state(ffi::TK_MOUSE_Y),
	}
}

fn get_mouse_scroll() -> Event {
	Event::MouseScroll{
		delta: ffi::state(ffi::TK_MOUSE_WHEEL),
	}
}

fn get_key(released: bool, key: KeyCode, ctrl: bool, shift: bool) -> Event {
	if released {
		Event::KeyReleased{
			key: key,
			ctrl: ctrl,
			shift: shift,
		}
	} else {
		Event::KeyPressed{
			key: key,
			ctrl: ctrl,
			shift: shift,
		}
	}
}
