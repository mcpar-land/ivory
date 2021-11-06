use std::fmt::Display;

use ivory_expression::Expression;
use nom::{
	bytes::complete::tag,
	combinator::map,
	multi::separated_list1,
	sequence::{separated_pair, tuple},
};

use crate::{
	expression::{ExpressionToken, Op},
	util::{ws0, ws1},
	variable::VariableName,
	Parse,
};

#[derive(Clone, Debug)]
pub struct FunctionValue {
	pub args: Vec<VariableName>,
	pub expr: Box<Expression<Op, ExpressionToken>>,
}

impl Parse for FunctionValue {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			separated_pair(
				separated_list1(ws1, VariableName::parse),
				tuple((ws0, tag("->"), ws0)),
				Expression::parse,
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
