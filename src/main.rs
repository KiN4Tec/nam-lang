mod repl;
mod lexer;
mod ast;

use color_eyre::eyre::Result;
use ast::ASTNode;

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
