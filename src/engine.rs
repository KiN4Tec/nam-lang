use crate::{
	ast::{ASTNode, ASTNodeKind, BinaryOpKind},
	errors::EvaluationError,
	runtime::RuntimeVal,
};
use nalgebra::{dmatrix, DMatrix, RowDVector};
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

			ASTNodeKind::Matrix(mat) => {
				let res = if mat.is_empty() {
					RuntimeVal::Matrix(dmatrix![])
				} else {
					let width = mat[0].len();
					let height = mat.len();
					let mut res_mat = Vec::with_capacity(mat.len());

					for row in mat {
						if width != row.len() {
							return Err(EvaluationError::InconsistantMatrixWidth(width, row.len()));
						}

						let mut res_row = Vec::with_capacity(width);
						for cell in row {
							match self.evaluate(cell)? {
								RuntimeVal::Number(res_cell) => res_row.push(res_cell),
								RuntimeVal::Matrix(_) => {
									return Err(EvaluationError::NestedMatrices)
								},
							}
						}
						res_mat.push(RowDVector::from_row_slice(res_row.as_slice()));
					}

					if width == 1 && height == 1 {
						RuntimeVal::Number(res_mat[0][0])
					} else {
						RuntimeVal::Matrix(DMatrix::from_rows(&res_mat))
					}
				};

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
				let res_lhs = self.evaluate(*lhs)?;
				let res_rhs = self.evaluate(*rhs)?;

				let res = match op {
					BinaryOpKind::Add => res_lhs.try_add(res_rhs)?,
					BinaryOpKind::Subtract => res_lhs.try_sub(res_rhs)?,
					BinaryOpKind::Multiply => res_lhs.try_mul(res_rhs)?,
					BinaryOpKind::Divide => res_lhs.try_div(res_rhs)?,
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
