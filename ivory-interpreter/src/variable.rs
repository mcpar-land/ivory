use nom::{
	character::complete::{char, multispace0, multispace1},
	combinator::map,
	multi::many0,
	sequence::{separated_pair, terminated, tuple},
};

use crate::{expression::Expression, util::variable_name, Parse};

#[derive(Clone, Debug)]
pub struct Variable {
	pub def: VariableDefinition,
	pub value: Expression,
}

impl Parse for Variable {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			separated_pair(
				VariableDefinition::parse,
				tuple((multispace0, char('='), multispace0)),
				Expression::parse,
			),
			|(name, value)| Self { def: name, value },
		)(input)
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariableDefinition {
	pub name: VariableName,
	pub args: Vec<VariableName>,
}

impl Parse for VariableDefinition {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			separated_pair(
				VariableName::parse,
				multispace1,
				many0(terminated(VariableName::parse, multispace1)),
			),
			|(name, args)| Self { name, args },
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

#[cfg(test)]
#[test]
fn parse_variable() {
	use crate::util::test_multiple;

	test_multiple::<Variable>(&[
		"foo = 69",
		"bar = 33 + 5",
		"baz = \"this is a string\"",
		"function_example x y = math.sqrt(x*x + y*y)",
	]);
}
