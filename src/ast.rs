use crate::{errors::ParsingError, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub struct ASTNode {
	pub value: ASTNodeValue,
	pub store_in_ans: bool,
	pub print_result: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNodeValue {
	Variable(String),
	Number(f64),
	Matrix(Vec<Vec<ASTNode>>),
	Operator(Operator),
	ArithmaticExpr(Vec<ASTNodeValue>),
}

impl From<ASTNodeValue> for ASTNode {
	fn from(value: ASTNodeValue) -> Self {
		Self {
			value,
			store_in_ans: false,
			print_result: false,
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
	Add,
	Subtract,
	Multiply,
	Divide,
	Assign,
}

impl Operator {
	pub fn precedence(&self) -> u8 {
		match self {
			Operator::Multiply | Operator::Divide => 3,
			Operator::Add | Operator::Subtract => 2,
			Operator::Assign => 1,
			// Token::OpenParen | None => 0
		}
	}

	pub fn tokenize(&self) -> Token {
		match self {
			Self::Add => Token::Plus,
			Self::Subtract => Token::Minus,
			Self::Multiply => Token::Asterisk,
			Self::Divide => Token::Slash,
			Self::Assign => Token::Equal,
		}
	}
}

impl TryFrom<Token> for Operator {
	type Error = ParsingError;

	fn try_from(value: Token) -> Result<Self, Self::Error> {
		match value {
			Token::Plus => Ok(Self::Add),
			Token::Minus => Ok(Self::Subtract),
			Token::Asterisk => Ok(Self::Multiply),
			Token::Slash => Ok(Self::Divide),
			Token::Equal => Ok(Self::Assign),
			t => Err(ParsingError::UnexpectedToken {
				expected: Some(String::from("Binary Operator")),
				found: Some(t.stringify()),
			}),
		}
	}
}
