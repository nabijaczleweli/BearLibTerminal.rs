use std::path::Path;
use geometry::Size;


/// The `terminal` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section repr.
///
/// See [`terminal::configure()`](../fn.configure.html).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Terminal {
	/// Encoding used for unibyte strings. This is better left at default, as Rust uses UTF-8 for everything.
	///
	/// Default: `"utf8"`
	pub encoding: Option<String>, //TODO: use an enum/validated struct?
}

impl Terminal {
	/// Construct a new `terminal` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section override
	/// segment
	///
	/// `None` values will not override current ones.
	pub fn new(encoding: Option<String>) -> Terminal {
		Terminal{
			encoding: encoding,
		}
	}
}

/// The `window` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section repr.
///
/// See [`terminal::configure()`](../fn.configure.html).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Window {
	/// Window size in cells.
	///
	/// Default: `80x25`
	pub size: Option<Size>,
	/// Size of all cells, in pixels.
	///
	/// Default: [`Cellsize::Auto`](enum.Cellsize.html#variant.Auto)
	pub cellsize: Option<Cellsize>,
	/// The terminal window's title.
	///
	/// Default: `"BearLibTerminal"`
	pub title: Option<String>,
	/// The path of the icon used for the terminal window.
	///
	/// Default: none
	pub icon: Option<String>,
	/// Whether the terminal window should be resizeable.
	///
	/// Default: `false`
	pub resizeable: Option<bool>,
	/// Whether to enforce fullscreen mode.
	///
	/// Default: `false`
	pub fullscreen: Option<bool>,
}

impl Window {
	/// Construct a new `window` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section override segment
	///
	/// `None` values will not override current ones.
	pub fn new<T: AsRef<Path>>(size: Option<Size>, cellsize: Option<Cellsize>, title: Option<String>, icon: Option<T>, resizeable: Option<bool>,
	                           fullscreen: Option<bool>) -> Window {
		Window{
			size: size,
			cellsize: cellsize,
			title: title,
			icon: icon.map(|s| s.as_ref().to_str().unwrap().to_string()),
			resizeable: resizeable,
			fullscreen: fullscreen,
		}
	}
}


/// The `input` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section repr.
///
/// See [`terminal::configure()`](../fn.configure.html).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Input {
	/// Whether to generate a mouse-move event when a mouse moves from one pixel to another as opposed to from one cell to another.
	///
	/// Default: `false`
	pub precise_mouse: Option<bool>,
	/// Whether to show the cursor.
	///
	/// Default: `true`
	pub mouse_cursor: Option<bool>,
	/// The cursor symbol to blink in the read string function.
	///
	/// Default: `'_'` a.k.a. `0x5F`
	pub cursor_symbol: Option<char>,
	/// Amount of time in milliseconds to blink the cursor symbol for.
	///
	/// Default: `500`
	pub cursor_blink_rate: Option<i32>,
}

impl Input {
	/// Construct a new `input` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section override segment
	///
	/// `None` values will not override current ones.
	pub fn new(precise_mouse: Option<bool>, mouse_cursor: Option<bool>, cursor_symbol: Option<char>, cursor_blink_rate: Option<i32>) -> Input {
		Input{
			precise_mouse: precise_mouse,
			mouse_cursor: mouse_cursor,
			cursor_symbol: cursor_symbol,
			cursor_blink_rate: cursor_blink_rate,
		}
	}
}


/// The `output` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section repr.
///
/// See [`terminal::configure()`](../fn.configure.html).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Output {
	/// Whether to process special tags in the [`print()`](../fn.print.html) function.
	///
	/// Default: `true`
	pub postformatting: Option<bool>,
	/// Toggle OpenGL VSync.
	///
	/// Default: `true`
	pub vsync: Option<bool>,
}

impl Output {
	/// Construct a new `output` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section override segment
	///
	/// `None` values will not override current ones.
	pub fn new(postformatting: Option<bool>, vsync: Option<bool>) -> Output {
		Output{
			postformatting: postformatting,
			vsync: vsync,
		}
	}
}


/// The `log` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section repr.
///
/// See [`terminal::configure()`](../fn.configure.html).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Log {
	/// The file to write the log to. Note, that, IME, it didn't work.
	///
	/// Default: `"bearlibterminal.log"`
	pub file: Option<String>,
	/// The minimal log level to print at.
	///
	/// Default: [`LogLevel::Error`](enum.LogLevel.html#variant.Error)
	pub level: Option<LogLevel>,
	/// How to write to the log file if one laready exists.
	///
	/// Default: [`LogMode::Truncate`](enum.LogMode.html#variant.Truncate)
	pub mode: Option<LogMode>,
}

impl Log {
	/// Construct a new `log` [configuration](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#library_configuration) section override segment
	///
	/// `None` values will not override current ones.
	pub fn new(file: Option<String>, level: Option<LogLevel>, mode: Option<LogMode>) -> Log {
		Log{
			file: file,
			level: level,
			mode: mode,
		}
	}
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
	/// Restart the log each time.
	Truncate,
	/// Continue writing at the end.
	Append,
}
