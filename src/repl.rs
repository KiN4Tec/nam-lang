use {
	crate::{engine::Engine, lexer::Lexer, parser::Parser},
	anyhow::Result,
	reedline::Signal,
};

pub struct Repl {
	is_running: bool,
	engine: Engine,
}

impl Repl {
	pub fn new() -> Self {
		Repl {
			is_running: false,
			engine: Engine::new(),
		}
	}

	pub fn run(&mut self) -> Result<()> {
		let mut line_editor = reedline::Reedline::create();
		let prompt = Prompt::default();

		println!("\nNamLang v{}", env!("CARGO_PKG_VERSION"));

		self.is_running = true;
		while self.is_running {
			let sig = line_editor.read_line(&prompt)?;
			match sig {
				Signal::CtrlD => break,
				Signal::CtrlC => continue,
				Signal::Success(input) => {
					let r = self.on_prompt(input);
					if let Err(e) = r {
						eprintln!("\n{e}");
					}
				},
			}
		}

		println!("\nGoodbye!");
		std::io::Write::flush(&mut std::io::stdout())?;
		Ok(())
	}

	fn on_prompt(&mut self, input: String) -> Result<()> {
		if input.trim() == "exit" {
			self.is_running = false;
			return Ok(());
		}

		if input.trim().is_empty() {
			return Ok(());
		}

		let lexer = Lexer::new(input.chars());
		let mut parser = Parser::new(lexer);
		self.engine.evaluate(parser.parse()?)?;

		Ok(())
	}
}

#[derive(Default)]
struct Prompt {}

impl reedline::Prompt for Prompt {
	fn render_prompt_left(&'_ self) -> std::borrow::Cow<'_, str> {
		std::borrow::Cow::Borrowed(" \nnam")
	}

	fn render_prompt_right(&'_ self) -> std::borrow::Cow<'_, str> {
		std::borrow::Cow::Borrowed("")
	}

	fn render_prompt_indicator(
		&'_ self,
		_prompt_mode: reedline::PromptEditMode,
	) -> std::borrow::Cow<'_, str> {
		std::borrow::Cow::Borrowed(" > ")
	}

	fn render_prompt_multiline_indicator(&'_ self) -> std::borrow::Cow<'_, str> {
		std::borrow::Cow::Borrowed(" > ")
	}

	fn render_prompt_history_search_indicator(
		&'_ self,
		_history_search: reedline::PromptHistorySearch,
	) -> std::borrow::Cow<'_, str> {
		std::borrow::Cow::Borrowed(" > ")
	}
}
