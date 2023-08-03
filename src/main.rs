mod lexer;
mod repl;

use color_eyre::eyre::Result;

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

    let _tokens = match lexer::try_parse_from(input) {
        Ok(tokens) => dbg!(tokens),
        Err(e) => {
            eprintln!("{e:?}");
            vec![]
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
