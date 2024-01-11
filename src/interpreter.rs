use std::{fmt::Display, error::Error};

use crate::{token::{Literal, TokenType}, parser::Stmt, expressions::Expr};

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

pub trait Evaluable {
	fn evaluate(&self) -> Option<RuntimeValue>;
}
impl Evaluable for Expr {
	fn evaluate(&self) -> Option<RuntimeValue> {
		match self {
			Expr::Literal(l) =>
				Some(RuntimeValue::from_literal(&l.literal.clone().unwrap())),
			Expr::Grouping(e) => e.evaluate(),
			Expr::Unary(op, e) => {
				let v = e.evaluate()?.as_value()?;
				use TokenType::*;
				use RuntimeValue::*;
				match op.kind {
					MINUS => Some(Value(-v)),
					ODD => Some(Boolean(v % 2 == 1)),
					_ => None,
				}
			},
			Expr::Binary(a, op, b) => {
				let va = a.evaluate()?.as_value()?;
				let vb = b.evaluate()?.as_value()?;
				use TokenType::*;
				use RuntimeValue::*;
				match op.kind {
					PLUS  => Some(Value(va + vb)),
					MINUS => Some(Value(va - vb)),
					STAR  => Some(Value(va * vb)),
					SLASH => Some(Value(va / vb)),
					EQU_EQU  => Some(Boolean(va == vb)),
					BANG_EQU => Some(Boolean(va != vb)),
					LESS_EQU => Some(Boolean(va <= vb)),
					MORE_EQU => Some(Boolean(va >= vb)),
					LESS => Some(Boolean(va < vb)),
					MORE => Some(Boolean(va > vb)),
					_ => None,
				}
			},
		}
	}
}

#[derive(Debug)]
pub struct RuntimeError { msg: String }
impl Display for RuntimeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&format!("Runtime error: {}", self.msg))
	}
}
impl Error for RuntimeError {}

pub struct Interpreter;

impl Interpreter {
	pub fn new() -> Self {
		Interpreter{}
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
	fn execute(&mut self, s: Stmt) -> Result<(), RuntimeError> {
		match s {
			Stmt::Print(e) => {
				if let Some(v) = e.evaluate() {
					if let Some(val) = v.as_value() {
						println!("> {val}");
						return Ok(())
					} else {
						Err(Self::error("Expected to find value")) // me irl amirite
					}
				}
				else {
					return Err(Self::error(&format!("Failed to evaluate: {e}")));
				}
			},
			_ => todo!("non-print statements"),
		}
	}
}
