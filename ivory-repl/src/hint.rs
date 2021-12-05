use colored::*;
use ivory_runtime::runtime::Runtime;
use rustyline::hint::{Hint, Hinter};
use rustyline_derive::{Completer, Helper, Highlighter, Validator};

#[derive(Helper, Completer, Highlighter, Validator)]
pub struct RuntimeHinter<'a>(pub &'a Runtime);

impl<'a> Hinter for RuntimeHinter<'a> {
	type Hint = AppHint;

	fn hint(
		&self,
		line: &str,
		pos: usize,
		_: &rustyline::Context<'_>,
	) -> Option<Self::Hint> {
		if line.is_empty() || pos < line.len() {
			None
		} else {
			let mut start_of_variable = 0;
			for (i, c) in line.char_indices().rev() {
				if !"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_1234567890"
					.contains(c)
				{
					start_of_variable = i + 1;
					break;
				}
			}
			let var_name = &line[start_of_variable..];
			self.0.values.variable_names().into_iter().find_map(|key| {
				if var_name.len() > 0 && key.starts_with(var_name) {
					let skip = var_name.len()..;
					let val = self
						.0
						.values
						.get_variable(&key)
						.map(|v| v.to_string())
						.unwrap_or("Def not found!".to_string());
					Some(AppHint {
						val: key[skip.clone()].to_string(),
						display: val[skip.clone()].bright_black().to_string(),
					})
				} else {
					None
				}
			})
		}
	}
}

pub struct AppHint {
	val: String,
	display: String,
}

impl Hint for AppHint {
	fn display(&self) -> &str {
		self.display.as_str()
	}

	fn completion(&self) -> Option<&str> {
		Some(self.val.as_str())
	}
}
