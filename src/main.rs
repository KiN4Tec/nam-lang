mod repl;

mod token;
mod ast;
mod runtime;
mod matrix;
mod scalar;

mod lexer;
mod parser;
mod engine;

mod errors;
use anyhow::Result;

fn main() -> Result<()> {
	let mut my_repl = repl::Repl::new();
	my_repl.run()
}
