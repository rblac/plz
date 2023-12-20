#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TokenType {
	LEFT_PAREN, RIGHT_PAREN,
	SEMICOLON, COMMA, DOT,
	PLUS, MINUS, STAR, SLASH,

	BANG, // ! - exclamation mark
	QMARK, // ? - question mark

	BANG_EQU,
	COLON_EQU,
	EQU, EQU_EQU,
	MORE, MORE_EQU,
	LESS, LESS_EQU,

	IDENTIFIER, NUMBER,

	VAR, BEGIN, END,
	WHILE, DO,
	IF, THEN,
	PROCEDURE, CALL,
	ODD,

	EOF
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
	Number(i32),
	Identifier(String),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
	pub kind: TokenType,
	pub lexeme: String,
	pub literal: Option<Literal>,
	pub line: usize,
}
impl Token {
	pub fn new(kind: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
		Token { kind, lexeme, literal, line }
	}
}

