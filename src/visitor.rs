use std::collections::HashMap;
use ast::{StatementBlock, TopNode};

pub trait Visitable {
	fn visit(&self, visitor: &mut Visitor);
}

pub enum Value {
	Text(String),
	Number(f64)
}

pub struct Visitor {
	variables: HashMap<String, Value>,
	functions: HashMap<String, (Vec<String>, StatementBlock)>
}

impl Visitor {
	pub fn new() -> Self {
		Visitor { variables: HashMap::new(), functions: HashMap::new() }
	}

	pub fn assign_function(&mut self, name: String, args: Vec<String>, body: StatementBlock) {
		self.functions.insert(name, (args, body));
	}

	pub fn assign_variable(&mut self, name: String, value: Value) {
		self.variables.insert(name, value);
	}
}
