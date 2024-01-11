use std::collections::HashMap;

use crate::{token::Token, error::RuntimeError};

pub struct Environment {
	values: HashMap<String, Option<i32>>,
}
impl Environment {
	pub fn new() -> Self {
		Environment { values: HashMap::new() }
	}
	pub fn get(&self, name: Token) -> Result<Option<i32>, RuntimeError> {
		if let Some(v) = self.values.get(&name.lexeme) { Ok(*v) }
		else {
			Err(RuntimeError{ msg: format!("Uninitialised variable: {}", name.lexeme) })
		}
	}
	pub fn declare(&mut self, name: Token) -> Result<(), RuntimeError> {
		if self.values.insert(name.lexeme.clone(), None).is_some() {
			Err(RuntimeError{msg: format!("Double declaration of name: {}", name.lexeme)})
		} else { Ok(()) }
	}
	pub fn assign(&mut self, name: Token, value: Option<i32>) -> Result<(), RuntimeError> {
		if self.values.insert(name.lexeme.clone(), value).is_none() {
			Err(RuntimeError{msg: format!("Assigning to undeclared variable: {}", name.lexeme)})
		} else { Ok(()) }
	}
}
