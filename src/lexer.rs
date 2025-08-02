use crate::{
	errors::{TokenizationError, TokenizationErrorKind},
	token::Token,
};

pub struct Lexer {
	input: Box<[char]>,
	idx: usize,
	pub last_error: Option<TokenizationError>,
}

impl Iterator for Lexer {
	type Item = Result<Token, TokenizationError>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.idx > self.input.len() {
			return None;
		}

		let first = match self.advance() {
			Some(c) => c,
			None => return Some(Ok(Token::EndOfFile)),
		};

		match first {
			'+' | '-' | '*' | '/' | '(' | ')' | '[' | ']' | '{' | '}' | '=' | ',' | ';' => {
				Some(first.to_string().parse())
			},

			'0'..='9' => {
				self.idx -= 1;
				Some(self.tokenize_number())
			},

			'A'..='Z' | 'a'..='z' | '_' => {
				let mut res = first.to_string();

				while let Some(&next) = self.advance() {
					if !next.is_ascii_alphanumeric() && next != '_' {
						break;
					}
					res.push(next);
				}

				self.idx -= 1;
				Some(res.parse())
			},

			'\n' => {
				self.advance();
				Some(Ok(Token::EndOfLine))
			},

			'\r' => {
				self.advance();
				if self.advance() != Some(&'\n') {
					self.idx -= 1;
				}
				Some(Ok(Token::EndOfLine))
			},

			' ' => {
				while self.advance() == Some(&' ') {}
				self.idx -= 1;
				self.next()
			},

			&c => {
				let e =
					TokenizationError::new(TokenizationErrorKind::UnexpectedChar(c), None, None);
				self.last_error = Some(e.clone());
				Some(Err(e))
			},
		}
	}
}

impl Lexer {
	pub fn new(input: Box<[char]>) -> Self {
		Self {
			input,
			idx: 0,
			last_error: None,
		}
	}

	fn advance(&mut self) -> Option<&char> {
		let res = self.input.get(self.idx);
		self.idx += 1;
		res
	}

	fn tokenize_number(&mut self) -> Result<Token, TokenizationError> {
		let mut res = String::new();
		let mut is_frac = false;
		let mut is_expo = false;

		loop {
			let next = match self.advance() {
				None => {
					self.idx -= 1;
					break;
				},
				Some(&c) => c,
			};

			match next {
				'0'..='9' => {
					res.push(next);
				},

				'.' => {
					res.push(next);

					if is_expo {
						return Err(TokenizationError::new(
							TokenizationErrorKind::UnexpectedChar('.'),
							Some(res),
							Some(String::from(
								"Could not parse a numeric literal with a floating point number after the 'e' in a scientific notation.",
							)),
						));
					}

					if is_frac {
						return Err(TokenizationError::new(
							TokenizationErrorKind::UnexpectedChar('.'),
							Some(res),
							Some(String::from(
								"Could not parse a numeric literal with more than one dot.",
							)),
						));
					}

					is_frac = true;
				},

				'e' | 'E' => {
					res.push(next);

					if is_expo {
						return Err(TokenizationError::new(
							TokenizationErrorKind::UnexpectedChar('e'),
							Some(res),
							Some(String::from(
								"Could not parse a numeric literal with more than one 'e' suffix,\n(Invalid scientific notation :)",
							)),
						));
					}

					is_expo = true;
					is_frac = true;

					while self.advance() == Some(&'_') {}
					self.idx -= 1;

					match self.advance() {
						Some('+') => res.push('+'),
						Some('-') => res.push('-'),
						Some(&n) if n.is_ascii_digit() => res.push(n),
						None | Some(_) => {
							return Err(TokenizationError::new(
								TokenizationErrorKind::UnexpectedChar('e'),
								Some(res),
								Some(String::from("A scientific notated number is not complete.")),
							));
						},
					}
				},

				'A'..='Z' | 'a'..='z' => {
					res.push(next);
					return Err(TokenizationError::new(
						TokenizationErrorKind::UnspportedSyntax(next.to_string()),
						Some(res),
						Some(String::from("Suffixes other than 'e' are not supported.")),
					));
				},

				'_' => {},

				_ => {
					self.idx -= 1;
					break;
				},
			}
		}

		match res.parse() {
			Ok(n) => Ok(Token::NumericLiteral(n)),
			Err(e) => Err(TokenizationError::new(
				TokenizationErrorKind::NotANumber,
				Some(res),
				Some(e.to_string()),
			)),
		}
	}
}
