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
	pub fn set(&mut self, name: Token, value: Option<i32>) {
		self.values.insert(name.lexeme, value);
	}
}
