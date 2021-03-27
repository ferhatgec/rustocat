// MIT License
//
// Copyright (c) 2020 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use std::fs::File;
use std::path::Path;

#[macro_use]
extern crate colodot;
use crate::colodot::dot;

mod header;
mod languages;
mod colors;

use std::io::{
	self,
	BufRead
};

use languages::{
	cplusplus,
	flascript,
	python,
	regular
};

use header::{
	print_top_header,
	print_bottom_header,
	header_text
};

const VERSION: &str = "0.1-beta-1";

fn read<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file(filename: &str, language: i8) {
	if let Ok(lines) = read(&filename) {
		// ahh. too bad code. sorry.

		if language == 0 {
			for line in lines {
				if let Ok(ip) = line {
					cplusplus(&ip);
				}
			}
		} else if language == 1 {
			for line in lines {
				if let Ok(ip) = line {
					flascript(&ip);
				}
			}
		} else if language == 2 {
			for line in lines {
				if let Ok(ip) = line {
					python(&ip);
				}
			}
		} else {
			for line in lines {
				if let Ok(ip) = line {
					regular(&ip);
				}
			}
		}
	} else {
		println!("{}", ":(");
	}
}

fn help_function(argument: &str) {
	colodot::colodot!(dot::DotTypes::Bold, dot::DotColors::Red, "Fegeya ",     false);
	colodot::colodot!(dot::DotTypes::Bold, dot::DotColors::Green, "RustoCat ", false);
	colodot::colodot!(dot::DotTypes::Bold, dot::DotColors::Blue, &VERSION,     false);
	colodot::colodot!(dot::DotTypes::Bold, dot::DotColors::Yellow,
		"\nColorized 'cat' implementation");

	colodot::colodot!(dot::DotTypes::Bold, dot::DotColors::LightMagenta,
		&format!("\n{} [file]\n", argument).to_string());

	dot::reset();
}

fn main() {
	let file: Vec<_> = std::env::args().collect();

	// languages as 8byte integer type
	let language: i8;

	if file.len() < 2 {
		help_function(&file[0]);
		std::process::exit(0);
	}

	print_top_header(10);

	if file[1].contains(".cpp") || file[1].contains(".hpp") || file[1].contains(".cc") || file[1].contains(".hh") {
		header_text(&file[1], "C++");
		language = 0;

	} else if file[1].contains(".fls") || file[1].contains(".flsh") {
		header_text(&file[1], "FlaScript");

		language = 1;
	} else if file[1].contains(".py") || file[1].contains(".pyw") || file[1].contains(".pyc") {
		header_text(&file[1], "Python");

		language = 2;
	} else {
		header_text(&file[1], "Regular");

		language = -1;
	}

	print_bottom_header(10);

	read_file(&file[1], language);

}
