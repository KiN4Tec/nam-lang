use crate::ast::{ASTNode, BinaryOpKind};
use crate::state::{State, RuntimeVal};

pub fn evaluate(ast: ASTNode, state: &mut State) -> Result<RuntimeVal, EvaluationError> {
    match ast {
        ASTNode::Number(n) => Ok(RuntimeVal::Number(n)),

        ASTNode::Variable(var_name) => match state.get_var(&var_name) {
            Some(var_value) => Ok(var_value.clone()),
            None => Err(EvaluationError::NonexistantVar(var_name)),
        },

        ASTNode::Assignment(var_name, var_value) => {
            let res = evaluate(*var_value, state)?;
            state.assign_var(var_name, res.clone());
            Ok(res)
        },

        ASTNode::BinaryExpr { lhs, op, rhs } => {
            let res_lhs: f64 = match evaluate(*lhs, state)? {
                RuntimeVal::Number(var_value) => var_value,
                _ => return Err(EvaluationError::NotANumber),
            };

            let res_rhs: f64 = match evaluate(*rhs, state)? {
                RuntimeVal::Number(var_value) => var_value,
                _ => return Err(EvaluationError::NotANumber),
            };

            match op {
                BinaryOpKind::Add => Ok(RuntimeVal::Number(res_lhs + res_rhs)),
                BinaryOpKind::Subtract => Ok(RuntimeVal::Number(res_lhs - res_rhs)),
                BinaryOpKind::Multiply => Ok(RuntimeVal::Number(res_lhs * res_rhs)),
                BinaryOpKind::Divide => Ok(RuntimeVal::Number(res_lhs / res_rhs)),
            }
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
