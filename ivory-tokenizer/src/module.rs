use std::fmt::Display;

use nom::{
	character::complete::multispace0,
	combinator::map,
	multi::many1,
	sequence::{preceded, terminated},
};

use crate::{commands::Command, Parse};

pub mod iuse;

#[derive(Clone, Debug)]
pub struct Module(pub Vec<Command>);

impl Parse for Module {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			preceded(multispace0, many1(terminated(Command::parse, multispace0))),
			|v| Self(v),
		)(input)
	}
}

impl Display for Module {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			self
				.0
				.iter()
				.fold(String::new(), |s, val| { format!("{}\n{}", s, val) })
		)
	}
}
