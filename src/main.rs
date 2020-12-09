/* MIT License
#
# Copyright (c) 2020 Ferhat Geçdoğan All Rights Reserved.
# Distributed under the terms of the MIT License.
#
# */

#[path = "languages.rs"]
#[allow(non_snake_case)]
mod languages;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const VERSION: &str = "0.1-beta-1";

#[allow(non_snake_case)]
fn Read<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(non_snake_case)]
fn ReadFile(filename: &str) {
	if let Ok(lines) = Read(&filename) {
		println!("{}", &filename);
		if filename.contains(".cpp") {
			for line in lines {
				if let Ok(ip) = line {
					languages::CPlusPlus(&ip);
				}	
			}
		} else {
			for line in lines {
				if let Ok(ip) = line {
					println!("{}", ip);
				}	
			}
		}
	}		
}

#[allow(non_snake_case)]
fn HelpFunction(argument: &str) {
	println!("{}\n{}\n{}", 
		format!("{}Fegeya {}Rustocat {}{}", 
			languages::WBOLD_RED_COLOR, 
			languages::WBOLD_GREEN_COLOR,
			languages::WBOLD_BLUE_COLOR,
			&VERSION).as_str(),
		format!("{} Colorized 'cat' implementation.", &languages::WBOLD_YELLOW_COLOR).as_str(),
		format!("{}{} [file]{}", 
			languages::WBOLD_LIGHT_MAGENTA_COLOR, 
			argument,
			languages::WRESET).as_str());

}

fn main() {
	let file: Vec<_> = std::env::args().collect();
	
	if file.len() < 2 {
		HelpFunction(&file[0]);
		std::process::exit(0);
	}
	
	ReadFile(&file[1]);
}
