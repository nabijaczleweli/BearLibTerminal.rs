use libc;
use std::mem;
use std::ffi::CString;


pub const TK_A          : i32 = 0x04;
pub const TK_B          : i32 = 0x05;
pub const TK_C          : i32 = 0x06;
pub const TK_D          : i32 = 0x07;
pub const TK_E          : i32 = 0x08;
pub const TK_F          : i32 = 0x09;
pub const TK_G          : i32 = 0x0A;
pub const TK_H          : i32 = 0x0B;
pub const TK_I          : i32 = 0x0C;
pub const TK_J          : i32 = 0x0D;
pub const TK_K          : i32 = 0x0E;
pub const TK_L          : i32 = 0x0F;
pub const TK_M          : i32 = 0x10;
pub const TK_N          : i32 = 0x11;
pub const TK_O          : i32 = 0x12;
pub const TK_P          : i32 = 0x13;
pub const TK_Q          : i32 = 0x14;
pub const TK_R          : i32 = 0x15;
pub const TK_S          : i32 = 0x16;
pub const TK_T          : i32 = 0x17;
pub const TK_U          : i32 = 0x18;
pub const TK_V          : i32 = 0x19;
pub const TK_W          : i32 = 0x1A;
pub const TK_X          : i32 = 0x1B;
pub const TK_Y          : i32 = 0x1C;
pub const TK_Z          : i32 = 0x1D;
pub const TK_1          : i32 = 0x1E;
pub const TK_2          : i32 = 0x1F;
pub const TK_3          : i32 = 0x20;
pub const TK_4          : i32 = 0x21;
pub const TK_5          : i32 = 0x22;
pub const TK_6          : i32 = 0x23;
pub const TK_7          : i32 = 0x24;
pub const TK_8          : i32 = 0x25;
pub const TK_9          : i32 = 0x26;
pub const TK_0          : i32 = 0x27;
pub const TK_ENTER      : i32 = 0x28;
pub const TK_ESCAPE     : i32 = 0x29;
pub const TK_BACKSPACE  : i32 = 0x2A;
pub const TK_TAB        : i32 = 0x2B;
pub const TK_SPACE      : i32 = 0x2C;
pub const TK_MINUS      : i32 = 0x2D;
pub const TK_EQUALS     : i32 = 0x2E;
pub const TK_LBRACKET   : i32 = 0x2F;
pub const TK_RBRACKET   : i32 = 0x30;
pub const TK_BACKSLASH  : i32 = 0x31;
pub const TK_SEMICOLON  : i32 = 0x33;
pub const TK_APOSTROPHE : i32 = 0x34;
pub const TK_GRAVE      : i32 = 0x35;
pub const TK_COMMA      : i32 = 0x36;
pub const TK_PERIOD     : i32 = 0x37;
pub const TK_SLASH      : i32 = 0x38;
pub const TK_F1         : i32 = 0x3A;
pub const TK_F2         : i32 = 0x3B;
pub const TK_F3         : i32 = 0x3C;
pub const TK_F4         : i32 = 0x3D;
pub const TK_F5         : i32 = 0x3E;
pub const TK_F6         : i32 = 0x3F;
pub const TK_F7         : i32 = 0x40;
pub const TK_F8         : i32 = 0x41;
pub const TK_F9         : i32 = 0x42;
pub const TK_F10        : i32 = 0x43;
pub const TK_F11        : i32 = 0x44;
pub const TK_F12        : i32 = 0x45;
pub const TK_PAUSE      : i32 = 0x48;
pub const TK_INSERT     : i32 = 0x49;
pub const TK_HOME       : i32 = 0x4A;
pub const TK_PAGEUP     : i32 = 0x4B;
pub const TK_DELETE     : i32 = 0x4C;
pub const TK_END        : i32 = 0x4D;
pub const TK_PAGEDOWN   : i32 = 0x4E;
pub const TK_RIGHT      : i32 = 0x4F;
pub const TK_LEFT       : i32 = 0x50;
pub const TK_DOWN       : i32 = 0x51;
pub const TK_UP         : i32 = 0x52;
pub const TK_KP_DIVIDE  : i32 = 0x54;
pub const TK_KP_MULTIPLY: i32 = 0x55;
pub const TK_KP_MINUS   : i32 = 0x56;
pub const TK_KP_PLUS    : i32 = 0x57;
pub const TK_KP_ENTER   : i32 = 0x58;
pub const TK_KP_1       : i32 = 0x59;
pub const TK_KP_2       : i32 = 0x5A;
pub const TK_KP_3       : i32 = 0x5B;
pub const TK_KP_4       : i32 = 0x5C;
pub const TK_KP_5       : i32 = 0x5D;
pub const TK_KP_6       : i32 = 0x5E;
pub const TK_KP_7       : i32 = 0x5F;
pub const TK_KP_8       : i32 = 0x60;
pub const TK_KP_9       : i32 = 0x61;
pub const TK_KP_0       : i32 = 0x62;
pub const TK_KP_PERIOD  : i32 = 0x63;
pub const TK_SHIFT      : i32 = 0x70;
pub const TK_CONTROL    : i32 = 0x71;

