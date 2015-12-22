extern crate bear_lib_terminal;

use bear_lib_terminal::{terminal, Color};


fn main() {
	terminal::open("Simple example", 80, 30);
	terminal::print_xy(0, 0, "Your mom");
	terminal::with_colors(Color::from_rgb(0xFA, 0xAF, 0x29), Color::from_rgb(0x05, 0x50, 0xD6), || terminal::print_xy(0, 1, "Colerd"));
	terminal::refresh();
	let _ = terminal::wait_event();
	terminal::close();
}
