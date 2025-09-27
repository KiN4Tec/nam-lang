use crate::errors::EvaluationError;
use crate::matrix::Matrix;
use crate::scalar::Scalar;

#[derive(Debug, Clone)]
pub enum RuntimeVal {
	Variable(String),
	Scalar(Scalar),
	Matrix(Matrix),
}

impl RuntimeVal {
	pub fn try_add(self, rhs: Self) -> Result<Self, EvaluationError> {
		match (self, rhs) {
			(Self::Scalar(lhs), Self::Scalar(rhs)) => Ok(Self::Scalar(lhs + rhs)),

			(Self::Matrix(lhs), Self::Matrix(rhs)) => {
				if lhs.get_shape() != rhs.get_shape() {
					return Err(EvaluationError::DimensionsMismatch(
						lhs.get_shape(),
						rhs.get_shape(),
					));
				}
				Ok(RuntimeVal::Matrix(lhs + rhs))
			},

			(Self::Matrix(mat), Self::Scalar(num)) | (Self::Scalar(num), Self::Matrix(mat)) => {
				Ok(RuntimeVal::Matrix(mat + num))
			},

			(Self::Variable(_), _) | (_, Self::Variable(_)) => {
				unreachable!("Variables must be evaluated in the engine first")
			},
		}
	}

	pub fn try_sub(self, rhs: Self) -> Result<Self, EvaluationError> {
		match (self, rhs) {
			(Self::Scalar(lhs), Self::Scalar(rhs)) => Ok(Self::Scalar(lhs - rhs)),

			(Self::Matrix(lhs), Self::Matrix(rhs)) => {
				if lhs.get_shape() != rhs.get_shape() {
					return Err(EvaluationError::DimensionsMismatch(
						lhs.get_shape(),
						rhs.get_shape(),
					));
				}
				Ok(Self::Matrix(lhs - rhs))
			},

			(Self::Matrix(mat), Self::Scalar(num)) => Ok(RuntimeVal::Matrix(mat - num)),
			(Self::Scalar(num), Self::Matrix(mat)) => Ok(RuntimeVal::Matrix(num - mat)),

			(Self::Variable(_), _) | (_, Self::Variable(_)) => {
				unreachable!("Variables must be evaluated in the engine first")
			},
		}
	}

	pub fn try_mul(self, rhs: Self) -> Result<Self, EvaluationError> {
		match (self, rhs) {
			(Self::Scalar(lhs), Self::Scalar(rhs)) => Ok(Self::Scalar(lhs * rhs)),

			(Self::Matrix(lhs), Self::Matrix(rhs)) => {
				if lhs.width() != rhs.height() {
					return Err(EvaluationError::DimensionsMismatch(
						lhs.get_shape(),
						rhs.get_shape(),
					));
				}
				Ok(RuntimeVal::Matrix(lhs * rhs))
			},

			(Self::Matrix(mat), Self::Scalar(num)) | (Self::Scalar(num), Self::Matrix(mat)) => {
				Ok(Self::Matrix(mat * num))
			},

			(Self::Variable(_), _) | (_, Self::Variable(_)) => {
				unreachable!("Variables must be evaluated in the engine first")
			},
		}
	}

	pub fn try_div(self, rhs: Self) -> Result<Self, EvaluationError> {
		match (self, rhs) {
			(Self::Scalar(lhs), Self::Scalar(rhs)) => Ok(Self::Scalar(lhs / rhs)),

			(Self::Matrix(lhs), Self::Matrix(rhs)) => {
				if !rhs.is_square() {
					return Err(EvaluationError::NoninvertibleDivisorMatrix);
				}

				if lhs.get_shape() != rhs.get_shape() {
					return Err(EvaluationError::DimensionsMismatch(
						lhs.get_shape(),
						rhs.get_shape(),
					));
				}

				if let Some(inv_rhs) = rhs.try_invert() {
					Ok(Self::Matrix(lhs * inv_rhs))
				} else {
					Err(EvaluationError::NoninvertibleDivisorMatrix)
				}
			},

			(Self::Matrix(mat), Self::Scalar(num)) => Ok(Self::Matrix(mat / num)),
			(Self::Scalar(num), Self::Matrix(mat)) => Ok(Self::Matrix(num / mat)),

			(Self::Variable(_), _) | (_, Self::Variable(_)) => {
				unreachable!("Variables must be evaluated in the engine first")
			},
		}
	}
}

impl std::fmt::Display for RuntimeVal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Scalar(n) => write!(f, "{n}"),
			Self::Matrix(mat) => write!(f, "{mat}"),
			Self::Variable(name) => write!(f, "{name}"),
		}
	}
}
