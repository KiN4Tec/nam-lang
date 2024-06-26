use crate::lexer::Token;
use color_eyre::eyre::Result;
use reedline::Span;

#[derive(Debug)]
pub struct ASTNode {
	pub kind: ASTNodeKind,
	pub store_in_ans: bool,
	pub print_result: bool,

	#[allow(unused)]
	pub span: Option<Span>,
}

#[derive(Debug)]
pub enum ASTNodeKind {
	Variable(String),
	Number(f64),
	Matrix(Vec<Vec<ASTNode>>),

	Assignment(String, Box<ASTNode>),

	BinaryExpr(BinaryOpKind, Box<ASTNode>, Box<ASTNode>),
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

	fn try_from(token: Token) -> std::result::Result<Self, Self::Error> {
		match token {
			Token::OpAdd => Ok(Self::Add),
			Token::OpSubtract => Ok(Self::Subtract),
			Token::OpMultiply => Ok(Self::Multiply),
			Token::OpDivide => Ok(Self::Divide),

			_ => Err(ParsingError::UnexpectedToken {
				expected: Some("Operator".to_string()),
				found: Some(token.stringify()),
			}),
		}
	}
}

impl TryFrom<&Token> for BinaryOpKind {
	type Error = ParsingError;

	fn try_from(token: &Token) -> std::result::Result<Self, Self::Error> {
		Self::try_from(token.clone())
	}
}

impl TryFrom<&Vec<Token>> for ASTNode {
	type Error = ParsingError;

	fn try_from(tokens: &Vec<Token>) -> Result<Self, Self::Error> {
		let (_, res) = Self::parse_program(0, tokens)?;
		Ok(res)
	}
}

impl From<ASTNodeKind> for ASTNode {
	fn from(value: ASTNodeKind) -> Self {
		Self {
			kind: value,
			span: None,
			store_in_ans: false,
			print_result: false,
		}
	}
}

impl ASTNode {
	fn parse_program(idx: usize, tokens: &[Token]) -> Result<(usize, Self), ParsingError> {
		Self::parse_stmt(idx, tokens)
	}

	fn parse_stmt(idx: usize, tokens: &[Token]) -> Result<(usize, Self), ParsingError> {
		let (res_len, mut res) = Self::parse_expr(idx, tokens)?;

		match tokens.get(idx + res_len) {
			Some(Token::EndOfFile) | Some(Token::EndOfLine) => {
				res.print_result = true;
			},

			Some(Token::SemiColon) => {
				res.print_result = false;
			},

			Some(token) => {
				return Err(ParsingError::UnexpectedToken {
					expected: Some(Token::EndOfFile.stringify()),
					found: Some(token.stringify()),
				})
			},

			None => unreachable!(),
		}

		res.store_in_ans = match res.kind {
			ASTNodeKind::Number(_) => true,
			ASTNodeKind::Matrix(_) => true,
			ASTNodeKind::BinaryExpr(_, _, _) => true,

			ASTNodeKind::Variable(_) => false,
			ASTNodeKind::Assignment(_, _) => false,
		};

		Ok((res_len, res))
	}

	fn parse_expr(idx: usize, tokens: &[Token]) -> Result<(usize, Self), ParsingError> {
		Self::parse_assignment_expr(idx, tokens)
	}

	fn parse_assignment_expr(idx: usize, tokens: &[Token]) -> Result<(usize, Self), ParsingError> {
		let (primary_len, primary) = Self::parse_additive_expr(idx, tokens)?;

		// Assignment Statement (x = 5)
		if let ASTNodeKind::Variable(lhs) = &primary.kind {
			if tokens.get(idx + primary_len) == Some(&Token::OpAssign) {
				let (rhs_len, rhs) = Self::parse_expr(idx + primary_len + 1, tokens)?;
				return Ok((
					primary_len + 1 + rhs_len,
					ASTNodeKind::Assignment(lhs.to_string(), Box::new(rhs)).into(),
				));
			}
		}

		Ok((primary_len, primary))
	}

	fn parse_additive_expr(idx: usize, tokens: &[Token]) -> Result<(usize, Self), ParsingError> {
		let (mut consumed_len, mut lhs) = Self::parse_multiplicative_expr(idx, tokens)?;

		while let Some(token) = tokens.get(idx + consumed_len) {
			if *token != Token::OpAdd && *token != Token::OpSubtract {
				break;
			}
			// Consume the operator
			consumed_len += 1;

			let (consumed_rhs, rhs) = Self::parse_multiplicative_expr(idx + consumed_len, tokens)?;
			consumed_len += consumed_rhs;

			lhs = ASTNodeKind::BinaryExpr(token.try_into()?, Box::new(lhs), Box::new(rhs)).into();
		}

		Ok((consumed_len, lhs))
	}

