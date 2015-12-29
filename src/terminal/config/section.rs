use std::fmt;
use std::path::Path;
use geometry::Size;
use terminal::config::{ConfigPart, escape_config_string};


/// The `terminal` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section repr.
///
/// See [`terminal::set()`](../fn.set.html).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Terminal {
	encoding: Option<String>,  //TODO: use an enum/validated struct?
}

/// The `window` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section repr.
///
/// `None` values will not override current ones.
///
/// See [`terminal::set()`](../fn.set.html).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Window {
	size: Option<Size>,
	cellsize: Option<Cellsize>,
	title: Option<String>,
	icon: Option<String>,
	resizeable: Option<bool>,
	fullscreen: Option<bool>,
}

/// The `input` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section repr.
///
/// `None` values will not override current ones.
///
/// See [`terminal::set()`](../fn.set.html).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Input {
	precise_mouse: Option<bool>,
	mouse_cursor: Option<bool>,
	cursor_symbol: Option<char>,
	cursor_blink_rate: Option<i32>,
}

/// The `output` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section repr.
///
/// `None` values will not override current ones.
///
/// See [`terminal::set()`](../fn.set.html).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Output {
	postformatting: Option<bool>,
	vsync: Option<bool>,
}


/// The `log` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section repr.
///
/// `None` values will not override current ones.
///
/// See [`terminal::set()`](../fn.set.html).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Log {
	file: Option<String>,
	level: Option<LogLevel>,
	mode: Option<LogMode>,
}


/// Possible cell size, `Auto` will make the size be selected based on the font.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Cellsize {
	Auto,
	Sized(Size),
}

/// Logging levels, as specified [here](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum LogLevel {
	None,
	Fatal,
	Error,
	Warning,
	Info,
	Debug,
	Trace,
}

/// Log writing mode.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum LogMode {
	/// Reset the log each time.
	Truncate,
	/// Continue writing at the end.
	Append,
}


impl Terminal {
	/// Construct a new `terminal` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section override
	/// segment with a specified used for unibyte strings, which is better left at default, as Rust uses UTF-8 for everything.
	///
	/// Default: `"utf8"`.
	pub fn new(encoding: String) -> Terminal {
		Terminal{
			encoding: Some(encoding),
		}
	}
}

impl Window {
	/// Construct a `window` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section override segment
	/// with everything being equal to `None`.
	pub fn empty() -> Window {
		Window{
			size: None,
			cellsize: None,
			title: None,
			icon: None,
			resizeable: None,
			fullscreen: None,
		}
	}

	/// Window size in cells.
	///
	/// Default: `80x25`.
	pub fn size                (mut self, size: Size)         -> Self {self.size       = Some(size)                                       ; self}

	/// Size of all cells, in pixels.
	///
	/// Default: [`Cellsize::Auto`](enum.Cellsize.html#variant.Auto).
	pub fn cellsize            (mut self, cellsize: Cellsize) -> Self {self.cellsize   = Some(cellsize)                                   ; self}

	/// The terminal window's title.
	///
	/// Default: `"BearLibTerminal"`.
	pub fn title               (mut self, title: String)      -> Self {self.title      = Some(title)                                      ; self}

	/// The path of the icon used for the terminal window.
	///
	/// Default: none.
	pub fn icon<T: AsRef<Path>>(mut self, icon: T)            -> Self {self.icon       = Some(icon.as_ref().to_str().unwrap().to_string()); self}

	/// Whether the terminal window should be resizeable.
	///
	/// Default: `false`.
	pub fn resizeable          (mut self, resizeable: bool)   -> Self {self.resizeable = Some(resizeable)                                 ; self}

	/// Whether to enforce fullscreen mode.
	///
	/// Default: `false`.
	pub fn fullscreen          (mut self, fullscreen: bool)   -> Self {self.fullscreen = Some(fullscreen)                                 ; self}
}

impl Input {
	/// Construct an `input` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section override segment
	/// with all elements equal to `None`.
	pub fn empty() -> Input {
		Input{
			precise_mouse: None,
			mouse_cursor: None,
			cursor_symbol: None,
			cursor_blink_rate: None,
		}
	}

	/// Whether to generate a mouse-move event when a mouse moves from one pixel to another as opposed to from one cell to another.
	///
	/// Default: `false`.
	pub fn precise_mouse    (mut self, precise_mouse: bool)    -> Self {self.precise_mouse     = Some(precise_mouse)    ; self}

	/// Whether to show the cursor.
	///
	/// Default: `true`.
	pub fn mouse_cursor     (mut self, mouse_cursor: bool)     -> Self {self.mouse_cursor      = Some(mouse_cursor)     ; self}

	/// The cursor symbol to blink in the read string function.
	///
	/// Default: `'_'` a.k.a. `0x5F`.
	pub fn cursor_symbol    (mut self, cursor_symbol: char)    -> Self {self.cursor_symbol     = Some(cursor_symbol)    ; self}

	/// Amount of time in milliseconds to blink the cursor symbol for.
	///
	/// Default: `500`.
	pub fn cursor_blink_rate(mut self, cursor_blink_rate: i32) -> Self {self.cursor_blink_rate = Some(cursor_blink_rate); self}
}

impl Output {
	/// Construct an `output` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section override segment
	/// with all values equalling `None`
	pub fn clean() -> Output {
		Output{
			postformatting: None,
			vsync: None,
		}
	}

