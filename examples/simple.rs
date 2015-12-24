extern crate bear_lib_terminal;

use std::char;
use bear_lib_terminal::Color;
use bear_lib_terminal::geometry::Point;
use bear_lib_terminal::terminal::{self, Event, KeyCode};


fn main() {
	terminal::open("Simple example", 80, 30);
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
	let mut col = 0;
	while let Some(event) = terminal::wait_event() {
		match event {
			Event::KeyPressed{key, ctrl: _, shift} => {
				match key {
					KeyCode::A                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('A' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::B                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('B' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::C                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('C' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::D                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('D' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::E                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('E' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::F                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('F' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::G                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('G' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::H                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('H' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::I                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('I' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::J                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('J' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::K                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('K' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::L                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('L' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::M                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('M' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::N                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('N' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::O                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('O' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::P                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('P' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::Q                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('Q' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::R                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('R' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::S                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('S' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::T                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('T' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::U                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('U' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::V                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('V' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::W                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('W' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::X                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('X' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::Y                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('Y' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::Z                    => terminal::put_xy({col += 1; col - 1}, 5, char::from_u32('Z' as u32 + if shift {0} else {0x20}).unwrap()),
					KeyCode::Row0 | KeyCode::Num0 => terminal::put_xy({col += 1; col - 1}, 5, if shift {')'} else {'0'}),
					KeyCode::Row1 | KeyCode::Num1 => terminal::put_xy({col += 1; col - 1}, 5, if shift {'!'} else {'1'}),
					KeyCode::Row2 | KeyCode::Num2 => terminal::put_xy({col += 1; col - 1}, 5, if shift {'@'} else {'2'}),
					KeyCode::Row3 | KeyCode::Num3 => terminal::put_xy({col += 1; col - 1}, 5, if shift {'#'} else {'3'}),
					KeyCode::Row4 | KeyCode::Num4 => terminal::put_xy({col += 1; col - 1}, 5, if shift {'$'} else {'4'}),
					KeyCode::Row5 | KeyCode::Num5 => terminal::put_xy({col += 1; col - 1}, 5, if shift {'%'} else {'5'}),
					KeyCode::Row6 | KeyCode::Num6 => terminal::put_xy({col += 1; col - 1}, 5, if shift {'^'} else {'6'}),
					KeyCode::Row7 | KeyCode::Num7 => terminal::put_xy({col += 1; col - 1}, 5, if shift {'&'} else {'7'}),
					KeyCode::Row8 | KeyCode::Num8 => terminal::put_xy({col += 1; col - 1}, 5, if shift {'*'} else {'8'}),
					KeyCode::Row9 | KeyCode::Num9 => terminal::put_xy({col += 1; col - 1}, 5, if shift {'('} else {'9'}),
					KeyCode::Grave                => terminal::put_xy({col += 1; col - 1}, 5, if shift {'~'} else {'`'}),
					KeyCode::Minus                => terminal::put_xy({col += 1; col - 1}, 5, if shift {'_'} else {'-'}),
					KeyCode::Equals               => terminal::put_xy({col += 1; col - 1}, 5, if shift {'+'} else {'='}),
					KeyCode::LeftBracket          => terminal::put_xy({col += 1; col - 1}, 5, if shift {'{'} else {'['}),
					KeyCode::RightBracket         => terminal::put_xy({col += 1; col - 1}, 5, if shift {'}'} else {']'}),
					KeyCode::Backslash            => terminal::put_xy({col += 1; col - 1}, 5, if shift {'|'} else {'\\'}),
					KeyCode::Semicolon            => terminal::put_xy({col += 1; col - 1}, 5, if shift {':'} else {';'}),
					KeyCode::Apostrophe           => terminal::put_xy({col += 1; col - 1}, 5, if shift {'"'} else {'\''}),
					KeyCode::Comma                => terminal::put_xy({col += 1; col - 1}, 5, if shift {'<'} else {','}),
					KeyCode::Period               => terminal::put_xy({col += 1; col - 1}, 5, if shift {'>'} else {'.'}),
					KeyCode::Slash                => terminal::put_xy({col += 1; col - 1}, 5, if shift {'?'} else {'/'}),
					KeyCode::Space                => terminal::put_xy({col += 1; col - 1}, 5, ' '),
					KeyCode::Escape               => break,
					_ => (),
				}
				terminal::refresh();
			}
			Event::Close => break,
			_ => (),
		}
	}
	terminal::close();
}
