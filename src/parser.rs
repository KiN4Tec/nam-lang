use crate::{
	ast::{ASTNode, ASTNodeValue, Operator},
	errors::ParsingError,
	token::Token,
};

pub struct Parser {
	idx: usize,
	tokens: Box<[Token]>,
	is_inside_matrix: bool,
}

impl Parser {
	pub fn new(tokens: &[Token]) -> Self {
		Self {
			idx: 0,
			tokens: tokens.into(),
			is_inside_matrix: false,
		}
	}

	fn current(&mut self) -> Option<&Token> {
		self.tokens.get(self.idx)
	}

	fn next(&mut self) -> Option<&Token> {
		self.idx += 1;
		self.tokens.get(self.idx)
	}

	fn advance(&mut self) {
		self.idx += 1;
	}

	pub fn parse(&mut self) -> Result<ASTNode, ParsingError> {
		self.parse_stmt()
	}

	fn parse_stmt(&mut self) -> Result<ASTNode, ParsingError> {
		let mut res = self.parse_expr()?;

		match self.current() {
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
				});
			},

			None => {
				res.print_result = true;
				unreachable!("We've somehow passed the EOF token at the end of the tokens array");
			},
		}

		res.store_in_ans = match res.value {
			ASTNodeValue::Number(_) => true,
			ASTNodeValue::Matrix(_) => true,

			ASTNodeValue::ArithmaticExpr(ref expr) => {
				!expr.contains(&ASTNodeValue::Operator(Operator::Assign))
			},

			ASTNodeValue::Variable(_) => false,

			ASTNodeValue::Operator(op) => {
				return Err(ParsingError::UnexpectedToken {
					expected: None,
					found: Some(op.tokenize().stringify()),
				});
			},
		};

		Ok(res)
	}

	fn parse_expr(&mut self) -> Result<ASTNode, ParsingError> {
		self.parse_arithmatic_expr()
	}

	fn parse_arithmatic_expr(&mut self) -> Result<ASTNode, ParsingError> {
		let mut temp: Vec<Token> = vec![];
		let mut res: Vec<ASTNodeValue> = vec![];
		let mut precedence_stack = vec![];
		let mut last_precedence = 0; // The precedence of the last element in the temp stack
		let mut last_was_operand = false;

		while let Some(token) = self.current() {
			match token {
				Token::NumericLiteral(n) => {
					if last_was_operand {
						if self.is_inside_matrix {
							break;
						}
						return Err(ParsingError::InvalidArithmaticExpression);
					}

					res.push(ASTNodeValue::Number(*n));
					self.advance();

					last_was_operand = true;
				},

				Token::Identifier(var_name) => {
					if last_was_operand {
						if self.is_inside_matrix {
							break;
						}
						return Err(ParsingError::InvalidArithmaticExpression);
					}

					res.push(ASTNodeValue::Variable(var_name.clone()));
					self.advance();

					last_was_operand = true;
				},

				Token::OpenBracket => {
					if last_was_operand {
						if self.is_inside_matrix {
							break;
						}
						return Err(ParsingError::InvalidArithmaticExpression);
					}

					res.push(self.parse_matrix()?);

					last_was_operand = true;
				},

				Token::Plus | Token::Minus | Token::Asterisk | Token::Slash | Token::Equal => {
					if !last_was_operand {
						return Err(ParsingError::InvalidArithmaticExpression);
					}

					let precedence = Operator::try_from(token.clone())?.precedence();

					while precedence < last_precedence {
						res.push(ASTNodeValue::Operator(Operator::try_from(
							temp.pop().unwrap(), // Unwrapping because loop will break on None
						)?));

						// Start of next loop
						last_precedence = match temp.last() {
							Some(Token::OpenParen) => 0, // loop will break automatically, so OpenParen should never get poped here
							Some(o) => Operator::try_from(o.clone())?.precedence(),
							None => 0, // Unwrapping should be safe because of this
						}
					}

					last_precedence = precedence;
					temp.push(token.clone());
					self.advance();

					last_was_operand = false;
				},

				Token::OpenParen => {
					if last_was_operand {
						if self.is_inside_matrix {
							break;
						}
						return Err(ParsingError::InvalidArithmaticExpression);
					}

					temp.push(Token::OpenParen);
					precedence_stack.push(last_precedence);
					last_precedence = 0;
					self.advance();

					last_was_operand = false;
				},

				Token::CloseParen => {
					if !last_was_operand {
						return Err(ParsingError::InvalidArithmaticExpression);
					}

					loop {
						let last = match temp.pop() {
							Some(t) => t,
							None => return Err(ParsingError::UnmatchedCloseParen),
						};

						if last == Token::OpenParen {
							last_precedence = precedence_stack.pop().unwrap();
							break;
						}

						res.push(ASTNodeValue::Operator(Operator::try_from(last)?));
					}

					self.advance();

					last_was_operand = true;
				},

				_ => break,
			};
		}

		if res.is_empty() {
			return Err(ParsingError::UnexpectedEndOfInput);
		}

		while let Some(token) = temp.pop() {
			if token == Token::OpenParen {
				return Err(ParsingError::UnmatchedOpenParen);
			}
			res.push(ASTNodeValue::Operator(Operator::try_from(token)?));
		}

		if res.len() == 1 {
			Ok(ASTNode {
				value: res[0].clone(),
				store_in_ans: false,
				print_result: false,
			})
		} else {
			res.reverse();
			Ok(ASTNode {
				value: ASTNodeValue::ArithmaticExpr(res),
				store_in_ans: false,
				print_result: false,
			})
		}
	}

	fn parse_matrix(&mut self) -> Result<ASTNodeValue, ParsingError> {
		assert_eq!(self.current(), Some(&Token::OpenBracket));
		if self.next() == Some(&Token::CloseBracket) {
			self.advance();
			return Ok(ASTNodeValue::Matrix(vec![]));
		}

		self.is_inside_matrix = true;
		let first = self.parse_expr()?;

		let mut mat = Vec::new();
		let mut row = vec![first];

		while let Some(t) = self.current() {
			match t {
				Token::Comma => self.advance(),
				Token::SemiColon => {
					self.advance();

					if row.is_empty() {
						continue;
					}

					let cap = row.len();
					mat.push(row);
					row = Vec::with_capacity(cap);
				},

				Token::CloseBracket => {
					self.advance();
					if !row.is_empty() {
						mat.push(row);
					}
					self.is_inside_matrix = false; // WARN: we could be inside a child matrix here
					return Ok(ASTNodeValue::Matrix(mat));
				},

				Token::EndOfFile => return Err(ParsingError::IncompleteStatement),

				_ => {
					self.is_inside_matrix = true; // This is here because a child matrix could set this to false
					let expr = self.parse_expr()?;
					row.push(expr);
				},
			}
		}

		unreachable!("We've somehow passed the EOF token at the end of the tokens array")
	}
}
