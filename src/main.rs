mod lexer;
mod repl;

use color_eyre::eyre::Result;
use lexer::Token;

#[derive(Debug)]
enum ASTNode {
    Variable(String),
    RuntimeVal(u32),

    BinaryExpr {
        lhs: Box<ASTNode>,
        op: Token,
        rhs: Box<ASTNode>,
    },
}

impl ASTNode {
    fn try_from(mut tokens: Vec<Token>) -> Result<Self> {
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
            tokens.rotate_left(1);
            let op = tokens.pop().unwrap_or_else(|| unreachable!());

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
            tokens.rotate_left(1);
            let op = tokens.pop().unwrap_or_else(|| unreachable!());

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
        tokens.rotate_left(1);
        let token = match tokens.pop() {
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

fn on_init() -> Result<()> {
    println!("\nConVector v0.0");
    Ok(())
}

fn on_update(repl: &mut repl::Repl, input: String) -> Result<()> {
    if input.trim() == "exit" {
        println!("Goodbye!");
        repl.is_running = false;
        return Ok(());
    }

    let tokens = lexer::try_tokenize(input)?;
    let ast = ASTNode::try_from(tokens)?;

    dbg!(ast);

    Ok(())
}

fn on_exit() -> Result<()> {
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut my_repl = repl::Repl::new(&on_init, &on_update, &on_exit, false);
    my_repl.run()?;

    Ok(())
}
