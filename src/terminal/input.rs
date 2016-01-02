/// All pressable keys.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KeyCode {
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,
	/// Top-row `1/!` key.
	Row1,
	/// Top-row `2/@` key.
	Row2,
	/// Top-row `3/#` key.
	Row3,
	/// Top-row `4/$` key.
	Row4,
	/// Top-row `5/%` key.
	Row5,
	/// Top-row `6/^` key.
	Row6,
	/// Top-row `7/&` key.
	Row7,
	/// Top-row `8/*` key.
	Row8,
	/// Top-row `9/(` key.
	Row9,
	/// Top-row `0/)` key.
	Row0,
	/// Top-row &#96;/~ key.
	Grave,
	/// Top-row `-/_` key.
	Minus,
	/// Top-row `=/+` key.
	Equals,
	/// Second-row `[/{` key.
	LeftBracket,
	/// Second-row `]/}` key.
	RightBracket,
	/// Second-row `\/|` key.
	Backslash,
	/// Third-row `;/:` key.
	Semicolon,
	/// Third-row `'/"` key.
	Apostrophe,
	/// Fourth-row `,/<` key.
	Comma,
	/// Fourth-row `./>` key.
	Period,
	/// Fourth-row `//?` key.
	Slash,
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
	Enter,
	Escape,
	Backspace,
	Tab,
	Space,
	Pause,
	Insert,
	Home,
	PageUp,
	Delete,
	End,
	PageDown,
	Right,
	Left,
	Down,
	Up,
	/// Numpad `/` key.
	NumDivide,
	/// Numpad `*` key.
	NumMultiply,
	/// Numpad `-` key.
	NumMinus,
	/// Numpad `+` key.
	NumPlus,
	/// Numpad &#9166; key.
	NumEnter,
	/// Numpad `Del/.` key (output locale-dependent).
	NumPeriod,
	/// Numpad `1/End` key.
	Num1,
	/// Numpad 2/&#8595; key.
	Num2,
	/// Numpad `3/PageDown` key.
	Num3,
	/// Numpad 4/&#8592; key.
	Num4,
	/// Numpad `5` key.
	Num5,
	/// Numpad 6/&#8594; key.
	Num6,
	/// Numpad `7/Home` key.
	Num7,
	/// Numpad 8/&#8593; key.
	Num8,
	/// Numpad `9/PageUp` key.
	Num9,
	/// Numpad `0/Insert` key.
	Num0,
	/// Left mouse button.
	MouseLeft,
	/// Right mouse button.
	MouseRight,
	/// Middle mouse button a.k.a. pressed scroll wheel.
	MouseMiddle,
	MouseFourth,
	MouseFifth,
}

/// A single input event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Event {
	/// Terminal window closed.
	Close,
	/// Terminal window resized. Needs to have `window.resizeable = true` to occur.
	///
	/// Note, that, as of [`40e6253`](https://bitbucket.org/cfyzium/bearlibterminal/commits/40e625311f0cccc43b94633add4dec0d6b77c2b7),
	/// the terminal window is cleared when resized.
	Resize{
		/// Width the terminal was resized to.
		width: i32,
		/// Heigth the terminal was resized to.
		height: i32,
	},
	/// Mouse moved.
	///
	/// If [`precise-mouse`](config/struct.Input.html#structfield.precise_mouse) is off, generated each time mouse moves from cell to cell, otherwise,
	/// when it moves from pixel to pixel.
	MouseMove{
		/// `0`-based cell index from the left to which the mouse cursor moved.
		x: i32,
		/// `0`-based cell index from the top to which the mouse cursor moved.
		y: i32
	},
	/// Mouse wheel moved.
	MouseScroll{
		/// Amount of steps the wheel rotated.
		///
		/// Positive when scrolled "down"/"backwards".
		///
		/// Negative when scrolled "up"/"forwards"/"away".
		delta: i32
	},
	/// A keyboard or mouse button pressed (might repeat, if set in OS).
	KeyPressed{
		/// The key pressed.
		key: KeyCode,
		/// Whether the Control key is pressed.
		ctrl: bool,
		/// Whether the Shift key is pressed.
		shift: bool
	},
	/// A keyboard or mouse button released.
	KeyReleased{
		/// The key released.
		key: KeyCode,
		/// Whether the Control key is pressed.
		ctrl: bool,
		/// Whether the Shift key is pressed.
		shift: bool
	},
	/// The Shift key pressed (might repeat, if set in OS).
	ShiftPressed,
	/// The Shift key released.
	ShiftReleased,
	/// The Shift key pressed (might repeat, if set in OS).
	ControlPressed,
	/// The Control key released.
	ControlReleased,
}
