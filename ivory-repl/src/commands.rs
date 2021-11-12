use nom::{
	bytes::complete::tag,
	character::complete::{alpha1, space0},
	combinator::{map, rest},
	sequence::{pair, separated_pair},
	Finish,
};

use crate::error::ReplError;

pub fn commands(
) -> &'static [(&'static str, fn(&str) -> Result<String, ReplError>)] {
	&[("set", set), ("load", load), ("unload", unload)]
}

pub fn set(arg: &str) -> Result<String, ReplError> {
	todo!();
}

pub fn load(arg: &str) -> Result<String, ReplError> {
	todo!();
}

pub fn unload(arg: &str) -> Result<String, ReplError> {
	todo!();
}

struct CommandCall {
	name: String,
	arg: String,
}

impl CommandCall {
	pub fn parse_run(input: &str) -> Result<String, ReplError> {
		CommandCall::parse(input)?.run()
	}

	pub fn parse(input: &str) -> Result<Self, ReplError> {
		map(
			separated_pair(alpha1, pair(tag(":"), space0), rest),
			|(name, arg): (&str, &str)| Self {
				name: name.to_string(),
				arg: arg.to_string(),
			},
		)(input)
		.finish()
		.map_err(|err: nom::error::Error<&str>| {
			ReplError::CommandParsingError(err.to_string())
		})
		.map(|(_, cmd)| cmd)
	}
	pub fn run(&self) -> Result<String, ReplError> {
		for (name, f) in commands() {
			if name == &self.name {
				return f(&self.arg);
			}
		}
		Err(ReplError::CommandNotFound(self.name.clone()))
	}
}
