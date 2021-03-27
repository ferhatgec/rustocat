// MIT License
//
// Copyright (c) 2020 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

// TODO: Use colodot instead.
#[path = "colors.rs"]
pub(self) mod colors;

use colors::colorized::*;

// For unsupported languages, documents
pub fn regular(line: &str) {
	let mut data = line.replace("int", format!("{}int{}", &WBOLD_RED_COLOR, &WRESET).as_str());

	data = data.replace("char", format!("{}char{}", &WBOLD_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("const", format!("{}const{}", &WBOLD_LIGHT_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("void", format!("{}void{}", &WBOLD_BLUE_COLOR, &WRESET).as_str());

	println!("{}", data);
}

pub fn cplusplus(line: &str) {
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

pub fn flascript(line: &str) {
	let mut data = line.replace("var", format!("{}var{}", &WBOLD_BLUE_COLOR, &WRESET).as_str());

	data = data.replace("bool", format!("{}bool{}", &WBOLD_LIGHT_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("char", format!("{}char{}", &WBOLD_LIGHT_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("string", format!("{}string{}", &WBOLD_LIGHT_BLUE_COLOR, &WRESET).as_str());

	data = data.replace("if", format!("{}if{}", &WBOLD_LIGHT_RED_COLOR, &WRESET).as_str());
	data = data.replace("else", format!("{}else{}", &WBOLD_LIGHT_RED_COLOR, &WRESET).as_str());

	data = data.replace("for", format!("{}for{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("do", format!("{}do{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("while", format!("{}while{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());

	data = data.replace("func", format!("{}func{}", &WBOLD_RED_COLOR, &WRESET).as_str());
	data = data.replace("main", format!("{}main{}", &WBOLD_LIGHT_RED_COLOR, &WRESET).as_str());

	data = data.replace("@append", format!("{}@append{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("@pop_back", format!("{}@pop_back{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("@between", format!("{}@between{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());

	data = data.replace("import", format!("{}import{}", &WBOLD_YELLOW_COLOR, &WRESET).as_str());

	data = data.replace("defin", format!("{}defin{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("put", format!("{}put{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());

	data = data.replace("#ifdef", format!("{}#ifdef{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("#endif", format!("{}#endif{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());

	data = data.replace("return", format!("{}return{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());

	data = data.replace("SystemInfo", format!("{}SystemInfo{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("Colorized", format!("{}Colorized{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());

	data = data.replace("newline", format!("{}newline{}", &WBOLD_LIGHT_BLACK_COLOR, &WRESET).as_str());

	data = data.replace("print", format!("{}print{}", &WBOLD_CYAN_COLOR, &WRESET).as_str());
	data = data.replace("fprintf", format!("{}fprintf{}", &WBOLD_CYAN_COLOR, &WRESET).as_str());
	data = data.replace("@echo", format!("{}@echo{}", &WBOLD_CYAN_COLOR, &WRESET).as_str());
	data = data.replace("fprintln", format!("{}fprintln{}", &WBOLD_CYAN_COLOR, &WRESET).as_str());

	data = data.replace("::", format!("{}::{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("{", format!("{}{{{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("}", format!("{}}}{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("(", format!("{}({}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace(")", format!("{}){}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("*", format!("{}*{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("+", format!("{}+{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("<", format!("{}<{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace(">", format!("{}>{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());

	println!("{}", data);
}

pub fn python(line: &str) {
	let mut data = line.replace("if", format!("{}if{}", &WBOLD_LIGHT_RED_COLOR, &WRESET).as_str());

	data = data.replace("else", format!("{}else{}", &WBOLD_LIGHT_RED_COLOR, &WRESET).as_str());
	data = data.replace("elif", format!("{}elif{}", &WBOLD_LIGHT_RED_COLOR, &WRESET).as_str());

	data = data.replace("for", format!("{}for{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());
	data = data.replace("while", format!("{}while{}", &WBOLD_MAGENTA_COLOR, &WRESET).as_str());

	data = data.replace("str", format!("{}str{}", &WBOLD_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("bool", format!("{}bool{}", &WBOLD_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("float", format!("{}float{}", &WBOLD_BLUE_COLOR, &WRESET).as_str());

	data = data.replace("def", format!("{}def{}", &WBOLD_RED_COLOR, &WRESET).as_str());
	data = data.replace("lambda", format!("{}lambda{}", &WBOLD_RED_COLOR, &WRESET).as_str());

	data = data.replace("import", format!("{}import{}", &WBOLD_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("from", format!("{}from{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());

	data = data.replace("global", format!("{}global{}", &WBOLD_LIGHT_BLUE_COLOR, &WRESET).as_str());
	data = data.replace("return", format!("{}return{}", &WBOLD_LIGHT_MAGENTA_COLOR, &WRESET).as_str());

	data = data.replace("print", format!("{}print{}", &WBOLD_CYAN_COLOR, &WRESET).as_str());

	data = data.replace("::", format!("{}::{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("{", format!("{}{{{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("}", format!("{}}}{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("(", format!("{}({}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace(")", format!("{}){}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("*", format!("{}*{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("+", format!("{}+{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace("<", format!("{}<{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());
	data = data.replace(">", format!("{}>{}", &WBOLD_LIGHT_YELLOW_COLOR, &WRESET).as_str());

	println!("{}", data);
}
