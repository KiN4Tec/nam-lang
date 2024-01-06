use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum RuntimeVal {
	Number(f64),
	Matrix(Vec<Vec<Self>>),
}

impl std::fmt::Display for RuntimeVal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Number(n) => write!(f, "{n}"),

			Self::Matrix(s) => {
				let mut buffer = String::new();
				buffer.push('[');
				for i in s {
					buffer.push_str("\n   ");
					for j in i {
						buffer.push_str("  ");
						buffer.push_str(j.to_string().as_str());
					}
				}
				buffer.push_str("\n]");
				write!(f, "{buffer}")
			},
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
