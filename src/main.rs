mod lexer;

use color_eyre::eyre::Result;
use std::io::Write;

fn main() -> Result<()> {
    color_eyre::install()?;

    // init()
    {
        println!("\nConVector v0.0");
    }

    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim() == "exit" {
            println!("Goodbye!");
            break;
        }

        let tokens = match lexer::try_parse_from(input) {
            Ok(tokens) => tokens,
            Err(e) => {
                eprintln!("{e:?}");
                continue;
            }
        };
        dbg!(tokens);
    }

    Ok(())
}
