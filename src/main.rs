// MIT License
//
// Copyright (c) 2020 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use std::fs::File;
use std::path::Path;

pub(self) mod header;
pub(self) mod languages;
pub(self) mod colors;

use std::io::{
	self, 
	BufRead
};

use languages::{
	cplusplus,
	flascript,
	regular
};

use header::{
	print_top_header,
	print_bottom_header,
	header_text
};

use colors::colorized::*;

const VERSION: &str = "0.1-beta-1";

fn read<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file(filename: &str, language: u8) {
	if let Ok(lines) = read(&filename) {
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
	println!("{}\n{}\n{}", 
		format!("{}Fegeya {}Rustocat {}{}", 
			WBOLD_RED_COLOR, 
			WBOLD_GREEN_COLOR,
			WBOLD_BLUE_COLOR,
			&VERSION).as_str(),
		format!("{} Colorized 'cat' implementation.", &WBOLD_YELLOW_COLOR).as_str(),
		format!("{}{} [file]{}", 
			WBOLD_LIGHT_MAGENTA_COLOR, 
			argument,
			WRESET).as_str());

}

fn main() {
	let file: Vec<_> = std::env::args().collect();
	let language: u8;
	 
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
	} else {
		header_text(&file[1], "Regular");
	
		language = 2;
	}
	
	print_bottom_header(10);
	
	read_file(&file[1], language);
	
}
