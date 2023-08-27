use color_eyre::eyre::Result;
use crate::lexer::Token;

#[derive(Debug)]
pub enum ASTNode {
    Variable(String),
    Number(f64),

    BinaryExpr {
        lhs: Box<ASTNode>,
        op: Token,
        rhs: Box<ASTNode>,
    },
}

impl ASTNode {
    pub fn try_from(tokens: &[Token]) -> Result<Self> {
        let (_, res) = Self::parse_program(0, tokens)?;
        Ok(res)
    }

    fn parse_program(idx: usize, tokens: &[Token]) -> Result<(usize, Self)> {
        Self::parse_stmt(idx, tokens)
    }

    fn parse_stmt(idx: usize, tokens: &[Token]) -> Result<(usize, Self)> {
        Self::parse_expr(idx, tokens)
    }

    fn parse_expr(idx: usize, tokens: &[Token]) -> Result<(usize, Self)> {
        Self::parse_additive_expr(idx, tokens)
    }

    fn parse_additive_expr(idx: usize, tokens: &[Token]) -> Result<(usize, Self)> {
        let (mut consumed_len, mut lhs) = Self::parse_multiplicative_expr(idx, tokens)?;

        while let Some(token) = tokens.get(idx + consumed_len) {
            if *token != Token::OpAdd && *token != Token::OpSupstract {
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

    fn parse_multiplicative_expr(idx: usize, tokens: &[Token]) -> Result<(usize, Self)> {
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

    fn parse_parenthesised_expr(idx: usize, tokens: &[Token]) -> Result<(usize, Self)> {
        if let Some(token) = tokens.get(idx) {
            if *token != Token::OpenParen {
                return Self::parse_primary_expr(idx, tokens);
            }
        } else {
            return Err(color_eyre::Report::msg(format!(
                "Unexpected end of array {tokens:?}"
            )));
        }

        let (mut consumed_len, result) = Self::parse_expr(idx + 1, tokens)?;
        consumed_len += 1; // Consuming the Open Paren

        if let Some(token) = tokens.get(idx + consumed_len) {
            if *token != Token::CloseParen {
                return Err(color_eyre::Report::msg(format!(
                    "Expected {:?} but found {:?}",
                    Token::CloseParen,
                    tokens[idx + consumed_len]
                )));
            }
        }
        consumed_len += 1; // Consuming the Close Paren

        Ok((consumed_len, result))
    }

    fn parse_primary_expr(idx: usize, tokens: &[Token]) -> Result<(usize, Self)> {
        let token = match tokens.get(idx) {
            Some(token) => token,
            None => {
                return Err(color_eyre::Report::msg(format!(
                    "Unexpected end of array {tokens:?}"
                )));
            },
        };

        match token {
            Token::Identifier(var_name) => Ok((1, Self::Variable(var_name.clone()))),
            Token::NumericLiteral(n) => Ok((1, Self::Number(*n))),

            _ => todo!(),
        }
    }
}
