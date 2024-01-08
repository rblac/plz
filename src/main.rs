mod error;
mod token;
mod scanner;
mod expressions;
mod parser;
mod interpreter;

use crate::error::had_error;
use interpreter::Interpretable;
use scanner::Scanner;
use parser::Parser;

fn main() {
	let data = std::fs::read_to_string(
			std::env::args().nth(1)
			.expect("gib sors")
		).expect("error while reading source");

	let mut scanner = Scanner::new(data);
	let tokens = scanner.scan_tokens();
	let mut parser = Parser::new(tokens);
	let ast = parser.parse();

	if had_error() || ast.is_none() {
		eprintln!("Parsing failed, exiting");
		std::process::exit(64);
	}

	let ast = ast.unwrap();
	ast.evaluate();
}
