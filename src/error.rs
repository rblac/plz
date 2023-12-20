use crate::token::{Token, TokenType};

fn report(line: usize, location: String, message: String) {
	eprintln!("Error @ line {line}, {location}: {message}");
}
pub fn error(token: Token, message: String) {
	if token.kind == TokenType::EOF {
		report(token.line, "at the end".to_string(), message)
	} else {
		report(token.line, format!("at `{}`", token.lexeme), message)
	}
}
