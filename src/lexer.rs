use color_eyre::eyre::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	OpAdd,
	OpSubtract,
	OpMultiply,
	OpDivide,
	OpAssign,

	OpenParen,
	CloseParen,

	NumericLiteral(f64),
	Identifier(String),

	SemiColon,
	EndOfLine,
	EndOfFile,
}

impl Token {
	pub fn stringify(&self) -> String {
		let res = match self {
			Self::OpAdd => "OpAdd",
			Self::OpSubtract => "OpSubstract",
			Self::OpMultiply => "OpMultiply",
			Self::OpDivide => "OpDivide",
			Self::OpAssign => "OpAssign",

			Self::OpenParen => "OpenParen",
			Self::CloseParen => "CloseParen",

			Self::NumericLiteral(number) => {
				if number.is_nan() {
					"NumericLiteral"
				} else {
					return format!("NumericLiteral: {number}");
				}
			},

			Self::Identifier(name) => {
				if name.is_empty() {
					"Identifier"
				} else {
					return format!("Identifier: {name}");
				}
			},

			Self::SemiColon => "SemiColon",
			Self::EndOfLine => "EndOfLine",
			Self::EndOfFile => "EndOfFile",
		};

		res.to_string()
	}
}

impl TryFrom<String> for Token {
	type Error = TokenizationError;

	fn try_from(input: String) -> std::result::Result<Self, Self::Error> {
		let first = match input.chars().next() {
			Some(c) => c,
			None => {
				return Err(TokenizationError {
					kind: TokenizationErrorKind::EmptyString,
					token_str: Some(input),
					message: None,
				})
			},
		};

		match first {
			'+' => Ok(Self::OpAdd),
			'-' => Ok(Self::OpSubtract),
			'*' => Ok(Self::OpMultiply),
			'/' => Ok(Self::OpDivide),
			'=' => Ok(Self::OpAssign),

			'(' => Ok(Self::OpenParen),
			')' => Ok(Self::CloseParen),

			'0'..='9' => match input.parse() {
				Ok(r) => Ok(Self::NumericLiteral(r)),
				Err(e) => Err(TokenizationError {
					kind: TokenizationErrorKind::NotANumber,
					token_str: Some(input),
					message: Some(e.to_string()),
				}),
			},

			'A'..='Z' | 'a'..='z' | '_' => {
				for c in input.chars() {
					if !c.is_ascii_alphanumeric() && c != '_' {
						return Err(TokenizationError {
							kind: TokenizationErrorKind::UnexpectedChar(c),
							token_str: Some(input),
							message: None,
						});
					}
				}
				Ok(Self::Identifier(input))
			},

			';' => Ok(Self::SemiColon),

			first => Err(TokenizationError {
				kind: TokenizationErrorKind::UnexpectedChar(first),
				token_str: Some(input),
				message: None,
			}),
		}
	}
}

#[allow(dead_code)]
pub fn tokenize(code: &[char]) -> Vec<Token> {
	match try_tokenize(code) {
		Ok(tokens) => tokens,
		Err(e) => panic!("{e:?}"),
	}
}

pub fn try_tokenize(code: &[char]) -> Result<Vec<Token>, TokenizationError> {
	let mut res = vec![];
	let mut idx = 0;

	while let Some(&first) = code.get(idx) {
		match first {
			'+' | '-' | '*' | '/' | '(' | ')' | '=' | ';' => {
				res.push(Token::try_from(first.to_string())?);
				idx += 1;
			},

			'0'..='9' => {
				let mut token = String::new();
				let mut is_frac = false;
				let mut is_expo = false;

				while let Some(&next) = code.get(idx) {
					match next {
						'0'..='9' => token.push(next),

						'.' => {
							token.push('.');
							if is_expo {
								return Err(TokenizationError {
                                    kind: TokenizationErrorKind::UnexpectedChar('.'),
                                    token_str: Some(token),
                                    message: Some(String::from(
                                        "Could not parse a numeric literal with a dot after the 'e' in a scientific notation.",
                                    )),
                                });
							}
							if is_frac {
								return Err(TokenizationError {
									kind: TokenizationErrorKind::UnexpectedChar('.'),
									token_str: Some(token),
									message: Some(String::from(
										"Could not parse a numeric literal with more than one dot.",
									)),
								});
							}
							is_frac = true;
						},

						'e' => {
							token.push('e');

							if is_expo {
								return Err(TokenizationError {
                                    kind: TokenizationErrorKind::UnexpectedChar('e'),
                                    token_str: Some(token),
                                    message: Some(String::from(
                                        "Could not parse a numeric literal with more than one 'e' suffix,\n(Invalid scientific notation :)",
                                    )),
                                });
							}
							is_expo = true;
							is_frac = true;
							idx += 1;

							while code.get(idx) == Some(&'_') {
								idx += 1;
							}

							match code.get(idx) {
								Some('+') => token.push('+'),
								Some('-') => token.push('-'),
								Some(&n) if n.is_ascii_digit() => token.push(n),
								None | Some(_) => {
									return Err(TokenizationError {
										kind: TokenizationErrorKind::UnexpectedChar('e'),
										token_str: Some(token),
										message: Some(String::from(
											"The scientific notation is not complete.",
										)),
									})
								},
							}
						},

						'A'..='Z' | 'a'..='z' => {
							token.push(next);
							return Err(TokenizationError {
								kind: TokenizationErrorKind::UnspportedSyntax(next.to_string()),
								token_str: Some(token),
								message: Some(String::from(
									"Suffixes other than 'e' are not supported.",
								)),
							});
						},

						'_' => {},
						_ => break,
					}
					idx += 1;
				}

				let res_num = match token.parse() {
					Ok(r) => Token::NumericLiteral(r),
					Err(e) => {
						return Err(TokenizationError {
							kind: TokenizationErrorKind::NotANumber,
							token_str: Some(token),
							message: Some(e.to_string()),
						});
					},
				};

				res.push(res_num);
			},

			'A'..='Z' | 'a'..='z' | '_' => {
				let mut token = String::new();
				while let Some(&next) = code.get(idx) {
					if !next.is_ascii_alphanumeric() && next != '_' {
						break;
					}
					token.push(next);
					idx += 1;
				}
				res.push(Token::try_from(token)?);
			},

			'\n' => {
				idx += 1;
				res.push(Token::EndOfLine);
			},

			'\r' => {
				idx += 1;
				if code.get(idx) == Some(&'\n') {
					idx += 1;
				}
				res.push(Token::EndOfLine);
			},

			_ if first.is_whitespace() => idx += 1,
			c => {
				return Err(TokenizationError {
					kind: TokenizationErrorKind::UnexpectedChar(c),
					token_str: None,
					message: None,
				})
			},
		}
	}

	res.push(Token::EndOfFile);
	Ok(res)
}

////////////////////////////////
//       Error Handling       //
////////////////////////////////

#[derive(Debug)]
pub struct TokenizationError {
	kind: TokenizationErrorKind,
	token_str: Option<String>,
	message: Option<String>,
}

#[derive(Debug)]
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
