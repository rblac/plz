use std::{error::Error, fmt::Display};

use crate::{token::{Token, TokenType}, expressions::Expr, error::error};

#[derive(Debug)]
struct ParseError;
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("Parser error")
    }
}
impl Error for ParseError {}

pub struct Parser {
	tokens: Vec<Token>,
	current: usize,
}
#[allow(unused)]
impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser { tokens, current: 0 }
	}
	pub fn parse(&mut self) -> Option<Expr> {
		self.expression().ok()
	}

	fn error(&mut self, message: &str) -> ParseError {
		error(self.peek(), message.to_string());
		ParseError{}
	}
	fn synchronise(&mut self) {
		use TokenType::*;

		self.advance();
		while !self.is_at_end() { 
			if self.previous().kind == SEMICOLON { return; }

			match self.peek().kind {
				CONST | VAR | PROCEDURE | IF | WHILE => return,
				_ => ()
			}

			self.advance();
		}
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
	fn condition(&mut self) -> Result<Expr, ParseError> {
		use TokenType::*;
		if self.matches(&[ODD]) {
			Ok(Expr::Unary(
				self.previous(),
				Box::new(self.expression()?)
			))
		} else {
			self.equality()
		}
	}
	fn equality(&mut self) -> Result<Expr, ParseError> {
		let left = self.expression()?;
		use TokenType::*;
		if self.matches(&[BANG_EQU, EQU_EQU, LESS, LESS_EQU, MORE, MORE_EQU]) {
			let operator = self.previous();
			let right = self.expression()?;
			return Ok(Expr::Binary(Box::new(left), operator, Box::new(right)))
		}
		Err(self.error("Invalid comparison operator"))
	}
	fn expression(&mut self) -> Result<Expr, ParseError> {
		use TokenType::*;

		let prefix =
			if self.matches(&[MINUS, PLUS]) { Some(self.previous()) }
			else { None };

		let mut expr = self.factor()?;
		if let Some(p) = prefix {
			expr = Expr::Unary(p, Box::new(expr));
		}

		while self.matches(&[MINUS, PLUS]) {
			let operator = self.previous();
			let right = self.factor()?;
			expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
		}
		Ok(expr)
	}
	fn factor(&mut self) -> Result<Expr, ParseError> {
		let mut expr = self.primary()?;
		use TokenType::*;
		while self.matches(&[STAR, SLASH]) {
			let operator = self.previous();
			let right = self.primary()?;
			expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
		}
		Ok(expr)
	}
	fn primary(&mut self) -> Result<Expr, ParseError> {
		use TokenType::*;
		if self.matches(&[IDENTIFIER, NUMBER]) {
			Ok(Expr::Literal(self.previous()))
		}
		else if self.matches(&[LEFT_PAREN]) {
			let expr = self.expression()?;
			if !self.matches(&[RIGHT_PAREN]) {
				return Err(self.error("Missing ')' after expression"))
			}
			Ok(Expr::Grouping(Box::new(expr)))
		}
		else {
			Err(self.error("Expected an expression"))
		}
	}
}
