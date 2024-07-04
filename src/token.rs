use crate::errors::{TokenizationError, TokenizationErrorKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Plus,     // +
	Minus,    // -
	Asterisk, // *
	Slash,    // /
	Equal,    // =

	OpenParen,    // )
	CloseParen,   // (
	OpenBracket,  // [
	CloseBracket, // ]
	OpenCurly,    // {
	CloseCurly,   // }

	NumericLiteral(f64),
	Identifier(String),

	Comma,     // ,
	SemiColon, // ;
	EndOfLine,
	EndOfFile,
}

impl Token {
	pub fn stringify(&self) -> String {
		let res = match self {
			Self::Plus => "Add",
			Self::Minus => "Minus",
			Self::Asterisk => "Asterisk",
			Self::Slash => "Slash",
			Self::Equal => "Equal",

			Self::OpenParen => "OpenParen",
			Self::CloseParen => "CloseParen",
			Self::OpenBracket => "OpenBracket",
			Self::CloseBracket => "CloseBracket",
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
		match input {
			"+" => return Ok(Self::Plus),
			"-" => return Ok(Self::Minus),
			"*" => return Ok(Self::Asterisk),
			"/" => return Ok(Self::Slash),
			"=" => return Ok(Self::Equal),

			"(" => return Ok(Self::OpenParen),
			")" => return Ok(Self::CloseParen),
			"[" => return Ok(Self::OpenBracket),
			"]" => return Ok(Self::CloseBracket),
			"{" => return Ok(Self::OpenCurly),
			"}" => return Ok(Self::CloseCurly),

			"," => return Ok(Self::Comma),
			";" => return Ok(Self::SemiColon),

			_ => {},
		};

		let input = input.to_string();

		let first = match input.chars().next() {
			Some(c) => c,
			None => {
				return Err(TokenizationError::new(
					TokenizationErrorKind::EmptyString,
					Some(input),
					None,
				))
			},
		};

		match first {
			'0'..='9' => match input.parse() {
				Ok(n) => Ok(Self::NumericLiteral(n)),
				Err(e) => Err(TokenizationError::new(
					TokenizationErrorKind::NotANumber,
					Some(input),
					Some(e.to_string()),
				)),
			},

			'A'..='Z' | 'a'..='z' | '_' => {
				for c in input.chars() {
					if !c.is_ascii_alphanumeric() && c != '_' {
						return Err(TokenizationError::new(
							TokenizationErrorKind::UnexpectedChar(c),
							Some(input),
							None,
						));
					}
				}
				Ok(Self::Identifier(input))
			},

			first => Err(TokenizationError::new(
				TokenizationErrorKind::UnexpectedChar(first),
				Some(input),
				None,
			)),
		}
	}
}
