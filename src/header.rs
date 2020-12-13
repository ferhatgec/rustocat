// MIT License
//
// Copyright (c) 2020 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
// 

#[path = "colors.rs"] mod colors;

use colors::colorized::*;

pub fn print_top_header(len: u32) {
	print!("{}", &WBOLD_YELLOW_COLOR);

	// print top-left corner
	print!("  ╭");
	
	let mut i = 0;
	
	while i != len {
		print!("───");
    	i += 1;
	}
	
	// print top-right corner
	print!("╮\n {}{}", " │ ",
		&WRESET); 
}

pub fn print_bottom_header(len: u32) {
	print!("{}", &WBOLD_LIGHT_CYAN_COLOR);

	// print bottom-left corner
	print!("  ╰");
	
	let mut i = 0;
	
	while i != len {
		print!("───");
    	i += 1;
	}
	
	// print right corner
	print!("╯\n{}", 
		&WRESET); 
}
