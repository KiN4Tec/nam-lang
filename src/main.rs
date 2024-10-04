mod repl;

mod token;
mod ast;
mod runtime;

mod lexer;
mod parser;
mod engine;

mod errors;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
	color_eyre::install()?;

	let mut my_repl = repl::Repl::new();
	my_repl.run()
}
