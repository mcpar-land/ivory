use nom::{
	character::complete::{char, multispace0},
	combinator::map,
	sequence::{separated_pair, tuple},
};

use crate::{expression::Expression, util::variable_name, Parse};

#[derive(Clone, Debug)]
pub struct Variable {
	pub name: VariableName,
	pub value: Expression,
}

impl Parse for Variable {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			separated_pair(
				VariableName::parse,
				tuple((multispace0, char('='), multispace0)),
				Expression::parse,
			),
			|(name, value)| Self { name, value },
		)(input)
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariableName(pub String);

impl Parse for VariableName {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, val) = variable_name(input)?;
		Ok((input, Self(val.to_string())))
	}
}
