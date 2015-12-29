//! Module containing BLT configuration options.
//!
//! # Examples
//! The following from [the official documentation](http://foo.wyrd.name/en:bearlibterminal:reference#set):
//!
//! ```text
//! window: title='foobar', size=80x25; input.filter='keyboard'
//! ```
//!
//! Is equivalent to
//!
//! ```
//! use bear_lib_terminal::terminal::{self, config};
//! use bear_lib_terminal::geometry::Size;
//! # assert!({let result =
//! terminal::set(config::Window::empty().title("foobar".to_string()).size(Size::new(80, 25)));
//! # result}); assert!({let result =
//! terminal::set(vec![config::InputFilter::Group{group: config::InputFilterGroup::Keyboard, both: false}]);
//! # result});
//! ```


mod section;
mod input_filter;

pub mod font;

pub use self::section::*;
pub use self::input_filter::*;


/// Trait for generating BLT configuration strings.
///
/// Those will get fed directly to the [`terminal_set()` C API function](http://foo.wyrd.name/en:bearlibterminal:reference#set).
pub trait ConfigPart {
	fn to_config_str(&self) -> String;
}


/// Escapes `'`s and wraps the strings with `'`s, as per [this](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#configuration_string_format).
///
/// # Examples
/// ```
/// # use bear_lib_terminal::terminal::config::escape_config_string;
/// assert_eq!(escape_config_string(&"".to_string()), "''");
/// ```
/// ```
/// # use bear_lib_terminal::terminal::config::escape_config_string;
/// assert_eq!(escape_config_string(&"'".to_string()), "''''");
/// ```
/// ```
/// # use bear_lib_terminal::terminal::config::escape_config_string;
/// assert_eq!(escape_config_string(&"asdf'asdf".to_string()), "'asdf''asdf'");
/// ```
pub fn escape_config_string(cfg: &String) -> String {
	format!("'{}'", cfg.replace("'", "''"))
}
