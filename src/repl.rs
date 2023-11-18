use {
	crate::{ast, eval, lexer, state::State},
	color_eyre::eyre::Result,
	reedline::Signal,
};

pub struct Repl {
	pub is_running: bool,
	pub state: State,
}

impl Repl {
	pub fn new() -> Self {
		Repl {
			is_running: false,
			state: State::new(),
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
						eprintln!("{e:?}");
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

		// Unfortunately, at the time of writing, Rust does not seem to support direct conversion
		// from a String to an array of characters, once it have it, this should be changed.
		let code: Vec<char> = input.chars().collect();

		let tokens = lexer::try_tokenize(code.as_ref())?;
		let ast = ast::ASTNode::try_from(&tokens)?;
		eval::evaluate(ast, &mut self.state)?;

		Ok(())
	}
}

#[derive(Default)]
pub struct Prompt {}

impl reedline::Prompt for Prompt {
	fn render_prompt_left(&self) -> std::borrow::Cow<str> {
		std::borrow::Cow::Borrowed(" \nnam")
	}

	fn render_prompt_right(&self) -> std::borrow::Cow<str> {
		std::borrow::Cow::Borrowed("")
	}

	fn render_prompt_indicator(
		&self,
		_prompt_mode: reedline::PromptEditMode,
	) -> std::borrow::Cow<str> {
		std::borrow::Cow::Borrowed(" > ")
	}

	fn render_prompt_multiline_indicator(&self) -> std::borrow::Cow<str> {
		std::borrow::Cow::Borrowed(" > ")
	}

	fn render_prompt_history_search_indicator(
		&self,
		_history_search: reedline::PromptHistorySearch,
	) -> std::borrow::Cow<str> {
		std::borrow::Cow::Borrowed(" > ")
	}
}
