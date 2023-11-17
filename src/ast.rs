use color_eyre::eyre::Result;
use crate::lexer::Token;

#[derive(Debug)]
pub enum ASTNode {
    Variable(String),
    Number(f64),

    Assignment(String, Box<Self>),

    BinaryExpr {
        lhs: Box<Self>,
        op: BinaryOpKind,
        rhs: Box<Self>,
    },
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

impl ASTNode {
    pub fn try_from(tokens: &[Token]) -> Result<Self, ParsingError> {
        let (_, res) = Self::parse_program(0, tokens)?;
        Ok(res)
    }

    fn parse_program(idx: usize, tokens: &[Token]) -> Result<(usize, Self), ParsingError> {
        Self::parse_stmt(idx, tokens)
    }

    fn parse_stmt(idx: usize, tokens: &[Token]) -> Result<(usize, Self), ParsingError> {
        let (res_len, res) = Self::parse_expr(idx, tokens)?;

        match tokens.get(idx + res_len) {
            Some(Token::EndOfFile) => Ok((res_len, res)),
            Some(token) => Err(ParsingError::UnexpectedToken {
                expected: Some(Token::EndOfFile.stringify()),
                found: Some(token.stringify()),
            }),
            None => unreachable!(),
        }
    }

    fn parse_expr(idx: usize, tokens: &[Token]) -> Result<(usize, Self), ParsingError> {
        Self::parse_assignment_expr(idx, tokens)
    }

    fn parse_assignment_expr(idx: usize, tokens: &[Token]) -> Result<(usize, Self), ParsingError> {
        let (primary_len, primary) = Self::parse_additive_expr(idx, tokens)?;

        // Assignment Statement (x = 5)
        if let Self::Variable(lhs) = &primary {
            if tokens.get(idx + primary_len) == Some(&Token::OpAssign) {
                let (rhs_len, rhs) = Self::parse_expr(idx + primary_len + 1, tokens)?;
                return Ok((
                    primary_len + 1 + rhs_len,
                    Self::Assignment(lhs.to_string(), Box::new(rhs)),
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

            lhs = Self::BinaryExpr {
                lhs: Box::new(lhs),
                op: token.try_into()?,
                rhs: Box::new(rhs),
            };
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

            lhs = Self::BinaryExpr {
                lhs: Box::new(lhs),
                op: token.try_into()?,
                rhs: Box::new(rhs),
            };
        }

        Ok((consumed_len, lhs))
    }

    fn parse_parenthesised_expr(
        idx: usize,
        tokens: &[Token],
    ) -> Result<(usize, Self), ParsingError> {
        // Skip if not an OpenParen
        match tokens.get(idx) {
            Some(Token::OpenParen) => {},

            None => return Err(ParsingError::UnexpectedEndOfInput),
            Some(_) => return Self::parse_primary_expr(idx, tokens),
        };

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

        match token {
            Token::Identifier(var_name) => Ok((1, Self::Variable(var_name.clone()))),
            Token::NumericLiteral(n) => Ok((1, Self::Number(*n))),

            token => Err(ParsingError::UnexpectedToken {
                expected: Some("Expression".to_string()),
                found: Some(format!("{token:?}")),
            }),
        }
    }
}

////////////////////////////////
//       Error Handling       //
////////////////////////////////

#[derive(Debug)]
pub enum ParsingError {
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
