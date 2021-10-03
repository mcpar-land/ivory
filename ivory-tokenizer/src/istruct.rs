use std::collections::HashMap;

use nom::{
	bytes::complete::tag,
	character::complete::{multispace0, multispace1},
	combinator::map,
	multi::separated_list0,
	sequence::{delimited, pair, preceded, separated_pair, tuple},
};

use crate::{
	itype::Type, util::variable_name, values::Value, variable::VariableName,
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

#[derive(Clone, Debug)]
pub enum StructDefinitionValue {
	Value(Value),
	Type(Type),
}

impl Parse for StructDefinitionValue {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		todo!()
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructName(String);

impl Parse for StructName {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, val) = variable_name(input)?;
		Ok((input, Self(val.to_string())))
	}
}