pub const TK_MOUSE_LEFT   : i32 = 0x80;
pub const TK_MOUSE_RIGHT  : i32 = 0x81;
pub const TK_MOUSE_MIDDLE : i32 = 0x82;
pub const TK_MOUSE_X1     : i32 = 0x83;
pub const TK_MOUSE_X2     : i32 = 0x84;
pub const TK_MOUSE_MOVE   : i32 = 0x85;
pub const TK_MOUSE_SCROLL : i32 = 0x86;
pub const TK_MOUSE_X      : i32 = 0x87;
pub const TK_MOUSE_Y      : i32 = 0x88;
#[allow(dead_code)]
pub const TK_MOUSE_PIXEL_X: i32 = 0x89;
#[allow(dead_code)]
pub const TK_MOUSE_PIXEL_Y: i32 = 0x8A;
pub const TK_MOUSE_WHEEL  : i32 = 0x8B;
#[allow(dead_code)]
pub const TK_MOUSE_CLICKS : i32 = 0x8C;

pub const TK_KEY_RELEASED: i32 = 0x100;

#[allow(dead_code)]
pub const TK_WIDTH      : i32 = 0xC0;
#[allow(dead_code)]
pub const TK_HEIGHT     : i32 = 0xC1;
#[allow(dead_code)]
pub const TK_CELL_WIDTH : i32 = 0xC2;
#[allow(dead_code)]
pub const TK_CELL_HEIGHT: i32 = 0xC3;
pub const TK_COLOR      : i32 = 0xC4;
pub const TK_BKCOLOR    : i32 = 0xC5;
#[allow(dead_code)]
pub const TK_LAYER      : i32 = 0xC6;
#[allow(dead_code)]
pub const TK_COMPOSITION: i32 = 0xC7;
#[allow(dead_code)]
pub const TK_CHAR       : i32 = 0xC8;
#[allow(dead_code)]
pub const TK_WCHAR      : i32 = 0xC9;
#[allow(dead_code)]
pub const TK_EVENT      : i32 = 0xCA;
#[allow(dead_code)]
pub const TK_FULLSCREEN : i32 = 0xCB;

pub const TK_CLOSE  : i32 = 0xE0;
pub const TK_RESIZED: i32 = 0xE1;

pub const TK_OFF: i32 = 0;
pub const TK_ON : i32 = 1;

#[allow(dead_code)]
pub const TK_INPUT_NONE     : i32 = 0;
#[allow(dead_code)]
pub const TK_INPUT_CANCELLED: i32 = -1;

pub type ColorT = u32;

#[link(name = "BearLibTerminal")]
extern {
	fn terminal_open() -> i32;
	fn terminal_close();
	fn terminal_set8(value: *const i8) -> i32;
	fn terminal_refresh();
	fn terminal_clear();
	fn terminal_clear_area(x: i32, y: i32, w: i32, h: i32);
	fn terminal_crop(x: i32, y: i32, w: i32, h: i32);
	fn terminal_layer(index: i32);
	fn terminal_color(color: ColorT);
	fn terminal_bkcolor(color: ColorT);
	fn terminal_composition(mode: i32);
	fn terminal_put(x: i32, y: i32, code: i32);
	fn terminal_put_ext(x: i32, y: i32, dx: i32, dy: i32, code: i32, corners: *const ColorT);
	fn terminal_pick(x: i32, y: i32, index: i32) -> i32;
	fn terminal_pick_color(x: i32, y: i32, index: i32) -> ColorT;
	fn terminal_pick_bkcolor(x: i32, y: i32) -> ColorT;
	fn terminal_print8(x: i32, y: i32, value: *const i8) -> i32;
	fn terminal_measure8(value: *const i8) -> i32;
	fn terminal_has_input() -> i32;
	fn terminal_state(code: i32) -> i32;
	fn terminal_read() -> i32;
	fn terminal_peek() -> i32;
	fn terminal_delay(period: i32);
}

