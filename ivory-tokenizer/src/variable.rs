use std::fmt::Display;

use ivory_expression::Expression;
use nom::{
	character::complete::{char, multispace0},
	combinator::map,
	sequence::{separated_pair, tuple},
};

use crate::{
	expression::{ExpressionToken, Op},
	util::variable_name,
	Parse,
};

#[derive(Clone, Debug)]
pub struct Variable {
	pub name: VariableName,
	pub value: Expression<Op, ExpressionToken>,
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

impl Display for Variable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} = {}", self.name, self.value)
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

impl Display for VariableName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[cfg(test)]
#[test]
fn parse_variable() {
	use crate::util::test_multiple;

	test_multiple::<Variable>(&[
		"foo = 69",
		"foo=69",
		"bar = 33 + 5",
		"baz = \"this is a string\"",
		"pythag = a b -> math.sqrt(x*x + y*y)",
	]);
}
