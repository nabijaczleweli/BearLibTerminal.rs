mod section;
mod input_filter;

pub mod font;

pub use self::section::*;
pub use self::input_filter::*;


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
