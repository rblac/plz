use crate::{token::{Token, TokenType}, expressions::Expr};

pub struct Parser {
	tokens: Vec<Token>,
	current: usize,
}

#[allow(unused)]
impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser { tokens, current: 0 }
	}

	// token parsing functions
	fn peek(&self) -> Token{ self.tokens[self.current].clone() }
	fn is_at_end(&self) -> bool { self.peek().kind == TokenType::EOF }
	fn previous(&mut self) -> Token { self.tokens[self.current-1].clone() }
	fn advance(&mut self) -> Token {
		if !self.is_at_end() { self.current += 1; }
		self.previous()
	}
	fn check(&self, kind: TokenType) -> bool {
		if self.is_at_end() { false }
		else { self.peek().kind == kind }
	}
	fn matches(&mut self, kinds: &[TokenType]) -> bool {
		for kind in kinds {
			if self.check(*kind) {
				self.advance();
				return true
			}
		}
		false
	}

	// recursive descent functions
	fn expression(&mut self) -> Expr {
		use TokenType::*;
		if self.matches(&[ODD]) {
			Expr::Unary(
				self.previous(),
				Box::new(self.term())
			)
		} else {
			self.equality()
		}
	}
	fn equality(&mut self) -> Expr {
		let left = self.term();
		use TokenType::*;
		if self.matches(&[BANG_EQU, EQU_EQU, LESS, LESS_EQU, MORE, MORE_EQU]) {
			let operator = self.previous();
			let right = self.term();
			Expr::Binary(Box::new(left), operator, Box::new(right));
		}
		panic!("invalid equality operator: {}", self.previous().lexeme)
	}
	fn term(&mut self) -> Expr {
		use TokenType::*;

		let prefix =
			if self.matches(&[MINUS, PLUS]) { Some(self.previous()) }
			else { None };

		let mut expr = self.factor();
		if let Some(p) = prefix {
			expr = Expr::Unary(p, Box::new(expr));
		}

		while self.matches(&[MINUS, PLUS]) {
			let operator = self.previous();
			let right = self.factor();
			expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
		}
		expr
	}
	fn factor(&mut self) -> Expr {
		let mut expr = self.primary();
		use TokenType::*;
		while self.matches(&[STAR, SLASH]) {
			let operator = self.previous();
			let right = self.primary();
			expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
		}
		expr
	}
	fn primary(&mut self) -> Expr {
		use TokenType::*;
		if self.matches(&[IDENTIFIER, NUMBER]) {
			Expr::Literal(self.previous())
		}
		else if self.matches(&[LEFT_PAREN]) {
			let expr = self.expression();
			if !self.matches(&[RIGHT_PAREN]) {
				panic!("Missing ')' after expression: {expr}")
			}
			Expr::Grouping(Box::new(expr))
		}
		else {
			panic!("Invalid primary expression: {}", self.previous().lexeme)
		}
	}
}
