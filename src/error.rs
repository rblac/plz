use std::{fmt::Display, error::Error};

use crate::token::{Token, TokenType};

static mut FAILED: bool = false;
fn report(line: usize, location: String, message: String) {
	eprintln!("Error @ line {line}, {location}: {message}");
}
pub fn error(token: Token, message: String) {
	unsafe { FAILED = true; }
	if token.kind == TokenType::EOF {
		report(token.line, "at the end".to_string(), message)
	} else {
		report(token.line, format!("at `{}`", token.lexeme), message)
	}
}
pub fn had_error() -> bool { unsafe { FAILED } }

#[derive(Debug)]
pub struct ParseError;
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("Parser error")
    }
}
impl Error for ParseError {}

#[derive(Debug)]
pub struct RuntimeError { pub msg: String }
impl Display for RuntimeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&format!("Runtime error: {}", self.msg))
	}
}
impl Error for RuntimeError {}
