// Author: Angel
// Playground: https://play.rust-lang.org
// Repo: https://github.com/gb-2312/emoji
// Date: Mon Mar  1 20:53:19 CST 2021
//
// Usage:
//    rustc main.rs
//    ./main

// import
use std::char;

// emoji unicode ref => http://www.unicode.org/charts/PDF/U1F300.pdf

// start position
const TABLE_START: u32 = 0x1f300;
// end position
const TABLE_END: u32 = 0x1f6ff;
// blank char
const BLANK_CHAR: char = ' ';
// next line char
const NEXT_LINE_CHAR: char = '\n';
// mask number of new line
const NEXT_LINE_MASK: u32 = 0x0f;

// defined struct of `emoji`
#[derive(Debug)]
pub struct Emoji<T> {
	// start position
	start: T,
	// end position
	end: T,
}

// defined interface of `IPrint`
pub trait IPrint {
	// get unicode-char
	// @param number type of u32
	fn get_unicode_char(number: u32) -> char;

	// bind & print emoji
	// @param emoji => pointer of `emoji`
	fn print(&self);
}

// bind & impl `IPrint`
impl IPrint for Emoji<u32> {
	// get unicode-char
	// @param number type of u32
	fn get_unicode_char(number: u32) -> char {
		unsafe { char::from_u32_unchecked(number) }
	}

	// bind & print emoji
	// @param emoji => pointer of `emoji`
	fn print(&self) {
		let start = self.start;
		let end = self.end;

		let mut i = start;
		let mut string_builder = String::new();

		// condition: i <= end
		while i <= end {
			let console = Self::get_unicode_char(i);
			string_builder.push(console);
			string_builder.push(BLANK_CHAR);

			// new line:
			// (the status of the lower 4 bits are all 1, need wrap!)
			if (i & NEXT_LINE_MASK) == NEXT_LINE_MASK {
				string_builder.push(NEXT_LINE_CHAR);
			}

			i += 1;
		}

		// string-container is not empty!
		if !string_builder.is_empty() {
			print!("{}", string_builder.to_string());
		}
	}
}

// main-method
fn main() {
	let emoji = Emoji {
		start: TABLE_START,
		end: TABLE_END,
	};
	emoji.print();
}