	/// Whether to process special tags in the [`print()`](../fn.print.html) function.
	///
	/// Default: `true`.
	pub fn postformatting(mut self, postformatting: bool) -> Self {self.postformatting = Some(postformatting); self}

	/// Toggle OpenGL VSync.
	///
	/// Default: `true`.
	pub fn vsync         (mut self, vsync: bool)          -> Self {self.vsync          = Some(vsync);          self}
}

impl Log {
	/// Construct an `log` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section override segment
	/// with everything set to `None`
	pub fn empty() -> Log {
		Log{
			file: None,
			level: None,
			mode: None,
		}
	}

	/// The file to write the log to. Note, that, IME, it didn't work.
	///
	/// Default: `"bearlibterminal.log"`
	pub fn file (mut self, file: String)    -> Log {self.file  = Some(file) ; self}

	/// The minimal log level to print at.
	///
	/// Default: [`LogLevel::Error`](enum.LogLevel.html#variant.Error)
	pub fn level(mut self, level: LogLevel) -> Log {self.level = Some(level); self}

	/// How to write to the log file if one laready exists.
	///
	/// Default: [`LogMode::Truncate`](enum.LogMode.html#variant.Truncate)
	pub fn mode (mut self, mode: LogMode)   -> Log {self.mode  = Some(mode) ; self}
}


impl ConfigPart for Terminal {
	fn to_config_str(&self) -> String {
		match self.encoding {
			Some(ref encoding) => format!("terminal.encoding={};", escape_config_string(&encoding)),
			None               => "".to_string(),
		}
	}
}

impl ConfigPart for Window {
	fn to_config_str(&self) -> String {
		if self.size.is_some() || self.cellsize.is_some() || self.title.is_some() || self.icon.is_some() || self.resizeable.is_some() ||
		   self.fullscreen.is_some() {
			format!("window: {}, {}, {}, {}, {}, {};",
				match self.size {
					Some(ref size) => format!("size={}", size),
					None           => "".to_string(),
				},
				match self.cellsize {
					Some(ref cellsize) =>
						match cellsize {
							&Cellsize::Sized(size) => format!("cellsize={}", size),
							&Cellsize::Auto        => "cellsize=auto".to_string(),
						},
					None               => "".to_string(),
				},
				match self.title {
					Some(ref title) => format!("title={}", escape_config_string(&title)),
					None            => "".to_string(),
				},
				match self.icon {
					Some(ref icon) => format!("icon={}", escape_config_string(&icon)),
					None           => "".to_string(),
				},
				match self.resizeable {
					Some(ref resizeable) => format!("resizeable={}", resizeable),
					None                 => "".to_string(),
				},
				match self.fullscreen {
					Some(ref fullscreen) => format!("fullscreen={}", fullscreen),
					None                 => "".to_string(),
				},
			)
		} else {
			"".to_string()
		}
	}
}

impl ConfigPart for Input {
	fn to_config_str(&self) -> String {
		if self.precise_mouse.is_some() || self.mouse_cursor.is_some() || self.cursor_symbol.is_some() || self.cursor_blink_rate.is_some() {
			format!("input: {}, {}, {}, {};",
				match self.precise_mouse {
					Some(ref precise_mouse) => format!("precise-mouse={}", precise_mouse),
					None                    => "".to_string(),
				},
				match self.mouse_cursor {
					Some(ref mouse_cursor) => format!("mouse-cursor={}", mouse_cursor),
					None                   => "".to_string(),
				},
				match self.cursor_symbol {
					Some(ref cursor_symbol) => format!("cursor-symbol=0x{:x}", *cursor_symbol as i8),
					None                    => "".to_string(),
				},
				match self.cursor_blink_rate {
					Some(ref cursor_blink_rate) => format!("cursor-blink-rate={}", cursor_blink_rate),
					None                        => "".to_string(),
				},
			)
		} else {
			"".to_string()
		}
	}
}

impl ConfigPart for Output {
	fn to_config_str(&self) -> String {
		if self.postformatting.is_some() || self.vsync.is_some() {
			format!("output: {}, {};",
				match self.postformatting {
					Some(ref postformatting) => format!("postformatting={}", postformatting),
					None                     => "".to_string(),
				},
				match self.vsync {
					Some(ref vsync) => format!("vsync={}", vsync),
					None            => "".to_string(),
				},
			)
		} else {
			"".to_string()
		}
	}
}

impl ConfigPart for Log {
	fn to_config_str(&self) -> String {
		if self.file.is_some() || self.level.is_some() || self.mode.is_some() {
			format!("log: {}, {}, {};",
				match self.file {
					Some(ref file) => format!("file={}", escape_config_string(&file)),
					None           => "".to_string(),
				},
				match self.level {
					Some(ref level) => format!("level={}", level),
					None            => "".to_string(),
				},
				match self.mode {
					Some(ref mode) => format!("mode={}", mode),
					None            => "".to_string(),
				},
			)
		} else {
			"".to_string()
		}
	}
}


impl fmt::Display for LogLevel {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str(match self {
			&LogLevel::None    => "none",
			&LogLevel::Fatal   => "fatal",
			&LogLevel::Error   => "error",
			&LogLevel::Warning => "warning",
			&LogLevel::Info    => "info",
			&LogLevel::Debug   => "debug",
			&LogLevel::Trace   => "trace",
		})
	}
}

impl fmt::Display for LogMode {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str(match self {
			&LogMode::Truncate => "truncate",
			&LogMode::Append   => "append",
		})
	}
}
