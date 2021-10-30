use std::fmt::Display;

use ivory_expression::{Expression, TernaryExpression};
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{multispace0, multispace1},
	combinator::{map, value},
	multi::{many0, many1, separated_list1},
	sequence::{preceded, separated_pair, tuple},
};

use crate::{
	expression::{ExpressionToken, Op},
	variable::VariableName,
	Parse,
};

#[derive(Clone, Debug)]
pub struct FunctionValue {
	pub args: Vec<VariableName>,
	pub expr: Box<TernaryExpression<Op, ExpressionToken>>,
}

impl Parse for FunctionValue {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			separated_pair(
				separated_list1(multispace1, VariableName::parse),
				tuple((multispace0, tag("->"), multispace0)),
				TernaryExpression::parse,
			),
			|(args, expr)| Self {
				args,
				expr: Box::new(expr),
			},
		)(input)
	}
}

impl Display for FunctionValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// TODO
		write!(f, "function()")
	}
}

#[cfg(test)]
#[test]
fn parse_function_value() {
	use crate::util::test_multiple;

	test_multiple::<FunctionValue>(&[
		"a b -> math.sqrt( a*a + b*b )",
		"woomy spang whammo -> woomy + spang + whammo",
	]);
}
