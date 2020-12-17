use nom::IResult;

use crate::{
	syntax::{function::Function, variable::VariableAssignment},
	Parse, Result,
};

use super::util::ws;

#[derive(Debug, Clone)]
pub struct Program(Vec<ProgramItem>);

impl Program {
	pub fn new(input: &str) -> Result<Program> {
		use nom::{combinator::map, multi::many1};
		Ok(map(many1(ws(ProgramItem::parse)), |v| Program(v))(input)?.1)
	}

	pub fn items(&self) -> &Vec<ProgramItem> {
		&self.0
	}
}

#[derive(Debug, Clone)]
pub enum ProgramItem {
	VariableAssignment(VariableAssignment),
	Function(Function),
}

impl Parse for ProgramItem {
	fn parse(input: &str) -> IResult<&str, Self> {
		use nom::{branch::alt, bytes::complete::tag, combinator::map};

		let (input, v) = alt((
			map(VariableAssignment::parse, |v| {
				ProgramItem::VariableAssignment(v)
			}),
			map(Function::parse, |v| ProgramItem::Function(v)),
		))(input)?;
		let (input, _) = tag(";")(input)?;
		Ok((input, v))
	}
}
