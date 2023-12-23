use crate::{token::{Literal, TokenType}, expressions::Expr};

#[allow(unused)]
pub enum RuntimeValue {
	Value(i32),
	Ident(String),
	Boolean(bool),
	None,
}

#[allow(unused)]
impl RuntimeValue {
	fn from_literal(literal: &Literal) -> Self {
		match literal {
			Literal::Number(v) => Self::Value(*v),
			Literal::Identifier(i) => Self::Ident(i.clone()),
		}
	}
	fn as_value(&self) -> i32 {
		match self {
			Self::Value(v) => *v,
			Self::Ident(_) => todo!("resolving ident values"),
			_ => panic!("Value is not a number"),
		}
	}
	fn as_bool(&self) -> bool {
		match self {
			Self::Boolean(b) => *b,
			_ => panic!("Value is not a boolean"),
		}
	}
}

pub trait Interpretable {
	fn interpret(&self) -> RuntimeValue;
}

impl Interpretable for Expr {
    fn interpret(&self) -> RuntimeValue {
		match self {
			Expr::Literal(l) =>
				RuntimeValue::from_literal(&l.literal.clone().unwrap()),
			Expr::Grouping(e) =>
				e.interpret(),
			Expr::Unary(op, e) => {
				let v = e.interpret();
				use TokenType::*;
				use RuntimeValue::*;
				match op.kind {
					MINUS => Value(-v.as_value()),
					ODD => Boolean(v.as_value() % 2 == 1),
					BANG => {
						println!("> {}", v.as_value());
						None
					},
					_ => todo!("Unimplemented unary operator: {:?}", op),
				}
			},
			Expr::Binary(a, op, b) => {
				let va = a.interpret();
				let vb = b.interpret();
				use TokenType::*;
				use RuntimeValue::*;
				match op.kind {
					PLUS  => Value(va.as_value() + vb.as_value()),
					MINUS => Value(va.as_value() - vb.as_value()),
					STAR  => Value(va.as_value() * vb.as_value()),
					SLASH => Value(va.as_value() / vb.as_value()),

					EQU_EQU  => Boolean(va.as_value() == vb.as_value()),
					BANG_EQU => Boolean(va.as_value() != vb.as_value()),
					LESS => Boolean(va.as_value() < vb.as_value()),
					MORE => Boolean(va.as_value() > vb.as_value()),
					LESS_EQU => Boolean(va.as_value() <= vb.as_value()),
					MORE_EQU => Boolean(va.as_value() >= vb.as_value()),

					_ => todo!("Unimplemented binary operator: {:?}", op),
				}
			},
		}
    }
}
