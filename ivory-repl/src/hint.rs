use colored::*;
use ivory_runtime::{mod_loader::ModLoader, runtime::Runtime};
use rand::Rng;
use rustyline::hint::{Hint, Hinter};
use rustyline_derive::{Completer, Helper, Highlighter, Validator};

#[derive(Helper, Completer, Highlighter, Validator)]
pub struct RuntimeHinter<'a, R: Rng, L: ModLoader>(pub &'a Runtime<R, L>);

impl<'a, R: Rng, L: ModLoader> Hinter for RuntimeHinter<'a, R, L> {
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
			self.0.values.variables.iter().find_map(|(key, variable)| {
				if var_name.len() > 0 && key.starts_with(var_name) {
					let skip = var_name.len()..;
					Some(AppHint {
						val: key[skip.clone()].to_string(),
						display: format!("{}", variable)[skip.clone()]
							.bright_black()
							.to_string(),
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
