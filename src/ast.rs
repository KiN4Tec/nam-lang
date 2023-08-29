use color_eyre::eyre::Result;
use crate::lexer::Token;

#[derive(Debug)]
pub enum ASTNode {
    Variable(String),
    Number(f64),

    Assignment(String, Box<Self>),

    BinaryExpr {
        lhs: Box<Self>,
        op: Token,
        rhs: Box<Self>,
    },
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
        if let Some(var_token) = tokens.get(idx) {
            if let Some(equals_token) = tokens.get(idx + 1) {
                if let Token::Identifier(var_name) = var_token {
                    if Token::OpAssign == *equals_token {
                        // x = (expession)
                        let (consumed_len, var_value) = Self::parse_expr(idx + 2, tokens)?;
                        return Ok((
                            consumed_len + 2,
                            Self::Assignment(var_name.clone(), Box::new(var_value)),
                        ));
                    }
                }
            }
        }

        Self::parse_expr(idx, tokens)
    }

    fn parse_expr(idx: usize, tokens: &[Token]) -> Result<(usize, Self), ParsingError> {
        Self::parse_additive_expr(idx, tokens)
    }

    fn parse_additive_expr(idx: usize, tokens: &[Token]) -> Result<(usize, Self), ParsingError> {
        let (mut consumed_len, mut lhs) = Self::parse_multiplicative_expr(idx, tokens)?;

        while let Some(token) = tokens.get(idx + consumed_len) {
            if *token != Token::OpAdd && *token != Token::OpSuptract {
                break;
            }

            let op = token.clone();
            consumed_len += 1;

            let (consumed_rhs, rhs) = Self::parse_multiplicative_expr(idx + consumed_len, tokens)?;
            consumed_len += consumed_rhs;

            lhs = Self::BinaryExpr {
                lhs: Box::new(lhs),
                op,
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

            let op = token.clone();
            consumed_len += 1;

            let (consumed_rhs, rhs) = Self::parse_parenthesised_expr(idx + consumed_len, tokens)?;
            consumed_len += consumed_rhs;

            lhs = Self::BinaryExpr {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok((consumed_len, lhs))
    }

    fn parse_parenthesised_expr(
        idx: usize,
        tokens: &[Token],
    ) -> Result<(usize, Self), ParsingError> {
        if let Some(token) = tokens.get(idx) {
            if *token != Token::OpenParen {
                return Self::parse_primary_expr(idx, tokens);
            }
        } else {
            return Err(ParsingError::UnexpectedEndOfInput);
        }

        let (mut consumed_len, result) = Self::parse_expr(idx + 1, tokens)?;
        consumed_len += 1; // Consuming the Open Paren

        if let Some(token) = tokens.get(idx + consumed_len) {
            if *token != Token::CloseParen {
                return Err(ParsingError::UnexpectedToken {
                    expected: Some(format!("{:?}", Token::CloseParen)),
                    found: Some(format!("{:?}", tokens[idx + consumed_len])),
                });
            }
        }
        consumed_len += 1; // Consuming the Close Paren

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
            Self::UnexpectedEndOfInput => write!(f, "Unexpected end of input array"),
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
