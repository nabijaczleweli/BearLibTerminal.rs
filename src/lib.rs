//! Slightly rustic FFI for [BearLibTerminal](https://bitbucket.org/cfyzium/bearlibterminal).
//!
//! All [BearLibTerminal](https://bitbucket.org/cfyzium/bearlibterminal) function equivalents reside inside the [`terminal`](terminal/index.html) module.
//!
//! # Examples
//!
//! ```ignore
//! use bear_lib_terminal::terminal;
//!
//! terminal::open("Test", 80, 30);
//! terminal::print_xy(0, 0, "[color=red]asdf[bkcolor=blue]asdf");
//! terminal::refresh();
//! let _ = terminal::wait_event();
//! terminal::close();
//! ```


extern crate bear_lib_terminal_sys;

mod colors;
pub mod geometry;
pub mod terminal;

pub use colors::Color;
