use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum RuntimeVal {
	Number(f64),

	#[allow(dead_code)]
	String(String),
}

impl std::fmt::Display for RuntimeVal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Number(n) => write!(f, "{n}"),
			Self::String(s) => write!(f, "{s}"),
		}
	}
}

pub struct State {
	variables: HashMap<String, RuntimeVal>,
}

impl State {
	pub fn new() -> Self {
		Self {
			variables: HashMap::new(),
		}
	}

	pub fn assign_var(&mut self, var_name: String, var_value: RuntimeVal) -> Option<RuntimeVal> {
		self.variables.insert(var_name, var_value)
	}

	pub fn get_var(&mut self, var_name: &String) -> Option<&mut RuntimeVal> {
		self.variables.get_mut(var_name)
	}
}
