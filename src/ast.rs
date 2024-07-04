use crate::{errors::ParsingError, token::Token};

#[derive(Debug)]
pub struct ASTNode {
	pub kind: ASTNodeKind,
	pub store_in_ans: bool,
	pub print_result: bool,
}

#[derive(Debug)]
pub enum ASTNodeKind {
	Variable(String),
	Number(f64),
	Matrix(Vec<Vec<ASTNode>>),

	Assignment(String, Box<ASTNode>),

	BinaryExpr(BinaryOpKind, Box<ASTNode>, Box<ASTNode>),
}

impl From<ASTNodeKind> for ASTNode {
	fn from(value: ASTNodeKind) -> Self {
		Self {
			kind: value,
			store_in_ans: false,
			print_result: false,
		}
	}
}

#[derive(Debug)]
pub enum BinaryOpKind {
	Add,
	Subtract,
	Multiply,
	Divide,
}

impl TryFrom<Token> for BinaryOpKind {
	type Error = ParsingError;

	fn try_from(token: Token) -> Result<Self, Self::Error> {
		match token {
			Token::Plus => Ok(Self::Add),
			Token::Minus => Ok(Self::Subtract),
			Token::Asterisk => Ok(Self::Multiply),
			Token::Slash => Ok(Self::Divide),

			_ => Err(ParsingError::UnexpectedToken {
				expected: Some("Operator".to_string()),
				found: Some(token.stringify()),
			}),
		}
	}
}