	fn parse_multiplicative_expr(
		idx: usize,
		tokens: &[Token],
	) -> Result<(usize, Self), ParsingError> {
		let (mut consumed_len, mut lhs) = Self::parse_parenthesised_expr(idx, tokens)?;

		while let Some(token) = tokens.get(idx + consumed_len) {
			if *token != Token::OpMultiply && *token != Token::OpDivide {
				break;
			}
			// Consume the operator
			consumed_len += 1;

			let (consumed_rhs, rhs) = Self::parse_parenthesised_expr(idx + consumed_len, tokens)?;
			consumed_len += consumed_rhs;

			lhs = ASTNodeKind::BinaryExpr(token.try_into()?, Box::new(lhs), Box::new(rhs)).into();
		}

		Ok((consumed_len, lhs))
	}

	fn parse_parenthesised_expr(
		idx: usize,
		tokens: &[Token],
	) -> Result<(usize, Self), ParsingError> {
		// Skip if not an OpenParen
		if tokens.get(idx) != Some(&Token::OpenParen) {
			return Self::parse_primary_expr(idx, tokens);
		}

		// Consuming the Open Paren
		let mut consumed_len = 1;

		// Parsing the expression inside
		let (inside_len, result) = Self::parse_expr(idx + 1, tokens)?;
		consumed_len += inside_len;

		// Expect a closing paren
		match tokens.get(idx + consumed_len) {
			Some(Token::CloseParen) => {},

			None => return Err(ParsingError::UnexpectedEndOfInput),
			Some(_) => {
				return Err(ParsingError::UnexpectedToken {
					expected: Some(Token::CloseParen.stringify()),
					found: Some(tokens[idx + consumed_len].stringify()),
				})
			},
		}

		// Consuming the Closing Paren
		consumed_len += 1;

		Ok((consumed_len, result))
	}

	fn parse_primary_expr(idx: usize, tokens: &[Token]) -> Result<(usize, Self), ParsingError> {
		let token = match tokens.get(idx) {
			Some(token) => token,
			None => {
				return Err(ParsingError::UnexpectedEndOfInput);
			},
		};

		let kind = match token {
			Token::Identifier(var_name) => ASTNodeKind::Variable(var_name.clone()),
			Token::NumericLiteral(n) => ASTNodeKind::Number(*n),
			Token::OpenBrace => return Self::parse_matrix(idx, tokens),

			token => {
				return Err(ParsingError::UnexpectedToken {
					expected: Some("Expression".to_string()),
					found: Some(token.stringify()),
				})
			},
		};

		let res = Self {
			kind,
			span: None,
			store_in_ans: false,
			print_result: false,
		};

		Ok((1, res))
	}

	fn parse_matrix(idx: usize, tokens: &[Token]) -> Result<(usize, Self), ParsingError> {
		// Consume the open bracket
		let mut consumed_len = 1;

		// Check if the matrix is empty
		if tokens.get(idx + consumed_len) == Some(&Token::CloseBrace) {
			let mat = vec![];

			let res = Self {
				kind: ASTNodeKind::Matrix(mat),
				span: None,
				store_in_ans: false,
				print_result: false,
			};

			consumed_len += 1;
			return Ok((consumed_len, res));
		}

		// Parse the first element (to initialize the matrix)
		let (first_len, first) = Self::parse_expr(idx + consumed_len, tokens)?;
		consumed_len += first_len;

		let mut mat = vec![vec![first]];
		let mut i = 0;
		let mut is_already_comma_seperated = false;

		loop {
			match tokens.get(idx + consumed_len) {
				None => return Err(ParsingError::UnexpectedEndOfInput),

				Some(&Token::CloseBrace) => {
					consumed_len += 1;
					break;
				},

				Some(&Token::Comma) => {
					if is_already_comma_seperated {
						return Err(ParsingError::EmptyMatrixElement);
					}
					is_already_comma_seperated = true;
					consumed_len += 1;
				},

				Some(&Token::SemiColon) => {
					while tokens.get(idx + consumed_len) == Some(&Token::SemiColon) {
						consumed_len += 1;
					}

					if i >= 1 && mat[i - 1].len() != mat[i].len() {
						return Err(ParsingError::DimensionsMismatch(
							mat[i - 1].len(),
							mat[i].len(),
						));
					}

					mat.push(vec![]);
					i += 1;
				},

				_ => {
					let (tmp_len, tmp) = Self::parse_expr(idx + consumed_len, tokens)?;
					mat[i].push(tmp);
					consumed_len += tmp_len;
					is_already_comma_seperated = false;
				},
			}
		}

		if i >= 1 && mat[i - 1].len() != mat[i].len() {
			return Err(ParsingError::DimensionsMismatch(
				mat[i - 1].len(),
				mat[i].len(),
			));
		}

		let res = Self {
			kind: ASTNodeKind::Matrix(mat),
			span: None,
			store_in_ans: false,
			print_result: false,
		};

		Ok((consumed_len, res))
	}
}

////////////////////////////////
//       Error Handling       //
////////////////////////////////

#[derive(Debug)]
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
