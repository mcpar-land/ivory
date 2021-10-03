use std::collections::HashMap;

use crate::{
	function::{Function, FunctionName},
	itype::Type,
	util::variable_name,
	values::Value,
	variable::VariableName,
	Parse,
};

#[derive(Clone, Debug)]
pub struct StructDefinition {
	pub name: StructName,
	pub values: HashMap<VariableName, StructDefinitionValue>,
	pub functions: HashMap<FunctionName, Function>,
}

#[derive(Clone, Debug)]
pub enum StructDefinitionValue {
	Value(Value),
	Type(Type),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructName(String);

impl Parse for StructName {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, val) = variable_name(input)?;
		Ok((input, Self(val.to_string())))
	}
}
