use crate::{
	errors::{TokenizationError, TokenizationErrorKind},
	token::Token,
};
use std::str::Chars;

pub struct Lexer<'l> {
	input: std::iter::Peekable<Chars<'l>>,
	is_eof_retruned: bool,
	pub last_error: Option<TokenizationError>,
}

impl<'l> Iterator for Lexer<'l> {
	type Item = Result<Token, TokenizationError>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.input.peek().is_none() {
			if self.is_eof_retruned {
				return None;
			} else {
				self.is_eof_retruned = true;
				return Some(Ok(Token::EndOfFile));
			}
		}

		match self.input.peek().unwrap() {
			'+' | '-' | '*' | '/' | '(' | ')' | '[' | ']' | '{' | '}' | '=' | ',' | ';' => {
				Some(self.input.next().unwrap().to_string().parse())
			},

			'0'..='9' => Some(self.tokenize_number()),

			'A'..='Z' | 'a'..='z' | '_' => {
				let mut res = self.input.next().unwrap().to_string();

				while let Some(c) = self
					.input
					.next_if(|c| c.is_ascii_alphanumeric() || c == &'_')
				{
					res.push(c);
				}

				Some(res.parse())
			},

			'\n' => {
				self.input.next();
				Some(Ok(Token::EndOfLine))
			},

			'\r' => {
				self.input.next();
				self.input.next_if_eq(&'\n');
				Some(Ok(Token::EndOfLine))
			},

			' ' => {
				while self.input.next_if_eq(&' ').is_some() {};
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

impl<'l> Lexer<'l> {
	pub fn new(input: Chars<'l>) -> Self {
		Self {
			input: input.peekable(),
			is_eof_retruned: false,
			last_error: None,
		}
	}

	fn tokenize_number(&mut self) -> Result<Token, TokenizationError> {
		let mut res = String::new();
		let mut is_frac = false;
		let mut is_expo = false;

		while let Some(next) = self.input.peek() {
			match next {
				'0'..='9' => {
					res.push(self.input.next().unwrap());
				},

				'.' => {
					res.push(self.input.next().unwrap());

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
					res.push(self.input.next().unwrap());

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

					while self.input.next_if_eq(&'_').is_some() {}

					match self.input.next() {
						Some('+') => res.push('+'),
						Some('-') => res.push('-'),
						Some(c) if c.is_ascii_digit() => res.push(c),
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
					let next = self.input.next().unwrap();
					res.push(next);
					return Err(TokenizationError::new(
						TokenizationErrorKind::UnspportedSyntax(next.to_string()),
						Some(res),
						Some(String::from("Suffixes other than 'e' are not supported.")),
					));
				},

				'_' => {},

				_ => break,
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
