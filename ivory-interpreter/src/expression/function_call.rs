use nom::{
	character::complete::{char, multispace0},
	combinator::map,
	multi::separated_list0,
	sequence::{delimited, pair, separated_pair},
};

use crate::{function::FunctionName, Parse};

use super::Expression;

#[derive(Clone, Debug)]
pub struct FunctionCall {
	pub name: FunctionName,
	pub params: Vec<Expression>,
}

impl Parse for FunctionCall {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			separated_pair(
				FunctionName::parse,
				multispace0,
				delimited(
					pair(char('('), multispace0),
					separated_list0(
						delimited(multispace0, char(','), multispace0),
						Expression::parse,
					),
					pair(multispace0, char(')')),
				),
			),
			|(name, params)| FunctionCall { name, params },
		)(input)
	}
}

#[cfg(test)]
#[test]
fn parse_function_call() {
	use crate::util::test_multiple;

	test_multiple::<FunctionCall>(&[
		"foobar()",
		"fizzlord(123,\n456\n,biggler)",
		"square_root(5 * 5)",
	]);
}
