mod error;
mod token;
mod scanner;
mod expressions;
mod parser;

use scanner::Scanner;
use parser::Parser;

fn main() {
	let data = std::fs::read_to_string(
			std::env::args().nth(1)
			.expect("gib sors")
		).expect("error while reading source");

	let mut scanner = Scanner::new(data);
	let tokens = scanner.scan_tokens();
	let parser = Parser::new(tokens);
}
