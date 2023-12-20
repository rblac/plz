mod error;
mod token;
mod scanner;
mod expressions;
use scanner::Scanner;

fn main() {
	let file = std::fs::read_to_string(
			std::env::args().nth(1)
			.expect("gib sors")
		).expect("error while reading source");
	let mut scanner = Scanner::new(file);

	let _tokens = scanner.scan_tokens();
}
