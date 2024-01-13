use std::{rc::Rc, cell::RefCell};

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
	env: Rc<RefCell<Environment>>,
}

impl Interpreter {
	pub fn new() -> Self {
		Interpreter { env: Rc::new(RefCell::new(Environment::new())), }
	}
	pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
		for s in statements {
			self.execute(s, &mut self.env.clone())?
		}
		Ok(())
	}
	fn error(msg: &str) -> RuntimeError {
		RuntimeError { msg: msg.to_string() }
	}

	fn evaluate(&mut self, expr: Expr, env: &Environment) -> Result<RuntimeValue, RuntimeError> {
		match expr {
			Expr::Literal(l) =>
				Ok(RuntimeValue::from_literal(&l.literal.clone().unwrap())),
			Expr::Grouping(e) => self.evaluate(*e, env),
			Expr::Unary(op, e) => {
				let v = self.evaluate(*e, env)?.as_value().ok_or(Self::error("not a value"))?;
				use TokenType::*;
				use RuntimeValue::*;
				match op.kind {
					MINUS => Ok(Value(-v)),
					ODD => Ok(Boolean(v % 2 == 1)),
					_ => Err(Self::error(&format!("Invalid unary operator: {}", op.lexeme))),
				}
			},
			Expr::Binary(a, op, b) => {
				let va = self.evaluate(*a, env)?.as_value().ok_or(Self::error("not a value"))?;
				let vb = self.evaluate(*b, env)?.as_value().ok_or(Self::error("not a value"))?;
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
				if let Some(v) = env.get_var(name.clone())? {
					Ok(RuntimeValue::Value(v))
				} else {
					Err(Self::error(&format!("Use of unitialised variable: {}", name.lexeme)))
				}
			},
		}
	}
	fn execute(&mut self, s: Stmt, env: &mut Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {
		match s {
			Stmt::Print(e) => {
				let v = self.evaluate(e, &env.borrow())?;
				if let Some(val) = v.as_value() {
					println!("> {val}");
					return Ok(())
				} else {
					Err(Self::error("Expected to find value")) // me irl amirite
				}
			},
			Stmt::PrintVar(name) => {
				let val = env.borrow().get_var(name.clone())?;
				let val = match val { Some(i) => i.to_string(), None => "unassigned".to_string() };
				println!("> {}: {val}", name.lexeme);
				Ok(())
			},
			Stmt::Var(names) => {
				for name in names {
					env.borrow_mut().declare_var(name)?;
				}
				Ok(())
			},
			Stmt::Expression(e) => {
				match self.evaluate(e, &env.borrow()) {
					Ok(_) => Ok(()),
					Err(e) => Err(e)
				}
			},
			Stmt::Scope(statements) => {
				for s in statements {
					self.execute(s, env)?;
				}
				Ok(())
			}
			Stmt::Assign(name, e) => {
				let val = self.evaluate(e, &env.borrow())?
					.as_value().ok_or(Self::error("not a value"))?;
				env.borrow_mut().assign_var(name, Some(val))?;
				Ok(())
			},
			Stmt::If(condition, then_branch) => {
				if self.evaluate(condition, &env.borrow_mut())?
					.as_bool().ok_or(Self::error("not a boolean"))? {
					self.execute(*then_branch, env)?;
				}
				Ok(())
			},
			Stmt::While(condition, branch) => {
				while self.evaluate(condition.clone(), &env.borrow_mut())?
					.as_bool().ok_or(Self::error("not a boolean"))? {
					self.execute(*branch.clone(), env)?;
				}
				Ok(())
			},
			Stmt::Proc(name, body) => {
				env.borrow_mut().define_proc(name, *body)?;
				Ok(())
			},
			Stmt::Call(name) => {
				let proc = env.borrow_mut().get_proc(name)?;
				let call_env = Environment::from_parent(env.clone());
				self.execute(Stmt::Scope(proc), &mut Rc::new(RefCell::new(call_env)))
			},
		}
	}
}
