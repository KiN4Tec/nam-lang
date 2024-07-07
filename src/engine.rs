use crate::{
	ast::{ASTNode, ASTNodeKind, BinaryOpKind},
	errors::EvaluationError,
	runtime::RuntimeVal,
};
use std::collections::HashMap;

pub struct Engine {
	variables: HashMap<String, RuntimeVal>,
}

impl Engine {
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

	pub fn evaluate(&mut self, ast: ASTNode) -> Result<RuntimeVal, EvaluationError> {
		match ast.kind {
			ASTNodeKind::Number(n) => {
				let res = RuntimeVal::Number(n);

				if ast.store_in_ans {
					self.assign_var("ans".to_string(), res.clone());
					if ast.print_result {
						println!("\nans = {n}");
					}
				}

				Ok(res)
			},

			ASTNodeKind::Matrix(m) => {
				let mut res_mat = vec![];
				for i in m {
					let mut row = vec![];
					for j in i {
						row.push(self.evaluate(j)?);
					}
					res_mat.push(row);
				}

				let res = RuntimeVal::Matrix(res_mat);
				if ast.store_in_ans {
					self.assign_var("ans".to_string(), res.clone());
					if ast.print_result {
						println!("\nans = {res}");
					}
				}

				Ok(res)
			},

			ASTNodeKind::Variable(var_name) => match self.get_var(&var_name) {
				Some(var_value) => {
					if ast.print_result {
						println!("\n{var_name} = {var_value}");
					}

					Ok(var_value.clone())
				},
				None => Err(EvaluationError::NonexistantVar(var_name)),
			},

			ASTNodeKind::Assignment(var_name, var_value) => {
				let res = self.evaluate(*var_value)?;
				self.assign_var(var_name.clone(), res.clone());

				if ast.print_result {
					println!("\n{var_name} = {res}");
				}

				Ok(res)
			},

			ASTNodeKind::BinaryExpr(op, lhs, rhs) => {
				let res_lhs: f64 = match self.evaluate(*lhs)? {
					RuntimeVal::Number(var_value) => var_value,
					_ => return Err(EvaluationError::NotANumber),
				};

				let res_rhs: f64 = match self.evaluate(*rhs)? {
					RuntimeVal::Number(var_value) => var_value,
					_ => return Err(EvaluationError::NotANumber),
				};

				let res = match op {
					BinaryOpKind::Add => RuntimeVal::Number(res_lhs + res_rhs),
					BinaryOpKind::Subtract => RuntimeVal::Number(res_lhs - res_rhs),
					BinaryOpKind::Multiply => RuntimeVal::Number(res_lhs * res_rhs),
					BinaryOpKind::Divide => RuntimeVal::Number(res_lhs / res_rhs),
				};

				if ast.store_in_ans {
					self.assign_var("ans".to_string(), res.clone());
					if ast.print_result {
						println!("\nans = {res}");
					}
				}

				Ok(res)
			},
		}
	}
}
