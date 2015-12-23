extern crate bear_lib_terminal;

use bear_lib_terminal::{terminal, Color};
use bear_lib_terminal::geometry::Point;


fn main() {
	terminal::open("Simple example", 80, 30);
	terminal::print_xy(0, 0, "Your mom");
	terminal::with_colors(Color::from_rgb(0xFA, 0xAF, 0x29), Color::from_rgb(0x05, 0x50, 0xD6), || terminal::print_xy(0, 1, "Colerd"));
	for (i, c) in "Coloured letters!".chars().enumerate() {
		terminal::put_ext(Point::new(i as i32, 2), Point::new(0, 0), c, &vec![Color::from_rgb(0xFF, 0x00, 0x00),
		                                                                      Color::from_rgb(0x00, 0xFF, 0x00),
		                                                                      Color::from_rgb(0x00, 0x00, 0xFF),
		                                                                      Color::from_rgb(0xFF, 0xFF, 0xFF)]);
	}
	terminal::refresh();
	let _ = terminal::wait_event();
	terminal::close();
}
