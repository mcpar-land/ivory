use std::fmt::Display;

use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{line_ending, multispace0},
	combinator::{eof, map},
	sequence::{delimited, pair, terminated, tuple},
};

use crate::{module::iuse::Use, variable::Variable, Parse};

#[derive(Clone, Debug)]
pub enum Command {
	Use(Use),
	Variable(Variable),
	StructDefinition, // TODO
}

impl Parse for Command {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		delimited(
			multispace0,
			alt((
				map(Variable::parse, |v| Self::Variable(v)),
				map(Use::parse, |v| Self::Use(v)),
			)),
			tuple((multispace0, tag(";"), multispace0)),
		)(input)
	}
}

impl Display for Command {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Command::Variable(v) => write!(f, "{}", v),
			Command::StructDefinition => write!(f, "StructDefinition"),
			Command::Use(u) => write!(f, "{}", u),
		}
	}
}
