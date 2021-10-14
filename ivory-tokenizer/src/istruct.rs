use std::{collections::HashMap, fmt::Display};

use ivory_expression::Expression;
use nom::{
	branch::alt,
	bytes::complete::{tag, take_while},
	character::{
		complete::{alphanumeric0, multispace0, multispace1, one_of},
		is_alphanumeric,
	},
	combinator::map,
	multi::separated_list0,
	sequence::{delimited, pair, preceded, separated_pair, tuple},
};

use crate::{
	expression::{ExpressionToken, Op},
	itype::Type,
	util::variable_name,
	values::Value,
	variable::VariableName,
	Parse,
};

#[derive(Clone, Debug)]
pub struct StructDefinition {
	pub name: StructName,
	pub values: Vec<(VariableName, StructDefinitionValue)>,
}

impl Parse for StructDefinition {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let name = preceded(pair(tag("struct"), multispace1), StructName::parse);
		let prop = separated_pair(
			VariableName::parse,
			tuple((multispace0, tag(":"), multispace0)),
			StructDefinitionValue::parse,
		);
		map(
			separated_pair(
				name,
				multispace0,
				delimited(
					pair(tag("{"), multispace0),
					separated_list0(tuple((multispace0, tag(","), multispace0)), prop),
					pair(multispace0, tag("}")),
				),
			),
			|(name, values)| Self { name, values },
		)(input)
	}
}

impl Display for StructDefinition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"struct {} {{\n{}}}",
			self.name,
			self.values.iter().fold(String::new(), |s, (name, val)| {
				format!("{}\t{}: {},\n", s, name, val)
			})
		)
	}
}

#[derive(Clone, Debug)]
pub enum StructDefinitionValue {
	Value(Expression<Op, ExpressionToken>),
	Type(Type),
}

impl Parse for StructDefinitionValue {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			map(Type::parse, |t| Self::Type(t)),
			map(Expression::<Op, ExpressionToken>::parse, |v| Self::Value(v)),
		))(input)
	}
}

impl Display for StructDefinitionValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			StructDefinitionValue::Value(v) => write!(f, "{}", v),
			StructDefinitionValue::Type(t) => write!(f, "{}", t),
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructName(pub String);

impl Parse for StructName {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, capital) = one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(input)?;
		let (input, rest_of) = alphanumeric0(input)?;
		Ok((input, Self(format!("{}{}", capital, rest_of))))
	}
}

impl Display for StructName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[cfg(test)]
#[test]
fn test_struct_name() {
	assert!(StructName::parse("ValidStructName").is_ok());
	assert!(StructName::parse("invalid_struct_name").is_err());
}

#[test]
fn test_struct_def() {
	use crate::util::test_multiple;

	test_multiple::<StructDefinition>(&[
		"struct Foo { bar: string, baz: int, qux: int[] }",
		"struct Foo { bar: string, baz: 69, qux: frindle }",
		"struct Bingus { my_beloved: a -> a + 5 }",
	]);
}