fn with_utf8_ptr<T, F: Fn(*const i8) -> T>(value: &str, callback: F) -> T {
	let converted = CString::new(value).unwrap();
	let char_ptr = converted.as_ptr();
	unsafe {
		let i8_ptr = mem::transmute::<*const libc::c_char, *const i8>(char_ptr);
		callback(i8_ptr)
	}
}

pub fn open() -> bool {
	unsafe {
		terminal_open() != 0
	}
}

pub fn close() {
	unsafe {
		terminal_close()
	}
}

pub fn set(value: &str) -> bool {
	with_utf8_ptr(value, |ptr|
		unsafe {
			terminal_set8(ptr) != 0
		}
	)
}

pub fn refresh() {
	unsafe {
		terminal_refresh()
	}
}

pub fn clear() {
	unsafe {
		terminal_clear()
	}
}

pub fn clear_area(x: i32, y: i32, width: i32, height: i32) {
	assert!(x >= 0);
	assert!(y >= 0);
	assert!(width >= 0);
	assert!(height >= 0);

	unsafe {
		terminal_clear_area(x, y, width, height)
	}
}

pub fn crop(x: i32, y: i32, width: i32, height: i32) {
	assert!(x >= 0);
	assert!(y >= 0);
	assert!(width >= 0);
	assert!(height >= 0);

	unsafe {
		terminal_crop(x, y, width, height)
	}
}

pub fn layer(index: i32) {
	assert!(index >= 0);
	assert!(index <= 255);

	unsafe {
		terminal_layer(index)
	}
}

pub fn color(value: ColorT) {
	unsafe {
		terminal_color(value)
	}
}

pub fn bkcolor(value: ColorT) {
	unsafe {
		terminal_bkcolor(value)
	}
}

pub fn composition(enable: bool) {
	let value = if enable { TK_ON } else { TK_OFF };
	unsafe {
		terminal_composition(value)
	}
}

pub fn put(x: i32, y: i32, code: i32) {
	assert!(x >= 0);
	assert!(y >= 0);
	assert!(code >= 0);

	unsafe {
		terminal_put(x, y, code)
	}
}

pub fn put_ext(x: i32, y: i32, dx: i32, dy: i32, code: i32, corners: &[ColorT]) {
	assert!(x >= 0);
	assert!(y >= 0);
	assert!(code >= 0);
	assert!(corners.len() == 4);

	let corners_ptr = corners.as_ptr();
	unsafe {
		terminal_put_ext(x, y, dx, dy, code, corners_ptr)
	}
}

pub fn pick(x: i32, y: i32, index: i32) -> i32 {
	assert!(x >= 0);
	assert!(y >= 0);
	assert!(index >= 0);

	unsafe {
		terminal_pick(x, y, index)
	}
}

pub fn pick_color(x: i32, y: i32, index: i32) -> ColorT {
	assert!(x >= 0);
	assert!(y >= 0);
	assert!(index >= 0);

	unsafe {
		terminal_pick_color(x, y, index)
	}
}

pub fn pick_bkcolor(x: i32, y: i32) -> ColorT {
	assert!(x >= 0);
	assert!(y >= 0);

	unsafe {
		terminal_pick_bkcolor(x, y)
	}
}

pub fn print(x: i32, y: i32, value: &str) -> i32 {
	assert!(x >= 0);
	assert!(y >= 0);

	with_utf8_ptr(value, |ptr| {
		unsafe {
			terminal_print8(x, y, ptr)
		}
	})
}

pub fn measure(value: &str) -> i32 {
	with_utf8_ptr(value, |ptr|
		unsafe {
			terminal_measure8(ptr)
		}
	)
}

pub fn has_input() -> bool {
	unsafe {
		terminal_has_input() != 0
	}
}

pub fn state(code: i32) -> i32 {
	assert!(code >= 0);

	unsafe {
		terminal_state(code)
	}
}

pub fn state_color(code: i32) -> ColorT {
	unsafe {
		let color = state(code);
		mem::transmute::<i32, ColorT>(color)
	}
}

pub fn check(code: i32) -> bool {
	state(code) > 0
}

pub fn read() -> i32 {
	unsafe {
		terminal_read()
	}
}

pub fn peek() -> i32 {
	unsafe {
		terminal_peek()
	}
}

pub fn delay(period: i32) {
	unsafe {
		terminal_delay(period)
	}
}
