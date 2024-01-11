use std::fmt::Display;

use crate::token::Token;

pub enum Expr {
	Literal(Token),
	Grouping(Box<Expr>),
	Unary(Token, Box<Expr>),
	Binary(Box<Expr>, Token, Box<Expr>),

	Variable(Token),
	Assign(Token, Box<Expr>),
}
impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use Expr::*;
		match self {
			Literal(l) => f.write_str(&l.lexeme),
			Grouping(a) => f.write_str(&format!("(group {a})")),
			Unary(o, a) => f.write_str(&format!("({} {a})", o.lexeme)),
			Binary(a, o, b) => f.write_str(&format!("({} {a} {b})", o.lexeme)),
			Variable(name) => f.write_str(&format!("`{}`", name.lexeme)),
			Assign(name, value) => f.write_str(&format!("({} := {value})", name.lexeme)),
		}
    }
}
