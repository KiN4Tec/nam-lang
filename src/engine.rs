use crate::{
	ast::{ASTNode, ASTNodeValue, Operator},
	errors::EvaluationError,
	matrix::Matrix,
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

	pub fn get_var(&mut self, var_name: &String) -> Option<RuntimeVal> {
		self.variables.get(var_name).cloned()
	}

	pub fn evaluate(&mut self, ast: ASTNode) -> Result<RuntimeVal, EvaluationError> {
		match ast.value {
			ASTNodeValue::Number(n) => {
				let res = RuntimeVal::Scalar(n);

				if ast.store_in_ans {
					self.assign_var("ans".to_string(), res.clone());
					if ast.print_result {
						println!("\nans = {n}");
					}
				}

				Ok(res)
			},

			ASTNodeValue::Matrix(mat) => {
				let res = if mat.is_empty() {
					RuntimeVal::Matrix(Matrix::new())
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
								RuntimeVal::Matrix(_) => {
									return Err(EvaluationError::NestedMatrices);
								},

								RuntimeVal::Scalar(res_cell) => res_row.push(res_cell),
								RuntimeVal::Variable(name) => {
									let val = match self.get_var(&name) {
										Some(RuntimeVal::Scalar(v)) => v,
										Some(RuntimeVal::Matrix(_)) => {
											return Err(EvaluationError::NestedMatrices);
										},
										None => return Err(EvaluationError::NonexistantVar(name)),
										Some(RuntimeVal::Variable(_)) => unreachable!(),
									};
									res_row.push(val);
								},
							}
						}
						res_mat.push(res_row);
					}

					if width == 1 && height == 1 {
						RuntimeVal::Scalar(res_mat[0][0])
					} else {
						RuntimeVal::Matrix(Matrix::try_from_rows(res_mat)?)
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

			ASTNodeValue::Variable(var_name) => match self.get_var(&var_name) {
				Some(var_value) => {
					if ast.print_result {
						println!("\n{var_name} = {var_value}");
					}
					Ok(var_value.clone())
				},
				None => Err(EvaluationError::NonexistantVar(var_name)),
			},

			ASTNodeValue::ArithmaticExpr(mut rpn_queue) => {
				let mut evaluation_stack: Vec<RuntimeVal> = vec![];
				while let Some(node) = rpn_queue.pop() {
					match node {
						ASTNodeValue::Operator(operator) => {
							let right = match evaluation_stack.pop() {
								Some(RuntimeVal::Variable(var_name)) => {
									match self.get_var(&var_name) {
										Some(x) => x,
										None => {
											return Err(EvaluationError::NonexistantVar(var_name));
										},
									}
								},
								Some(x) => x,
								None => return Err(EvaluationError::InvalidArithmaticExpression),
							};

							let left = match evaluation_stack.pop() {
								Some(RuntimeVal::Variable(var_name))
									if operator == Operator::Assign =>
								{
									self.assign_var(var_name.clone(), right.clone());
									evaluation_stack.push(RuntimeVal::Variable(var_name));
									continue;
								},

								Some(RuntimeVal::Variable(var_name)) => {
									match self.get_var(&var_name) {
										Some(x) => x,
										None => {
											return Err(EvaluationError::NonexistantVar(var_name));
										},
									}
								},

								Some(x) => x,
								None => return Err(EvaluationError::InvalidArithmaticExpression),
							};

							match operator {
								Operator::Add => evaluation_stack.push(left.try_add(right)?),
								Operator::Subtract => evaluation_stack.push(left.try_sub(right)?),
								Operator::Multiply => evaluation_stack.push(left.try_mul(right)?),
								Operator::Divide => evaluation_stack.push(left.try_div(right)?),
								Operator::Assign => match left {
									RuntimeVal::Variable(_) => unreachable!(),
									_ => return Err(EvaluationError::AssignmentToNonVariable),
								},
							}
						},

						ASTNodeValue::Number(val) => evaluation_stack.push(RuntimeVal::Scalar(val)),

						ASTNodeValue::Variable(name) => {
							evaluation_stack.push(RuntimeVal::Variable(name))
						},

						ASTNodeValue::Matrix(_) => {
							evaluation_stack.push(self.evaluate(node.into())?)
						},

						ASTNodeValue::ArithmaticExpr(_) => unreachable!(
							"Arithmatic expression inside another arithmatic expression"
						),
					}
				}

				if evaluation_stack.len() != 1 {
					return Err(EvaluationError::InvalidArithmaticExpression);
				}

				let res = evaluation_stack.pop().unwrap();
				if let RuntimeVal::Variable(var_name) = &res {
					if ast.print_result {
						match self.get_var(var_name) {
							Some(res) => println!("\n{var_name} = {res}"),
							None => return Err(EvaluationError::NonexistantVar(var_name.clone())),
						}
					}
				} else if ast.store_in_ans {
					self.assign_var("ans".to_string(), res.clone());
					if ast.print_result {
						println!("\nans = {res}");
					}
				}

				Ok(res)
			},

			ASTNodeValue::Operator(_) => Err(EvaluationError::InvalidArithmaticExpression),
		}
	}
}
