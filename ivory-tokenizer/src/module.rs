use nom::{
	character::complete::multispace0,
	combinator::map,
	multi::many1,
	sequence::{preceded, terminated},
};

use crate::{commands::Command, Parse};

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
