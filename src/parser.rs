use crate::{
	ast::{ASTNode, ASTNodeValue, Operator},
	errors::ParsingError,
	lexer::Lexer,
	token::Token,
};

pub struct Parser<'a> {
	input: std::iter::Peekable<Lexer<'a>>,
	is_inside_matrix: bool,
	last_token: Option<Token>,
}

impl<'a> Parser<'a> {
	pub fn new(input: Lexer<'a>) -> Self {
		Self {
			input: input.peekable(),
			is_inside_matrix: false,
			last_token: None,
		}
	}

	pub fn parse(&mut self) -> Result<ASTNode, ParsingError> {
		self.parse_stmt()
	}

	fn parse_stmt(&mut self) -> Result<ASTNode, ParsingError> {
		let mut res = self.parse_expr()?;

		match self.next_token()? {
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

		while let Some(token) = self.next_token()? {
			match token {
				Token::NumericLiteral(n) => {
					if last_was_operand {
						if self.is_inside_matrix {
							self.last_token = Some(token);
							break;
						}
						return Err(ParsingError::InvalidArithmaticExpression);
					}

					res.push(ASTNodeValue::Number(n));

					last_was_operand = true;
				},

				Token::Identifier(var_name) => {
					if last_was_operand {
						if self.is_inside_matrix {
							self.last_token = Some(Token::Identifier(var_name));
							break;
						}
						return Err(ParsingError::InvalidArithmaticExpression);
					}

					res.push(ASTNodeValue::Variable(var_name));

					last_was_operand = true;
				},

				Token::OpenBracket => {
					if last_was_operand {
						if self.is_inside_matrix {
							self.last_token = Some(token);
							break;
						}
						return Err(ParsingError::InvalidArithmaticExpression);
					}

					self.last_token = Some(token);
					res.push(self.parse_matrix()?);

					last_was_operand = true;
				},

				Token::Plus | Token::Minus | Token::Asterisk | Token::Slash | Token::Equal => {
					if !last_was_operand {
						return Err(ParsingError::InvalidArithmaticExpression);
					}

					let precedence = Operator::try_from(token.clone())?.precedence();

					while precedence < last_precedence {
						res.push(ASTNodeValue::Operator(
							temp.pop().unwrap().try_into().unwrap(), // Unwrapping because loop will break on None
						));

						// Start of next loop
						last_precedence = match temp.last().cloned() {
							Some(Token::OpenParen) => 0, // loop will break automatically, so OpenParen should never get poped here
							Some(o) => Operator::try_from(o).unwrap().precedence(),
							None => 0, // Unwrapping should be safe because of this
						}
					}

					last_precedence = precedence;
					temp.push(token);

					last_was_operand = false;
				},

				Token::OpenParen => {
					if last_was_operand {
						if self.is_inside_matrix {
							self.last_token = Some(token);
							break;
						}
						return Err(ParsingError::InvalidArithmaticExpression);
					}

					temp.push(token);
					precedence_stack.push(last_precedence);
					last_precedence = 0;

					last_was_operand = false;
				},

				Token::CloseParen => {
					if !last_was_operand {
						return Err(ParsingError::InvalidArithmaticExpression);
					}

					loop {
						let last = match temp.pop() {
							Some(Token::OpenParen) => break,
							Some(t) => t,
							None => return Err(ParsingError::UnmatchedCloseParen),
						};

						res.push(ASTNodeValue::Operator(Operator::try_from(last).unwrap()));
					}

					last_precedence = precedence_stack.pop().unwrap();
					last_was_operand = true;
				},

				t => {
					self.last_token = Some(t);
					break;
				},
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
		assert_eq!(self.last_token, Some(Token::OpenBracket));
		self.last_token = None;

		if self.input.peek() == Some(&Ok(Token::CloseBracket)) {
			self.input.next();
			return Ok(ASTNodeValue::Matrix(vec![]));
		}

		self.is_inside_matrix = true;
		let first = self.parse_expr()?;

		let mut mat = Vec::new();
		let mut row = vec![first];

		while let Some(t) = self.next_token()? {
			match t {
				Token::Comma => {},

				Token::SemiColon => {
					if row.is_empty() {
						continue;
					}

					let cap = row.len();
					mat.push(row);
					row = Vec::with_capacity(cap);
				},

				Token::CloseBracket => {
					if !row.is_empty() {
						mat.push(row);
					}
					self.is_inside_matrix = false; // WARN: we could be inside a child matrix here
					return Ok(ASTNodeValue::Matrix(mat));
				},

				Token::EndOfFile => return Err(ParsingError::IncompleteStatement),

				_ => {
					self.is_inside_matrix = true; // This is here because a child matrix could set this to false
					self.last_token = Some(t);
					let expr = self.parse_expr()?;
					row.push(expr);
				},
			}
		}

		unreachable!("We've somehow passed the EOF token at the end of the tokens array")
	}

	fn next_token(&mut self) -> Result<Option<Token>, ParsingError> {
		if self.last_token.is_some() {
			return Ok(std::mem::take(&mut self.last_token));
		}

		// The ? operator here is turning the `TokenizationError` into `ParsingError::TokenizationError`
		Ok(self.input.next().transpose()?)
	}
}
