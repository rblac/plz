use crate::error::ParseError;
use crate::{token::{Token, TokenType}, expressions::Expr, error::error};

#[derive(Clone)]
pub enum Stmt {
	Proc(Token, Box<Vec<Stmt>>),
	Const(Vec<(Token, Token)>),
	Var(Vec<Token>),

	Print(Expr),
	PrintVar(Token),
	Expression(Expr),
	Scope(Vec<Stmt>), // couldn't call it a 'block' because of the EBNF's naming convention
	Assign(Token, Expr),
	If(Expr, Box<Stmt>),
	While(Expr, Box<Stmt>),
	Call(Token),
}

pub struct Parser {
	tokens: Vec<Token>,
	current: usize,
}
impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Parser { tokens, current: 0 }
	}
	pub fn parse(&mut self) -> Vec<Stmt> {
		let mut out = Vec::new();
		while !self.is_at_end() {
			out.append(&mut self.block())
		}
		out
	}

	fn error(&mut self, message: &str) -> ParseError {
		error(self.peek(), message.to_string());
		ParseError{}
	}
	fn synchronise(&mut self) {
		use TokenType::*;

		self.advance();
		while !self.is_at_end() { 
			match self.peek().kind {
				CONST | VAR | PROCEDURE | BEGIN => return,
				_ => {}
			}
			self.advance();
		}
	}

	// token parsing functions
	fn peek(&self) -> Token{ self.tokens[self.current].clone() }
	fn is_at_end(&self) -> bool { self.peek().kind == TokenType::DOT }
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
	fn consume(&mut self, kind: TokenType, err_msg: &str) -> Result<Token, ParseError> {
		if self.matches(&[kind]) { Ok(self.previous()) }
		else { Err(self.error(err_msg)) }
	}

	// recursive descent functions
	fn block(&mut self) -> Vec<Stmt> {
		use TokenType::*;
		let mut out = Vec::new();

		if self.matches(&[CONST]) {
			let consts = self.const_declaration();
			if let Some(c) = consts.ok() { out.push(c); }
		}
		if self.matches(&[VAR]) {
			let vars = self.var_declaration();
			if let Some(v) = vars.ok() { out.push(v); }
		}
		while self.matches(&[PROCEDURE]) {
			let proc = self.proc_declaration();
			if let Some(p) = proc.ok() { out.push(p); }
		}
		
		let stmt = self.statement();
		if stmt.is_err() { self.synchronise(); }
		else { out.push(stmt.unwrap()); }

		out
	}

	fn statement(&mut self) -> Result<Stmt, ParseError> {
		use TokenType::*;
		if self.matches(&[BEGIN]) {
			return self.scope();
		}
		if self.matches(&[BANG]) {
			return Ok(Stmt::Print(self.expression()?));
		}
		if self.matches(&[QMARK]) {
			if self.matches(&[IDENTIFIER]) {
				return Ok(Stmt::PrintVar(self.previous()));
			}
			return Err(self.error("Expected identifier for `?` expression"));
		}
		if self.matches(&[IF]) {
			return self.if_statement();
		}
		if self.matches(&[CALL]) {
			if self.matches(&[IDENTIFIER]) {
				return Ok(Stmt::Call(self.previous()));
			}
			return Err(self.error("Expected procedure identifier for CALL expression"));
		}
		if self.matches(&[WHILE]) {
			return self.while_statement();
		}
		if self.matches(&[CALL]) {
			todo!("statement types");
		}
		return Ok(self.assignment_or_expr()?);
	}
	fn const_declaration(&mut self) -> Result<Stmt, ParseError> {
		use TokenType::*;
		let mut consts: Vec<(Token, Token)> = Vec::new();
		loop {
			let name = self.consume(IDENTIFIER, "Expected const name")?;
			self.consume(EQU, &format!("Expected `=` after const name: {}", name.lexeme))?;
			let value = self.consume(NUMBER, "Expected const value")?;
			consts.push((name, value));
			if !self.matches(&[COMMA]) { break }
		}
		self.consume(SEMICOLON, "Expected `;` after const declaration")?;
		Ok(Stmt::Const(consts))
	}
	fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
		use TokenType::*;
		let mut names = Vec::new();
		names.push(self.consume(IDENTIFIER, "Expected var name")?);
		while self.matches(&[COMMA]) {
			names.push(self.consume(IDENTIFIER, "Expected var name after comma")?);
		}
		self.consume(SEMICOLON, "Expected `;` after var declaration")?;
		Ok(Stmt::Var(names))
	}
	fn proc_declaration(&mut self) -> Result<Stmt, ParseError> {
		use TokenType::*;
		let name = self.consume(IDENTIFIER, "Expected procedure identifier")?;
		self.consume(SEMICOLON, "Expected `;` after procedure identifier")?;
		let block = self.block();
		self.consume(SEMICOLON, "Expected `;` after procedure block")?;
		Ok(Stmt::Proc(name, Box::new(block)))
	}

	fn scope(&mut self) -> Result<Stmt, ParseError> {
		use TokenType::*;
		let mut statements = Vec::new();
		statements.push(self.statement()?);
		while !self.is_at_end() {
			if !self.matches(&[SEMICOLON]) {
				self.consume(END, "Expected END token")?;
				break
			}
			if self.peek().kind == END {
				self.advance();
				break
			}
			statements.push(self.statement()?);
		}
		Ok(Stmt::Scope(statements))
	}
	fn if_statement(&mut self) -> Result<Stmt, ParseError> {
		use TokenType::*;
		let cond = self.condition()?;
		self.consume(THEN, "Expected THEN token after IF condition")?;
		let stmt = self.statement()?;
		Ok(Stmt::If(cond, Box::new(stmt)))
	}
	fn while_statement(&mut self) -> Result<Stmt, ParseError> {
		use TokenType::*;
		let cond = self.condition()?;
		self.consume(DO, "Expected DO token after WHILE condition")?;
		let stmt = self.statement()?;
		Ok(Stmt::While(cond, Box::new(stmt)))
	}
	fn assignment_or_expr(&mut self) -> Result<Stmt, ParseError> {
		use TokenType::*;
		let expr = self.expression()?;

		if self.matches(&[COLON_EQU]) {
			let value = self.expression()?;
			match expr {
				Expr::Variable(name) => Ok(Stmt::Assign(name, value)),
				_ => {
					// Report, but don't throw Err -- no need to synchronise.
					self.error(&format!("Invalid lvalue: {expr}"));
					// return lvalue as placeholder
					Ok(Stmt::Expression(expr))
				}
			}
		} else { Ok(Stmt::Expression(expr)) }
	}

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
		if self.matches(&[NUMBER]) {
			Ok(Expr::Literal(self.previous()))
		}
		else if self.matches(&[IDENTIFIER]) {
			Ok(Expr::Variable(self.previous()))
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
