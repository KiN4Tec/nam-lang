use color_eyre::eyre::Result;

pub struct Repl<'a> {
    on_init: &'a dyn Fn() -> Result<()>,
    on_update: &'a dyn Fn(&mut Self, String) -> Result<()>,
    on_exit: &'a dyn Fn() -> Result<()>,
    pub is_running: bool,
}

impl<'a> Repl<'a> {
    pub fn new(
        on_init: &'a impl Fn() -> Result<()>,
        on_update: &'a impl Fn(&mut Self, String) -> Result<()>,
        on_exit: &'a impl Fn() -> Result<()>,
    ) -> Self {
        Repl {
            on_init,
            on_update,
            on_exit,
            is_running: false,
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
        (self.on_init)()?;
        self.is_running = true;
        Ok(())
    }

    fn _on_update(&mut self) -> Result<()> {
        print!("> ");
        std::io::Write::flush(&mut std::io::stdout())?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        (self.on_update)(self, input)?;
        Ok(())
    }

    fn _on_exit(&mut self) -> Result<()>
    {
        (self.on_exit)()?;
        std::io::Write::flush(&mut std::io::stdout())?;
        Ok(())
    }
}
