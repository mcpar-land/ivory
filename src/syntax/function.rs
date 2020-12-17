use crate::{
	syntax::{expression::Expression, util::identifier},
	Parse,
};
use nom::{
	bytes::complete::tag,
	character::complete::multispace1,
	combinator::{map, opt},
	multi::separated_list0,
	IResult,
};
use serde::{Deserialize, Serialize};

use super::{
	dice::Dice,
	util::{paren, ws},
};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Function {
	pub name: String,
	pub arguments: Vec<String>,
	pub expression: Expression<Dice>,
}

impl Parse for Function {
	fn parse(input: &str) -> IResult<&str, Self> {
		let (input, name) = map(identifier, |v| String::from(v))(input)?;

		let (input, _) = opt(multispace1)(input)?;
		let (input, arguments) =
			map(separated_list0(multispace1, identifier), |v| {
				v.iter().map(|v| String::from(*v)).collect()
			})(input)?;
		let (input, _) = ws(tag(":"))(input)?;
		let (input, expression) = Expression::<Dice>::parse(input)?;
		Ok((
			input,
			Function {
				name,
				arguments,
				expression,
			},
		))
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionCall {
	pub name: String,
	pub arguments: Vec<Expression<Dice>>,
}

impl FunctionCall {
	pub fn new(name: &str, arguments: Vec<Expression<Dice>>) -> Self {
		FunctionCall {
			name: name.to_string(),
			arguments,
		}
	}
}

impl Parse for FunctionCall {
	fn parse(input: &str) -> IResult<&str, Self> {
		let (input, name) = map(identifier, |v| String::from(v))(input)?;
		let (input, _) = opt(multispace1)(input)?;
		let (input, arguments) = map(
			opt(paren(separated_list0(
				ws(tag(",")),
				Expression::<Dice>::parse,
			))),
			|v: Option<Vec<Expression<Dice>>>| v.unwrap_or(vec![]),
		)(input)?;

		Ok((input, FunctionCall { name, arguments }))
	}
}

#[cfg(test)]
mod test {
	use crate::syntax::{expression::ExpressionItem, number::Number};

	use super::*;

	#[test]
	fn test_function_def() {
		let func = Function::parse("test_function arg1 arg2: val1 + val2")
			.unwrap()
			.1;
		assert_eq!(func.name, "test_function");
		assert_eq!(func.arguments, vec!["arg1", "arg2"]);
	}

	#[test]
	fn test_function_call() {
		assert_eq!(
			FunctionCall::parse("test_function(10, -888)").unwrap().1,
			FunctionCall::new(
				"test_function",
				vec![
					Expression::new(ExpressionItem::Number(Number(10.0)), vec![]),
					Expression::new(ExpressionItem::Number(Number(-888.0)), vec![])
				]
			)
		);
		assert_eq!(
			FunctionCall::parse("test_function()").unwrap().1,
			FunctionCall::new("test_function", vec![])
		);
		assert_eq!(
			FunctionCall::parse("test_function").unwrap().1,
			FunctionCall::new("test_function", vec![])
		);
	}
}
