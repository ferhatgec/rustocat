/* MIT License
#
# Copyright (c) 2020 Ferhat Geçdoğan All Rights Reserved.
# Distributed under the terms of the MIT License.
#
# */

pub const WBOLD_RED_COLOR:          &str = "\x1b[1;31m";
pub const WBOLD_GREEN_COLOR:        &str = "\x1b[1;32m";
pub const WBOLD_YELLOW_COLOR:       &str = "\x1b[1;33m";
pub const WBOLD_BLUE_COLOR:         &str = "\x1b[1;34m";
pub const WBOLD_MAGENTA_COLOR:      &str = "\x1b[1;35m";
pub const WBOLD_CYAN_COLOR:         &str = "\x1b[1;36m";
pub const WBOLD_LIGHT_BLACK_COLOR:  &str = "\x1b[1;90m";
pub const WBOLD_LIGHT_RED_COLOR:    &str = "\x1b[1;91m";

#[allow(dead_code)]
pub const WBOLD_LIGHT_GREEN_COLOR:  &str = "\x1b[1;92m";
pub const WBOLD_LIGHT_YELLOW_COLOR: &str = "\x1b[1;93m";
pub const WBOLD_LIGHT_BLUE_COLOR:   &str = "\x1b[1;94m";
pub const WBOLD_LIGHT_MAGENTA_COLOR:&str = "\x1b[1;95m";

#[allow(dead_code)]
pub const WBOLD_LIGHT_CYAN_COLOR:   &str = "\x1b[1;96m";

#[allow(dead_code)]
pub const WBOLD_LIGHT_WHITE_COLOR:  &str = "\x1b[1;97m";

pub const WRESET: &str = "\x1b[0m";

/* For unsupported languages, documents */
pub fn Regular(line: &str) {
	let mut data = line.replace("int", format!("{}int{}", &WBOLD_RED_COLOR, &WRESET).as_str());
	
	data = data.replace("char", format!("{}char{}", &WBOLD_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("const", format!("{}const{}", &WBOLD_LIGHT_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("void", format!("{}void{}", &WBOLD_BLUE_COLOR, &WRESET).as_str());

	println!("{}", data);	
}

/* For C++ */
pub fn CPlusPlus(line: &str) {
	let mut data = line.replace("int", format!("{}int{}", &WBOLD_BLUE_COLOR, &WRESET).as_str());

	data = data.replace("long", format!("{}long{}", &WBOLD_BLUE_COLOR, &WRESET).as_str());
	
	data = data.replace("bool", format!("{}bool{}", &WBOLD_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("char", format!("{}char{}", &WBOLD_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("auto", format!("{}auto{}", &WBOLD_BLUE_COLOR, &WRESET).as_str());
	
	data = data.replace("if", format!("{}if{}", &WBOLD_LIGHT_RED_COLOR, &WRESET).as_str());
	data = data.replace("else", format!("{}else{}", &WBOLD_LIGHT_RED_COLOR, &WRESET).as_str());
	
	data = data.replace("for", format!("{}for{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("do", format!("{}do{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("while", format!("{}while{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());
	
	data = data.replace("void", format!("{}void{}", &WBOLD_RED_COLOR, &WRESET).as_str());
	data = data.replace("main", format!("{}main{}", &WBOLD_LIGHT_RED_COLOR, &WRESET).as_str());
	data = data.replace("asm", format!("{}asm{}", &WBOLD_LIGHT_RED_COLOR, &WRESET).as_str());
	
	data = data.replace("const", format!("{}const{}", &WBOLD_LIGHT_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("static", format!("{}static{}", &WBOLD_LIGHT_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("extern", format!("{}extern{}", &WBOLD_LIGHT_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("inline", format!("{}inline{}", &WBOLD_LIGHT_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("virtual", format!("{}virtual{}", &WBOLD_LIGHT_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("friend", format!("{}friend{}", &WBOLD_LIGHT_BLUE_COLOR, &WRESET).as_str());
	
	data = data.replace("public", format!("{}public{}", &WBOLD_LIGHT_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("private", format!("{}private{}", &WBOLD_LIGHT_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("protected", format!("{}protected{}", &WBOLD_LIGHT_BLUE_COLOR, &WRESET).as_str());
	
	data = data.replace("#include", format!("{}#include{}", &WBOLD_YELLOW_COLOR, &WRESET).as_str());
	
	data = data.replace("typedef", format!("{}typedef{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());
	
	data = data.replace("#define", format!("{}#define{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("#ifndef", format!("{}#ifndef{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("#ifdef", format!("{}#ifdef{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("#endif", format!("{}#endif{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());
	
	data = data.replace("return", format!("{}return{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	
	data = data.replace("nodiscard", format!("{}nodiscard{}", &WBOLD_LIGHT_BLACK_COLOR, &WRESET).as_str());
	
	data = data.replace("class", format!("{}class{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("struct", format!("{}struct{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("namespace", format!("{}namespace{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());
	
	data = data.replace("using", format!("{}using{}", &WBOLD_GREEN_COLOR, &WRESET).as_str());
	
	data = data.replace("std", format!("{}std{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	
	data = data.replace("iostream", format!("{}iostream{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("cstring", format!("{}cstring{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("sstream", format!("{}sstream{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("fstream", format!("{}fstream{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("memory", format!("{}memory{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("cstdlib", format!("{}cstdlib{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("cstdio", format!("{}cstdio{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("vector", format!("{}vector{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("algorithm", format!("{}algorithm{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("thread", format!("{}thread{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("array", format!("{}array{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("bitset", format!("{}bitset{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("deque", format!("{}deque{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("map", format!("{}map{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("stack", format!("{}stack{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("set", format!("{}set{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("iterator", format!("{}iterator{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("tuple", format!("{}tuple{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("locale", format!("{}locale{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("regex", format!("{}regex{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("istream", format!("{}istream{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("ostream", format!("{}ostream{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	
	data = data.replace("cout", format!("{}cout{}", &WBOLD_CYAN_COLOR, &WRESET).as_str());
	data = data.replace("printf", format!("{}printf{}", &WBOLD_CYAN_COLOR, &WRESET).as_str());
	data = data.replace("getline", format!("{}getline{}", &WBOLD_CYAN_COLOR, &WRESET).as_str());
	data = data.replace("cin", format!("{}cin{}", &WBOLD_CYAN_COLOR, &WRESET).as_str());
	
	data = data.replace("::", format!("{}::{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	
	data = data.replace("{", format!("{}{{{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("}", format!("{}}}{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	
	data = data.replace("(", format!("{}({}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace(")", format!("{}){}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("*", format!("{}*{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("<", format!("{}<{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace(">", format!("{}>{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("+", format!("{}+{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	
	println!("{}", data);
}
