#[derive(Debug, Clone)]
pub struct TokenizationError {
	kind: TokenizationErrorKind,
	token_str: Option<String>,
	message: Option<String>,
}

impl TokenizationError {
	pub fn new(
		kind: TokenizationErrorKind,
		token_str: Option<String>,
		message: Option<String>,
	) -> TokenizationError {
		TokenizationError {
			kind,
			token_str,
			message,
		}
	}
}

#[derive(Debug, Clone)]
pub enum TokenizationErrorKind {
	EmptyString,
	NotANumber,
	UnexpectedChar(char),
	UnspportedSyntax(String),
}

impl std::error::Error for TokenizationError {}
impl std::fmt::Display for TokenizationError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use TokenizationErrorKind::*;

		let mut err_message = match &self.kind {
			EmptyString => String::from("Unexpected empty string"),
			NotANumber => String::from("Could not parse as number"),
			UnexpectedChar(c) => format!("Unexpected character '{}'", c.escape_default()),
			UnspportedSyntax(s) => format!("Unsupported syntax '{}'", s.escape_default()),
		};

		if let Some(token) = &self.token_str {
			err_message = format!(
				"{err_message}\nError found in string '{}'",
				token.escape_default()
			);
		}

		if let Some(message) = &self.message {
			err_message = format!("{err_message}\n{message}");
		}

		write!(f, "{err_message}")
	}
}

#[derive(Debug, Clone)]
pub enum ParsingError {
	UnexpectedEndOfInput,
	UnexpectedToken {
		expected: Option<String>,
		found: Option<String>,
	},
}

impl std::error::Error for ParsingError {}
impl std::fmt::Display for ParsingError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::UnexpectedEndOfInput => write!(f, "Unexpected end of input tokens array"),

			Self::UnexpectedToken { expected, found } => {
				let mut res = String::from("Unexpected token");
				if let Some(expected) = expected {
					res = format!("{res}, expected '{expected}'");
				}
				if let Some(found) = found {
					res = format!("{res}, found '{found}'");
				}
				write!(f, "{res}")
			},
		}
	}
}

#[derive(Debug, Clone)]
pub enum EvaluationError {
	NonexistantVar(String),
	NestedMatrices,
	InconsistantMatrixWidth(usize, usize),
	DimensionsMismatch((usize, usize), (usize, usize)),
	NoninvertibleDivisorMatrix,
}

impl std::error::Error for EvaluationError {}
impl std::fmt::Display for EvaluationError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NonexistantVar(var_name) => write!(f, "Variable {var_name} does not exist"),
			Self::NoninvertibleDivisorMatrix => {
				write!(f, "Can't divide by a non-invertible matrix")
			},

			EvaluationError::NestedMatrices => write!(
				f,
				"Matrices with more than two dimensions are not supported, yet!"
			),

			Self::InconsistantMatrixWidth(i, j) => {
				write!(f, "Inconsistant Matrix Width ({i} vs {j})")
			},

			EvaluationError::DimensionsMismatch(
				(lhs_width, lhs_height),
				(rhs_width, rhs_height),
			) => write!(
				f,
				"Dimentions Mismatch ({lhs_width}x{lhs_height} vs {rhs_width}x{rhs_height})"
			),
		}
	}
}
