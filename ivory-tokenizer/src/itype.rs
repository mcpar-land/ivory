use std::{collections::HashMap, fmt::Display};

use nom::{
	branch::alt,
	bytes::complete::tag,
	combinator::{map, value},
	sequence::terminated,
};

use crate::{istruct::StructName, variable::VariableName, Parse};

#[derive(Clone, Debug)]
pub enum Type {
	Integer,
	Decimal,
	Boolean,
	String,
	Struct(StructName),
	Array(Box<Type>),
	Object(HashMap<VariableName, Type>),
}

impl Parse for Type {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			value(Self::Integer, tag("int")),
			value(Self::Decimal, tag("decimal")),
			value(Self::Boolean, tag("bool")),
			value(Self::String, tag("string")),
			map(StructName::parse, |name| Self::Struct(name)),
			map(terminated(Type::parse, tag("[]")), |t| {
				Self::Array(Box::new(t))
			}),
		))(input)
	}
}

impl Display for Type {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

#[derive(Clone, Debug)]
struct ArrayLiteralType(Vec<Type>);

impl Parse for ArrayLiteralType {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		todo!();
	}
}

impl Display for ArrayLiteralType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

#[derive(Clone, Debug)]
struct ObjectLiteralType(Vec<(VariableName, Type)>);
