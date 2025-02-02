use crate::errors::EvaluationError;
use nalgebra::DMatrix;

#[derive(Debug, Clone)]
pub enum RuntimeVal {
	Number(f64),
	Matrix(DMatrix<f64>),
}

impl RuntimeVal {
	pub fn try_add(self, rhs: Self) -> Result<Self, EvaluationError> {
		match (self, rhs) {
			(Self::Number(lhs), Self::Number(rhs)) => Ok(Self::Number(lhs + rhs)),

			(Self::Matrix(lhs), Self::Matrix(rhs)) => {
				if lhs.ncols() != rhs.ncols() || lhs.nrows() != rhs.nrows() {
					return Err(EvaluationError::DimensionsMismatch(
						lhs.shape(),
						rhs.shape(),
					));
				}

				Ok(RuntimeVal::Matrix(lhs + rhs))
			},

			(Self::Matrix(mat), Self::Number(num)) | (Self::Number(num), Self::Matrix(mat)) => {
				Ok(RuntimeVal::Matrix(mat.add_scalar(num)))
			},
		}
	}

	pub fn try_sub(self, rhs: Self) -> Result<Self, EvaluationError> {
		match (self, rhs) {
			(Self::Number(lhs), Self::Number(rhs)) => Ok(Self::Number(lhs - rhs)),

			(Self::Matrix(lhs), Self::Matrix(rhs)) => {
				if lhs.ncols() != rhs.ncols() || lhs.nrows() != rhs.nrows() {
					return Err(EvaluationError::DimensionsMismatch(
						lhs.shape(),
						rhs.shape(),
					));
				}

				Ok(Self::Matrix(lhs - rhs))
			},

			(Self::Matrix(mat), Self::Number(num)) => Ok(RuntimeVal::Matrix(mat.add_scalar(-num))),

			(Self::Number(num), Self::Matrix(mut mat)) => {
				mat.neg_mut();
				Ok(RuntimeVal::Matrix(mat.add_scalar(num)))
			},
		}
	}

	pub fn try_mul(self, rhs: Self) -> Result<Self, EvaluationError> {
		match (self, rhs) {
			(Self::Number(lhs), Self::Number(rhs)) => Ok(Self::Number(lhs * rhs)),

			(Self::Matrix(lhs), Self::Matrix(rhs)) => {
				if lhs.ncols() != rhs.nrows() {
					Err(EvaluationError::DimensionsMismatch(
						lhs.shape(),
						rhs.shape(),
					))
				} else {
					Ok(RuntimeVal::Matrix(lhs * rhs))
				}
			},

			(Self::Matrix(mut mat), Self::Number(num))
			| (Self::Number(num), Self::Matrix(mut mat)) => {
				for e in mat.iter_mut() {
					*e *= num;
				}
				Ok(Self::Matrix(mat))
			},
		}
	}

	pub fn try_div(self, rhs: Self) -> Result<Self, EvaluationError> {
		match (self, rhs) {
			(Self::Number(lhs), Self::Number(rhs)) => Ok(Self::Number(lhs / rhs)),

			(Self::Matrix(lhs), Self::Matrix(rhs)) => {
				if !rhs.is_square() {
					return Err(EvaluationError::NoninvertibleDivisorMatrix);
				}

				if lhs.shape() != rhs.shape() {
					return Err(EvaluationError::DimensionsMismatch(lhs.shape(), rhs.shape()));
				}

				if let Some(inv_rhs) = rhs.try_inverse() {
					Ok(Self::Matrix(lhs * inv_rhs))
				} else {
					Err(EvaluationError::NoninvertibleDivisorMatrix)
				}
			},

			(Self::Matrix(mut mat), Self::Number(num)) => {
				for e in mat.iter_mut() {
					*e /= num;
				}
				Ok(Self::Matrix(mat))
			},

			(Self::Number(_num), Self::Matrix(mat)) => {
				Err(EvaluationError::DimensionsMismatch((1, 1), mat.shape()))
			},
		}
	}
}

impl std::fmt::Display for RuntimeVal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Number(n) => write!(f, "{n}"),
			Self::Matrix(mat) => write!(f, "{mat}"),
		}
	}
}
