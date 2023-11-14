mod repl;
mod state;

mod lexer;
mod ast;
mod eval;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut my_repl = repl::Repl::new();
    my_repl.run()
}
