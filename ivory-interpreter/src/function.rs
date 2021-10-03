use nom::{
	bytes::complete::tag,
	character::complete::{char, multispace0},
	multi::separated_list1,
	sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
};

use crate::{
	expression::Expression, util::variable_name, variable::VariableName, Parse,
};

#[derive(Clone, Debug)]
pub struct Function {
	pub name: FunctionName,
	pub parameters: Vec<VariableName>,
	pub expr: Expression,
}

impl Parse for Function {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, name) = terminated(FunctionName::parse, multispace0)(input)?;
		let (input, parameters) = terminated(
			delimited(
				char('('),
				separated_list1(
					tuple((multispace0, char(','), multispace0)),
					VariableName::parse,
				),
				char(')'),
			),
			multispace0,
		)(input)?;
		let (input, expr) =
			preceded(pair(tag("->"), multispace0), Expression::parse)(input)?;

		Ok((
			input,
			Function {
				name,
				parameters,
				expr,
			},
		))
	}
}

#[derive(Clone, Debug)]
pub struct FunctionName(pub String);

impl Parse for FunctionName {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, val) = variable_name(input)?;
		Ok((input, Self(val.to_string())))
	}
}
