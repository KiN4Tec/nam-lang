use crate::ast::{ASTNode, ASTNodeKind, BinaryOpKind};
use crate::state::{RuntimeVal, State};

pub fn evaluate(ast: ASTNode, state: &mut State) -> Result<RuntimeVal, EvaluationError> {
	match ast.kind {
		ASTNodeKind::Number(n) => {
			let res = RuntimeVal::Number(n);

			if ast.store_in_ans {
				state.assign_var("ans".to_string(), res.clone());
				if ast.print_result {
					println!("\nans = {n}");
				}
			}

			Ok(res)
		},

		ASTNodeKind::Variable(var_name) => match state.get_var(&var_name) {
			Some(var_value) => {
				if ast.print_result {
					println!("\n{var_name} = {var_value}");
				}

				Ok(var_value.clone())
			},
			None => Err(EvaluationError::NonexistantVar(var_name)),
		},

		ASTNodeKind::Assignment(var_name, var_value) => {
			let res = evaluate(*var_value, state)?;
			state.assign_var(var_name.clone(), res.clone());

			if ast.print_result {
				println!("\n{var_name} = {res}");
			}

			Ok(res)
		},

		ASTNodeKind::BinaryExpr { lhs, op, rhs } => {
			let res_lhs: f64 = match evaluate(*lhs, state)? {
				RuntimeVal::Number(var_value) => var_value,
				_ => return Err(EvaluationError::NotANumber),
			};

			let res_rhs: f64 = match evaluate(*rhs, state)? {
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
				state.assign_var("ans".to_string(), res.clone());
				if ast.print_result {
					println!("\nans = {res}");
				}
			}

			Ok(res)
		},
	}
}

//////////////////////////
//    Error Handling    //
//////////////////////////

#[derive(Debug)]
pub enum EvaluationError {
	NonexistantVar(String),
	NotANumber,
}

impl std::error::Error for EvaluationError {}
impl std::fmt::Display for EvaluationError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NonexistantVar(var_name) => write!(f, "Variable {var_name} does not exist"),
			Self::NotANumber => write!(f, "Some value was used as a number while it is not"),
		}
	}
}
