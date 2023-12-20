use crate::token::*;

pub struct Scanner {
	source: String,
	tokens: Vec<Token>,
	start: usize,
	current: usize,
	line: usize,
	failed: bool,
}
impl Scanner {
	pub fn new(source: String) -> Self {
		Scanner {
			source,
			tokens: vec![],
			start: 0, current: 0, line: 1,
			failed: false,
		}
	}

	fn error(&mut self, line: usize, msg: String) {
		eprintln!("{line}: {msg}");
		self.failed = true;
	}
	
	fn is_at_end(&self) -> bool {
		self.current >= self.source.len()
	}
	fn advance(&mut self) -> char {
		self.current += 1;
		self.source.as_bytes()[self.current-1] as char
	}

	fn add_token(&mut self, kind: TokenType) {
		self.add_token_full(None, kind)
	}
	fn add_token_full(&mut self, literal: Option<Literal>, kind: TokenType) {
		let lex = &self.source[self.start..self.current];
		self.tokens.push(Token::new(kind, lex.to_string(), literal, self.line))
	}

	fn matches(&mut self, expected: char) -> bool {
		if self.is_at_end() { return false }
		if self.source.as_bytes()[self.current] as char != expected { return false }

		self.current += 1;
		true
	}
	fn peek(&self) -> char {
		if self.is_at_end() { return '\0' }
		self.source.as_bytes()[self.current] as char
	}

	fn is_digit(c : char) -> bool { '0' <= c && c <= '9' }
	fn is_alpha(c : char) -> bool { ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || c == '_' }
	fn is_alphanumeric(c : char) -> bool { Self::is_alpha(c) || Self::is_digit(c) }
	fn keyword(s: &str) -> Option<TokenType> {
		use TokenType::*;
		match s {
			"var" => Some(VAR),
			"begin" => Some(BEGIN),
			"end" => Some(END),
			"while" => Some(WHILE),
			"do" => Some(DO),
			"if" => Some(IF),
			"then" => Some(THEN),
			"procedure" => Some(PROCEDURE),
			"call" => Some(CALL),
			"odd" => Some(ODD),
			_ => None
		}
	}

	fn number(&mut self) {
		while Self::is_digit(self.peek()) {
			self.advance();
		}

		let substr = &self.source.as_str()[self.start..self.current];
		let lit = substr.parse::<i32>();
		if lit.is_err() {
			self.error(self.line, format!("Failed to parse number literal `{substr}`: {}", lit.clone().unwrap_err()));
		}

		self.add_token_full(Some(Literal::Number(lit.unwrap_or(-1))), TokenType::NUMBER);
	}
	fn identifier(&mut self) {
		while Self::is_alphanumeric(self.peek()) && !self.is_at_end() { self.advance(); }

		let substr = &self.source.as_str()[self.start..self.current];
		if let Some(keyword) = Self::keyword(substr) {
			self.add_token(keyword);
		} else {
			self.add_token_full(Some(Literal::Identifier(substr.to_string())), TokenType::IDENTIFIER);
		}
	}

	fn scan_token(&mut self) {
		let c = self.advance();
		use TokenType::*;
		match c {
			'(' => self.add_token(LEFT_PAREN),
			')' => self.add_token(RIGHT_PAREN),
			',' => self.add_token(COMMA),
			';' => self.add_token(SEMICOLON),
			'.' => self.add_token(DOT),
			'+' => self.add_token(PLUS),
			'-' => self.add_token(MINUS),
			'*' => self.add_token(STAR),
			'/' => self.add_token(SLASH),
			'?' => self.add_token(QMARK),
			'#' => {
				while self.peek() != '\n' { self.advance(); }
			},
			'!' => {
				let t = if self.matches('=') { BANG_EQU } else { BANG };
				self.add_token(t);
			},
			':' => {
				if self.matches('=') { self.add_token(COLON_EQU); }
				else {
					self.error(self.line, "Invalid token; Expected `:=`".to_string());
				}
			},
			'=' => {
				let t = if self.matches('=') { EQU_EQU } else { EQU };
				self.add_token(t);
			},
			'>' => {
				let t = if self.matches('=') { MORE_EQU } else { MORE };
				self.add_token(t);
			},
			'<' => {
				let t = if self.matches('=') { LESS_EQU } else { LESS };
				self.add_token(t);
			},

			' ' | '\r' | '\t' => (),
			'\n' => self.line += 1,

			o => {
				if Self::is_digit(o) {
					self.number();
				} else if Self::is_alpha(o) {
					self.identifier();
				} else {
					self.error(self.line, format!("Unexpected character: {c}"));
				}
			},
		}
	}
	pub fn scan_tokens(&mut self) -> Vec<Token> {
		while !self.is_at_end() {
			self.start = self.current;
			self.scan_token();
		}
		self.tokens.push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
		self.tokens.clone()
	}
}
