use crate::{
	ast::{ASTNode, ASTNodeKind},
	errors::ParsingError,
	token::Token,
};

pub struct Parser {
	idx: usize,
	tokens: Box<[Token]>,
}

impl Parser {
	pub fn new(tokens: &[Token]) -> Self {
		Self {
			idx: 0,
			tokens: tokens.into(),
		}
	}

	fn get(&mut self) -> Option<&Token> {
		self.tokens.get(self.idx)
	}

	fn advance(&mut self) -> Option<&Token> {
		self.idx += 1;
		self.tokens.get(self.idx - 1)
	}

	pub fn parse(&mut self) -> Result<ASTNode, ParsingError> {
		self.parse_stmt()
	}

	fn parse_stmt(&mut self) -> Result<ASTNode, ParsingError> {
		let mut res = self.parse_expr()?;

		match self.advance() {
			None | Some(Token::EndOfFile) | Some(Token::EndOfLine) => {
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
		}

		res.store_in_ans = match res.kind {
			ASTNodeKind::Number(_) => true,
			ASTNodeKind::Matrix(_) => true,
			ASTNodeKind::BinaryExpr(_, _, _) => true,

			ASTNodeKind::Variable(_) => false,
			ASTNodeKind::Assignment(_, _) => false,
		};

		Ok(res)
	}

	fn parse_expr(&mut self) -> Result<ASTNode, ParsingError> {
		self.parse_assignment_expr()
	}

	fn parse_assignment_expr(&mut self) -> Result<ASTNode, ParsingError> {
		let primary = self.parse_additive_expr()?;

		// Assignment Statement (x = 5)
		if let ASTNodeKind::Variable(lhs) = &primary.kind {
			if self.get() == Some(&Token::Equal) {
				self.advance();
				let rhs = self.parse_expr()?;
				return Ok(ASTNodeKind::Assignment(lhs.to_string(), Box::new(rhs)).into());
			}
		}

		Ok(primary)
	}

	fn parse_additive_expr(&mut self) -> Result<ASTNode, ParsingError> {
		let mut lhs = self.parse_multiplicative_expr()?;

		while let Some(token) = self.get() {
			if token != &Token::Plus && token != &Token::Minus {
				break;
			}
			let token = self.advance().cloned().unwrap();
			let rhs = self.parse_multiplicative_expr()?;
			lhs = ASTNodeKind::BinaryExpr(token.try_into()?, Box::new(lhs), Box::new(rhs)).into();
		}

		Ok(lhs)
	}

	fn parse_multiplicative_expr(&mut self) -> Result<ASTNode, ParsingError> {
		let mut lhs = self.parse_parenthesised_expr()?;

		while let Some(token) = self.get() {
			if token != &Token::Asterisk && token != &Token::Slash {
				break;
			}
			let token = self.advance().cloned().unwrap();
			let rhs = self.parse_parenthesised_expr()?;
			lhs = ASTNodeKind::BinaryExpr(token.try_into()?, Box::new(lhs), Box::new(rhs)).into();
		}

		Ok(lhs)
	}

	fn parse_parenthesised_expr(&mut self) -> Result<ASTNode, ParsingError> {
		if self.get() != Some(&Token::OpenParen) {
			return self.parse_primary_expr();
		}

		self.advance();
		let result = self.parse_expr()?;

		match self.advance() {
			Some(Token::CloseParen) => {},

			None => return Err(ParsingError::UnexpectedEndOfInput),
			Some(t) => {
				return Err(ParsingError::UnexpectedToken {
					expected: Some(Token::CloseParen.stringify()),
					found: Some(t.stringify()),
				})
			},
		}

		Ok(result)
	}

	fn parse_primary_expr(&mut self) -> Result<ASTNode, ParsingError> {
		let token = match self.advance().cloned() {
			Some(token) => token,
			None => {
				return Err(ParsingError::UnexpectedEndOfInput);
			},
		};

		let kind = match token {
			Token::Identifier(var_name) => ASTNodeKind::Variable(var_name.clone()),
			Token::NumericLiteral(n) => ASTNodeKind::Number(n),
			Token::OpenBracket => return self.parse_matrix(),

			token => {
				return Err(ParsingError::UnexpectedToken {
					expected: Some("Expression".to_string()),
					found: Some(token.stringify()),
				})
			},
		};

		let res = ASTNode::from(kind);

		Ok(res)
	}

	fn parse_matrix(&mut self) -> Result<ASTNode, ParsingError> {
		if self.get() == Some(&Token::CloseBracket) {
			let mat = vec![];

			let res = ASTNode {
				kind: ASTNodeKind::Matrix(mat),
				store_in_ans: false,
				print_result: false,
			};

			self.advance();
			return Ok(res);
		}

		let first = self.parse_expr()?;

		let mut mat = vec![vec![first]];
		let mut i = 0;
		let mut is_already_comma_seperated = false;

		loop {
			match self.get() {
				None => return Err(ParsingError::UnexpectedEndOfInput),

				Some(&Token::CloseBracket) => {
					self.advance();
					break;
				},

				Some(&Token::Comma) => {
					if is_already_comma_seperated {
						return Err(ParsingError::EmptyMatrixElement);
					}
					is_already_comma_seperated = true;
					self.advance();
				},

				Some(&Token::SemiColon) => {
					if i >= 1 && mat[i - 1].len() != mat[i].len() {
						return Err(ParsingError::DimensionsMismatch(
							mat[i - 1].len(),
							mat[i].len(),
						));
					}

					mat.push(vec![]);
					i += 1;

					self.advance();
				},

				_ => {
					let expr = self.parse_expr()?;
					mat[i].push(expr);
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

		let res = ASTNode {
			kind: ASTNodeKind::Matrix(mat),
			store_in_ans: false,
			print_result: false,
		};

		Ok(res)
	}
}
