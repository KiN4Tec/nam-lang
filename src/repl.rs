use color_eyre::eyre::Result;
use crate::state::State;

pub struct Repl {
    on_init: fn(&mut Self) -> Result<()>,
    on_update: fn(&mut Self, String) -> Result<()>,
    on_exit: fn() -> Result<()>,

    pub is_running: bool,
    pub crash_on_error: bool,
    pub state: State,
}

impl Repl {
    pub fn new(
        on_init: fn(&mut Self) -> Result<()>,
        on_update: fn(&mut Self, String) -> Result<()>,
        on_exit: fn() -> Result<()>,

        crash_on_error: bool,
        initial_state: Option<State>,
    ) -> Self {
        let initial_state = match initial_state {
            Some(state) => state,
            None => State::new(),
        };

        Repl {
            on_init,
            on_update,
            on_exit,

            is_running: false,
            crash_on_error,
            state: initial_state,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self._on_init()?;

        while self.is_running {
            self._on_update()?;
        }

        self._on_exit()?;
        Ok(())
    }

    fn _on_init(&mut self) -> Result<()> {
        (self.on_init)(self)?;
        self.is_running = true;
        Ok(())
    }

    fn _on_update(&mut self) -> Result<()> {
        print!("> ");
        std::io::Write::flush(&mut std::io::stdout())?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        match (self.on_update)(self, input) {
            Ok(_) => Ok(()),
            Err(e) => {
                if self.crash_on_error {
                    Err(e)
                } else {
                    eprintln!("{e:?}");
                    Ok(())
                }
            },
        }
    }

    fn _on_exit(&mut self) -> Result<()> {
        (self.on_exit)()?;
        std::io::Write::flush(&mut std::io::stdout())?;
        Ok(())
    }
}
