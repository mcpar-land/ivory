use ivory_tokenizer::variable::VariableName;
use rustyline::hint::{Hint, Hinter};

use crate::App;

impl Hinter for App {
	type Hint = AppHint;

	fn hint(
		&self,
		line: &str,
		pos: usize,
		ctx: &rustyline::Context<'_>,
	) -> Option<Self::Hint> {
		let _ = (line, pos, ctx);
		None
	}
}

pub enum AppHint {
	Variable(String),
}

impl Hint for AppHint {
	fn display(&self) -> &str {
		match self {
			AppHint::Variable(s) => s.as_str(),
		}
	}

	fn completion(&self) -> Option<&str> {
		Some(self.display())
	}
}
