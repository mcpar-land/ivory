use std::fmt::Display;

use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{line_ending, multispace0, space0},
	combinator::{eof, map},
	sequence::{pair, terminated},
};

use crate::{
	istruct::StructDefinition, module::iuse::Use, variable::Variable, Parse,
};

#[derive(Clone, Debug)]
pub enum Command {
	Use(Use),
	Variable(Variable),
	StructDefinition(StructDefinition),
}

impl Parse for Command {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			terminated(
				map(Variable::parse, |v| Self::Variable(v)),
				pair(multispace0, tag(";")),
			),
			terminated(
				map(Use::parse, |v| Self::Use(v)),
				pair(multispace0, tag(";")),
			),
			terminated(
				map(StructDefinition::parse, |v| Self::StructDefinition(v)),
				pair(space0, alt((line_ending, eof))),
			),
		))(input)
	}
}

impl Display for Command {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Command::Variable(v) => write!(f, "{};", v),
			Command::StructDefinition(d) => write!(f, "{}", d),
			Command::Use(u) => write!(f, "{};", u),
		}
	}
}

#[cfg(test)]
#[test]
fn parse_command() {
	crate::util::test_multiple::<Command>(&[
		"x = y + z;",
		"use * from \"http://fakewebsite.biz/source.ivory\";",
		"struct Foo { array_2d: int[][] }",
	]);
}
