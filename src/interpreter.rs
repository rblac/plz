use crate::{token::{Literal, TokenType}, parser::Stmt, expressions::Expr, error::RuntimeError, environment::Environment};

#[allow(unused)]
pub enum RuntimeValue {
	Value(i32),
	Ident(String),
	Boolean(bool),
}
#[allow(unused)]
impl RuntimeValue {
	fn from_literal(literal: &Literal) -> Self {
		match literal {
			Literal::Number(v) => Self::Value(*v),
			Literal::Identifier(i) => Self::Ident(i.clone()),
		}
	}
	fn as_value(&self) -> Option<i32> {
		match self {
			Self::Value(v) => Some(*v),
			_ => None,
		}
	}
	fn as_ident(&self) -> Option<String> {
		match self {
			Self::Ident(s) => Some(s.clone()),
			_ => None,
		}
	}
	fn as_bool(&self) -> Option<bool> {
		match self {
			Self::Boolean(b) => Some(*b),
			_ => None,
		}
	}
}


pub struct Interpreter {
	env: Environment,
}

impl Interpreter {
	pub fn new() -> Self {
		Interpreter {
			env: Environment::new(),
		}
	}
	pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
		for s in statements {
			self.execute(s)?
		}
		Ok(())
	}
	fn error(msg: &str) -> RuntimeError {
		RuntimeError { msg: msg.to_string() }
	}

	fn evaluate(&mut self, expr: Expr) -> Result<RuntimeValue, RuntimeError> {
		match expr {
			Expr::Literal(l) =>
				Ok(RuntimeValue::from_literal(&l.literal.clone().unwrap())),
			Expr::Grouping(e) => self.evaluate(*e),
			Expr::Unary(op, e) => {
				let v = self.evaluate(*e)?.as_value().ok_or(Self::error("not a value"))?;
				use TokenType::*;
				use RuntimeValue::*;
				match op.kind {
					MINUS => Ok(Value(-v)),
					ODD => Ok(Boolean(v % 2 == 1)),
					_ => Err(Self::error(&format!("Invalid unary operator: {}", op.lexeme))),
				}
			},
			Expr::Binary(a, op, b) => {
				let va = self.evaluate(*a)?.as_value().ok_or(Self::error("not a value"))?;
				let vb = self.evaluate(*b)?.as_value().ok_or(Self::error("not a value"))?;
				use TokenType::*;
				use RuntimeValue::*;
				match op.kind {
					PLUS  => Ok(Value(va + vb)),
					MINUS => Ok(Value(va - vb)),
					STAR  => Ok(Value(va * vb)),
					SLASH => Ok(Value(va / vb)),
					EQU_EQU  => Ok(Boolean(va == vb)),
					BANG_EQU => Ok(Boolean(va != vb)),
					LESS_EQU => Ok(Boolean(va <= vb)),
					MORE_EQU => Ok(Boolean(va >= vb)),
					LESS => Ok(Boolean(va < vb)),
					MORE => Ok(Boolean(va > vb)),
					_ => Err(Self::error(&format!("Invalid binary operator: {}", op.lexeme))),
				}
			},
			Expr::Variable(name) => {
				if let Some(v) = self.env.get(name.clone())? {
					Ok(RuntimeValue::Value(v))
				} else {
					Err(Self::error(&format!("Use of unitialised variable: {}", name.lexeme)))
				}
			},
		}
	}
	fn execute(&mut self, s: Stmt) -> Result<(), RuntimeError> {
		match s {
			Stmt::Print(e) => {
				let v = self.evaluate(e)?;
				if let Some(val) = v.as_value() {
					println!("> {val}");
					return Ok(())
				} else {
					Err(Self::error("Expected to find value")) // me irl amirite
				}
			},
			Stmt::PrintVar(name) => {
				let val = self.env.get(name.clone())?;
				let val = match val { Some(i) => i.to_string(), None => "unassigned".to_string() };
				println!("> {}: {val}", name.lexeme);
				Ok(())
			},
			Stmt::Var(names) => {
				for name in names {
					self.env.declare(name)?;
				}
				Ok(())
			},
			Stmt::Expression(e) => {
				match self.evaluate(e) {
					Ok(_) => Ok(()),
					Err(e) => Err(e)
				}
			},
			Stmt::Scope(statements) => {
				for s in statements {
					self.execute(s)?;
				}
				Ok(())
			}
			Stmt::Assign(name, e) => {
				let val = self.evaluate(e)?.as_value().ok_or(Self::error("not a value"))?;
				self.env.assign(name, Some(val))?;
				Ok(())
			},
		}
	}
}
