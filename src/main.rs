mod lexer;
mod repl;

use color_eyre::eyre::Result;
use lexer::Token;

#[derive(Debug)]
enum ASTNode {
    Variable(String),
    RuntimeVal(u32),

    BinaryOp {
        lhs: Box<ASTNode>,
        op: Token,
        rhs: Box<ASTNode>,
    },

    None, // ! Remove this
}

impl ASTNode {
    fn new() -> Self {
        Self::None
    }

    fn try_from(mut tokens: Vec<Token>) -> Result<Self> {
        Ok(Self::parse_program(&mut tokens))
    }

    fn parse_program(tokens: &mut Vec<Token>) -> Self {
        Self::parse_stmt(tokens)
    }

    fn parse_stmt(tokens: &mut Vec<Token>) -> Self {
        Self::parse_expr(tokens)
    }

    fn parse_expr(tokens: &mut Vec<Token>) -> Self {
        Self::parse_additive_expr(tokens)
    }

    fn parse_additive_expr(tokens: &mut Vec<Token>) -> Self {
        let mut lhs = Self::parse_multiplicative_expr(tokens);

        while tokens[0] == Token::OpAdd || tokens[0] == Token::OpSupstract {
            tokens.rotate_left(1);
            let op = tokens.pop().unwrap();

            let rhs = Self::parse_multiplicative_expr(tokens);

            lhs = Self::BinaryOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            };
        }

        lhs
    }

    fn parse_multiplicative_expr(tokens: &mut Vec<Token>) -> Self {
        let mut lhs = Self::parse_primary_expr(tokens);

        while tokens[0] == Token::OpMultiply || tokens[0] == Token::OpDivide {
            tokens.rotate_left(1);
            let op = tokens.pop().unwrap();

            let rhs = Self::parse_primary_expr(tokens);

            lhs = Self::BinaryOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            };
        }

        lhs
    }

    fn parse_primary_expr(tokens: &mut Vec<Token>) -> Self {
        tokens.rotate_left(1);
        match tokens.pop().unwrap() {
            Token::Identifier(var_name) => Self::Variable(var_name),
            Token::Number(n) => Self::RuntimeVal(n),

            _ => panic!(),
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

    let tokens = match lexer::try_parse_from(input) {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("{e:?}");
            vec![]
        },
    };

    let _ast = match ASTNode::try_from(tokens) {
        Ok(ast) => dbg!(ast),
        Err(e) => {
            eprintln!("{e:?}");
            ASTNode::new()
        },
    };

    Ok(())
}

fn on_exit() -> Result<()> {
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut my_repl = repl::Repl::new(&on_init, &on_update, &on_exit);
    my_repl.run()?;

    Ok(())
}
