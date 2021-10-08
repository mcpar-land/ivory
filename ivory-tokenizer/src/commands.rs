use std::fmt::Display;

use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{line_ending, multispace0},
	combinator::{eof, map},
	sequence::{delimited, pair, terminated, tuple},
};

use crate::{variable::Variable, Parse};

#[derive(Clone, Debug)]
pub enum Command {
	Variable(Variable),
	StructDefinition, // TODO
}

impl Parse for Command {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			delimited(
				multispace0,
				Variable::parse,
				tuple((multispace0, tag(";"), multispace0)),
			),
			|v| Self::Variable(v),
		)(input)
	}
}

impl Display for Command {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Command::Variable(v) => write!(f, "{}", v),
			Command::StructDefinition => write!(f, "StructDefinition"),
		}
	}
}
