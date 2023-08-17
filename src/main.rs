mod utils;
mod repl;

mod lexer;
mod ast;

use color_eyre::eyre::Result;
use ast::ASTNode;

fn evaluate(ast: ASTNode) -> f64 {
    match ast {
        ASTNode::Number(n) => n,

        ASTNode::BinaryExpr { lhs, op, rhs } => {
            let res_lhs: f64 = evaluate(*lhs);
            let res_rhs: f64 = evaluate(*rhs);

            use lexer::Token;
            match op {
                Token::OpAdd => res_lhs + res_rhs,
                Token::OpSupstract => res_lhs - res_rhs,
                Token::OpMultiply => res_lhs * res_rhs,
                Token::OpDivide => res_lhs / res_rhs,

                _ => unimplemented!(),
            }
        },

        _ => todo!(),
    }
}

fn on_init() -> Result<()> {
    println!("\nNamLang v0.1");
    Ok(())
}

fn on_update(repl: &mut repl::Repl, input: String) -> Result<()> {
    if input.trim() == "exit" {
        repl.is_running = false;
        return Ok(());
    }

    let tokens = lexer::try_tokenize(input)?;
    let ast = ASTNode::try_from(tokens)?;
    let result = evaluate(ast);

    println!("ans = {result}");

    Ok(())
}

fn on_exit() -> Result<()> {
    println!("Goodbye!");
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut my_repl = repl::Repl::new(on_init, on_update, on_exit, false);
    my_repl.run()
}
