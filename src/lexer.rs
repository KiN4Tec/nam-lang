use color_eyre::eyre::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	OpAdd,      // +
	OpSubtract, // -
	OpMultiply, // *
	OpDivide,   // /
	OpAssign,   // =

	OpenParen,  // )
	CloseParen, // (
	OpenBrace,  // [
	CloseBrace, // ]
	OpenCurly,  // {
	CloseCurly, // }

	NumericLiteral(f64),
	Identifier(String),

	Comma,      // ,
	SemiColon,  // ;
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
			Self::OpenBrace => "OpenBrace",
			Self::CloseBrace => "CloseBrace",
			Self::OpenCurly => "OpenCurly",
			Self::CloseCurly => "CloseCurly",

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

			Self::Comma => "Comma",
			Self::SemiColon => "SemiColon",
			Self::EndOfLine => "EndOfLine",
			Self::EndOfFile => "EndOfFile",
		};

		res.to_string()
	}
}

impl std::str::FromStr for Token {
	type Err = TokenizationError;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let input = input.to_string();

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
			'[' => Ok(Self::OpenBrace),
			']' => Ok(Self::CloseBrace),
			'{' => Ok(Self::OpenCurly),
			'}' => Ok(Self::CloseCurly),

			'0'..='9' => match input.parse() {
				Ok(n) => Ok(Self::NumericLiteral(n)),
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

			',' => Ok(Self::Comma),
			';' => Ok(Self::SemiColon),

			first => Err(TokenizationError {
				kind: TokenizationErrorKind::UnexpectedChar(first),
				token_str: Some(input),
				message: None,
			}),
		}
	}
}

#[allow(unused)]
pub fn tokenize(idx: usize, code: &str) -> Vec<Token> {
	match try_tokenize(idx, code) {
		Ok(tokens) => tokens,
		Err(e) => panic!("{e:?}"),
	}
}

pub fn try_tokenize(mut idx: usize, code: &str) -> Result<Vec<Token>, TokenizationError> {
	let mut chars = code.chars().skip(idx).peekable();
	let mut res = vec![];

	/*
		Rust borrow checker dows not allow us to use these closures
		because they borrow idx and chars untill the last call of these closures

		let mut advance = |i: usize| {
			idx += i;
			chars.nth(i - 1);
		};
		let mut advance_once = || advance(1);
	*/

	while let Some(first) = chars.peek() {
		match first {
			'+' | '-' | '*' | '/' | '(' | ')' | '[' | ']' | '{' | '}' | '=' | ',' | ';' => {
				idx += 1;
				res.push(chars.next().unwrap().to_string().parse()?);
			},

			'0'..='9' => {
				let (token_len, token) = try_tokenize_number(idx, code)?;
res.push(token);

				idx += token_len;
				chars.nth(token_len - 1);
							},

			'A'..='Z' | 'a'..='z' | '_' => {
								idx += 1;
let mut token = chars.next().unwrap().to_string();

				while let Some(&next) = chars.peek() {
					if !next.is_ascii_alphanumeric() && next != '_' {
						break;
					}

					idx += 1;
					token.push(chars.next().unwrap());
									}

				res.push(token.parse()?);
			},

			'\n' => {
								res.push(Token::EndOfLine);

				idx += 1;
				chars.next();
			},

			'\r' => {
				idx += 1;
chars.next();

				if chars.peek() == Some(&'\n') {
					chars.next();
					idx += 1;
				}

				res.push(Token::EndOfLine);
			},

			' ' => {
				idx += 1;
				chars.next();
			},

			&c => {
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

pub fn try_tokenize_number(idx: usize, code: &str) -> Result<(usize, Token), TokenizationError> {
	let mut chars = code.chars().skip(idx).peekable();
	let mut token = String::new();
	let mut is_frac = false;
	let mut is_expo = false;
	let mut token_len = 0;

	while let Some(&next) = chars.peek() {
		match next {
			'0'..='9' => {
				token.push(chars.next().unwrap());
				token_len += 1;
			},

			'.' => {
				token.push(chars.next().unwrap());
				token_len += 1;

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

			'e' | 'E' => {
				token.push(chars.next().unwrap());
				token_len += 1;

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

				while chars.peek() == Some(&'_') {
					chars.next();
					token_len += 1;
				}

				match chars.next() {
					Some('+') => token.push('+'),
					Some('-') => token.push('-'),
					Some(n) if n.is_ascii_digit() => token.push(n),
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

				token_len += 1;
			},

			'A'..='Z' | 'a'..='z' => {
				token.push(next);
				return Err(TokenizationError {
					kind: TokenizationErrorKind::UnspportedSyntax(chars.next().unwrap().to_string()),
					token_str: Some(token),
					message: Some(String::from(
						"Suffixes other than 'e' are not supported.",
					)),
				});
			},

			'_' => {
				chars.next();
				token_len += 1;
			},

			_ => break,
		}
	}

	match token.parse() {
		Ok(n) => Ok((token_len, Token::NumericLiteral(n))),
		Err(e) => Err(TokenizationError {
			kind: TokenizationErrorKind::NotANumber,
			token_str: Some(token),
			message: Some(e.to_string()),
		}),
	}
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
