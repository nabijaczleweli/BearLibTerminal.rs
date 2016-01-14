extern crate bear_lib_terminal;

use bear_lib_terminal::Color;
use bear_lib_terminal::geometry::Point;
use bear_lib_terminal::terminal::{self, config, Event, KeyCode};


fn main() {
	terminal::open("Simple example", 80, 30);
	terminal::set(config::Window::empty().resizeable(true));

	terminal::print_xy(0, 0, "Your mom");
	terminal::with_colors(Color::from_rgb(0xFA, 0xAF, 0x29), Color::from_rgb(0x05, 0x50, 0xD6), || terminal::print_xy(0, 1, "Colerd"));
	for (i, c) in "Coloured letters with pixel-offset!".chars().enumerate() {
		terminal::put_ext(Point::new(i as i32, 2), Point::new(i as i32, i as i32), c, &vec![Color::from_rgb(0xFF, 0x00, 0x00),
		                                                                                    Color::from_rgb(0x00, 0xFF, 0x00),
		                                                                                    Color::from_rgb(0x00, 0x00, 0xFF),
		                                                                                    Color::from_rgb(0xFF, 0xFF, 0xFF)]);
	}
	terminal::refresh();

	terminal::set_foreground(Color::from_rgb(0xFF, 0xFF, 0xFF));
	if let Some(string) = terminal::read_str(Point::new(0, 5), 30) {
		terminal::print_xy(0, 5, &*&string);
	}
	terminal::refresh();
	for event in terminal::events() {
		match event {
			Event::Resize{width, height} => {
				terminal::print_xy(0, 0, &*&format!("Width: {}\nHeight: {}", width, height));
				terminal::refresh();
			},
			Event::Close | Event::KeyPressed{key: KeyCode::Escape, ctrl: _, shift: _} => break,
			_                                                                         => (),
		}
	}
	terminal::close();
}
