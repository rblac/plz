use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::{token::Token, error::RuntimeError, parser::Stmt};

#[derive(Default)]
pub struct Environment {
	parent: Option<Rc<RefCell<Environment>>>,
	values: HashMap<String, Option<i32>>,
	consts: HashMap<String, i32>,
	procedures: HashMap<String, Vec<Stmt>>,
}
impl Environment {
	pub fn new() -> Self {
		Environment {
			..Default::default()
		}
	}
	pub fn from_parent(parent: Rc<RefCell<Environment>>) -> Self {
		Environment {
			parent: Some(parent),
			..Default::default()
		}
	}

	pub fn get_var(&self, name: Token) -> Result<Option<i32>, RuntimeError> {
		let lex = name.lexeme.clone();
		if let Some(v) = self.values.get(&lex) { Ok(*v) }
		else if let Some(c) = self.consts.get(&lex) { Ok(Some(*c)) }
		else {
			if self.parent.is_some() {
				return self.parent.as_ref().unwrap().borrow().get_var(name)
			}
			Err(RuntimeError{ msg: format!("Uninitialised variable: {}", lex) })
		}
	}
	pub fn declare_var(&mut self, name: Token) -> Result<(), RuntimeError> {
		if self.values.insert(name.lexeme.clone(), None).is_some() {
			Err(RuntimeError{msg: format!("Double declaration of name: {}", name.lexeme)})
		} else { Ok(()) }
	}
	pub fn assign_var(&mut self, name: Token, value: Option<i32>) -> Result<(), RuntimeError> {
		let lex = name.lexeme.clone();
		if self.values.insert(lex.clone(), value).is_none() {
			if self.parent.is_some() {
				return self.parent.as_ref().unwrap().borrow_mut().assign_var(name, value)
			}
			if self.consts.contains_key(&lex) {
				return Err(RuntimeError{msg: format!("Attempting to assign to a const: {}", lex)})
			}
			Err(RuntimeError{msg: format!("Assigning to undeclared variable: {}", lex)})
		} else { Ok(()) }
	}

	pub fn declare_const(&mut self, name: Token, value: i32) -> Result<(), RuntimeError> {
		let lex = name.lexeme.clone();
		if self.consts.insert(lex.clone(), value).is_some() {
			Err(RuntimeError{msg: format!("Double definition of const: {}", lex)})
		} else { Ok(()) }
	}

	pub fn get_proc(&self, name: Token) -> Result<Vec<Stmt>, RuntimeError> {
		if let Some(v) = self.procedures.get(&name.lexeme) { Ok(v.clone()) }
		else {
			if self.parent.is_some() {
				return self.parent.as_ref().unwrap().borrow_mut().get_proc(name)
			}
			Err(RuntimeError{msg: format!("Undefined procedure: {}", name.lexeme)})
		}
	}
	pub fn define_proc(&mut self, name: Token, def: Vec<Stmt>) -> Result<(), RuntimeError> {
		if self.procedures.insert(name.lexeme.clone(), def).is_some() {
			Err(RuntimeError{msg: format!("Double definition of procedure: {}", name.lexeme)})
		} else { Ok(()) }
	}
}
