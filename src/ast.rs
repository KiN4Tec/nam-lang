use color_eyre::eyre::Result;
use crate::lexer::Token;
use crate::utils::pop_front;

#[derive(Debug)]
pub enum ASTNode {
    Variable(String),
    RuntimeVal(u32),

    BinaryExpr {
        lhs: Box<ASTNode>,
        op: Token,
        rhs: Box<ASTNode>,
    },
}

impl ASTNode {
    pub fn try_from(mut tokens: Vec<Token>) -> Result<Self> {
        Self::parse_program(&mut tokens)
    }

    fn parse_program(tokens: &mut Vec<Token>) -> Result<Self> {
        Self::parse_stmt(tokens)
    }

    fn parse_stmt(tokens: &mut Vec<Token>) -> Result<Self> {
        Self::parse_expr(tokens)
    }

    fn parse_expr(tokens: &mut Vec<Token>) -> Result<Self> {
        Self::parse_additive_expr(tokens)
    }

    fn parse_additive_expr(tokens: &mut Vec<Token>) -> Result<Self> {
        let mut lhs = Self::parse_multiplicative_expr(tokens)?;

        while tokens[0] == Token::OpAdd || tokens[0] == Token::OpSupstract {
            let op = pop_front(tokens).unwrap_or_else(|| unreachable!());
            let rhs = Self::parse_multiplicative_expr(tokens)?;

            lhs = Self::BinaryExpr {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_multiplicative_expr(tokens: &mut Vec<Token>) -> Result<Self> {
        let mut lhs = Self::parse_primary_expr(tokens)?;

        while tokens[0] == Token::OpMultiply || tokens[0] == Token::OpDivide {
            let op = pop_front(tokens).unwrap_or_else(|| unreachable!());
            let rhs = Self::parse_primary_expr(tokens)?;

            lhs = Self::BinaryExpr {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_primary_expr(tokens: &mut Vec<Token>) -> Result<Self> {
        let token = match pop_front(tokens) {
            Some(token) => token,
            None => {
                return Err(color_eyre::Report::msg(format!(
                    "Unexpected end of vector {tokens:?}"
                )));
            },
        };

        match token {
            Token::Identifier(var_name) => Ok(Self::Variable(var_name)),
            Token::Number(n) => Ok(Self::RuntimeVal(n)),

            _ => todo!(),
        }
    }
}
