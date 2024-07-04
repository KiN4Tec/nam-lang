mod repl;

mod ast;
mod token;

mod engine;
mod lexer;
mod parser;

mod errors;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
	color_eyre::install()?;

	let mut my_repl = repl::Repl::new();
	my_repl.run()
}
