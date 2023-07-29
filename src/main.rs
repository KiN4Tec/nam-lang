mod lexer;

use color_eyre::eyre::Result;
use std::io::Write;

struct Repl<'a> {
    on_init: &'a dyn Fn() -> Result<()>,
    on_update: &'a dyn Fn(&mut Self, String) -> Result<()>,
    _is_running: bool,
}

impl<'a> Repl<'a> {
    pub fn new(
        on_init: &'a impl Fn() -> Result<()>,
        on_update: &'a impl Fn(&mut Self, String) -> Result<()>,
    ) -> Self {
        Repl {
            on_init,
            on_update,
            _is_running: false,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self._on_init()?;

        while self._is_running {
            self._on_update()?;
        }

        Ok(())
    }

    fn _on_init(&mut self) -> Result<()> {
        (self.on_init)()?;
        self._is_running = true;
        Ok(())
    }

    fn _on_update(&mut self) -> Result<()> {
        print!("> ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        // _on_exit ()
        if input.trim() == "exit" {
            println!("Goodbye!");
            self._is_running = false;
            return Ok(());
        }

        (self.on_update)(self, input)?;
        Ok(())
    }
}

fn on_init() -> Result<()> {
    println!("\nConVector v0.0");
    Ok(())
}

fn on_update(_repl: &mut Repl, input: String) -> Result<()> {
    match lexer::try_parse_from(input) {
        Ok(tokens) => dbg!(tokens),
        Err(e) => {
            eprintln!("{e:?}");
            vec![]
        },
    };
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut my_repl = Repl::new(&on_init, &on_update);
    my_repl.run()?;

    Ok(())
}
