use std::{collections::HashMap, fmt::Display};

use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::multispace0,
	combinator::{map, value},
	multi::separated_list1,
	sequence::{delimited, pair, separated_pair, terminated, tuple},
};

use crate::{
	istruct::StructName, util::comma_separated_display, variable::VariableName,
	Parse,
};

#[derive(Clone, Debug)]
pub enum Type {
	Any,
	Integer,
	Decimal,
	Boolean,
	String,
	Roll,
	Struct(StructName),
	Array(Box<Type>),
	Object,
}

impl Parse for Type {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			value(Self::Any, tag("any")),
			value(Self::Integer, tag("int")),
			value(Self::Decimal, tag("decimal")),
			value(Self::Boolean, tag("bool")),
			value(Self::Roll, tag("roll")),
			value(Self::String, tag("string")),
			value(Self::Object, tag("object")),
			map(StructName::parse, |name| Self::Struct(name)),
			map(terminated(Type::parse, tag("[]")), |t| {
				Self::Array(Box::new(t))
			}),
		))(input)
	}
}

impl Display for Type {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Type::Any => write!(f, "any"),
			Type::Integer => write!(f, "integer"),
			Type::Decimal => write!(f, "decimal"),
			Type::Boolean => write!(f, "bool"),
			Type::Roll => write!(f, "roll"),
			Type::String => write!(f, "string"),
			Type::Struct(name) => write!(f, "{}", name),
			Type::Array(t) => write!(f, "{}[]", t),
			Type::Object => write!(f, "object"),
		}
	}
}

#[derive(Clone, Debug)]
enum ArrayType {
	Single(Box<Type>),
	Literal(ArrayLiteralType),
}

impl Parse for ArrayType {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			map(terminated(Type::parse, tag("[]")), |t| {
				Self::Single(Box::new(t))
			}),
			map(ArrayLiteralType::parse, |t| Self::Literal(t)),
		))(input)
	}
}

impl Display for ArrayType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ArrayType::Single(t) => write!(f, "{}", t),
			ArrayType::Literal(t) => write!(f, "{}", t),
		}
	}
}

#[derive(Clone, Debug)]
struct ArrayLiteralType(Vec<Type>);

impl Parse for ArrayLiteralType {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			separated_list1(tuple((multispace0, tag(","), multispace0)), Type::parse),
			|types| Self(types),
		)(input)
	}
}

impl Display for ArrayLiteralType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}]", comma_separated_display(&self.0))
	}
}

#[derive(Clone, Debug)]
struct ObjectLiteralType(HashMap<VariableName, Type>);

impl Parse for ObjectLiteralType {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			delimited(
				pair(tag("{"), multispace0),
				separated_list1(
					tuple((multispace0, tag(","), multispace0)),
					separated_pair(
						VariableName::parse,
						tuple((multispace0, tag(","), multispace0)),
						Type::parse,
					),
				),
				pair(multispace0, tag("}")),
			),
			|pairs| {
				let mut map = HashMap::new();
				for (name, ty) in pairs.into_iter() {
					map.insert(name, ty);
				}
				Self(map)
			},
		)(input)
	}
}

impl Display for ObjectLiteralType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{{object literal}}")
	}
}
