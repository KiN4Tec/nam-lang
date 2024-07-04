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
	EmptyMatrixElement,
	DimensionsMismatch(usize, usize),
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
			Self::EmptyMatrixElement => write!(f, "Empty matrix elements are not allowed"),
			Self::DimensionsMismatch(i, j) => write!(f, "Dimensions mismatch ({i} vs {j})"),

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
