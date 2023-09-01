mod repl;
mod state;

mod lexer;
mod ast;
mod eval;

use color_eyre::eyre::Result;

fn on_init(_repl: &mut repl::Repl) -> Result<()> {
    println!("\nNamLang v{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}

fn on_update(repl: &mut repl::Repl, input: String) -> Result<()> {
    if input.trim() == "exit" {
        repl.is_running = false;
        return Ok(());
    }

    // Unfortunately, at the time of writing, Rust does not seem to support direct conversion
    // from a String to an array of characters, once it have it, this should be changed.
    let code: Vec<char> = input.chars().collect();

    let tokens = lexer::try_tokenize(code.as_ref())?;
    let ast = ast::ASTNode::try_from(&tokens)?;
    let result = eval::evaluate(ast, &mut repl.state)?;

    println!("ans = {result}");

    Ok(())
}

fn on_exit() -> Result<()> {
    println!("Goodbye!");
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut my_repl = repl::Repl::new(on_init, on_update, on_exit, false, None);
    my_repl.run()
}
