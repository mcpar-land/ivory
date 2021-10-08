use std::fmt::Display;

use nom::{
	branch::alt,
	combinator::{map, value},
};

use crate::Parse;

use self::{
	array::ArrayValue, boolean::BooleanValue, decimal::DecimalValue,
	function::FunctionValue, integer::IntegerValue, object::ObjectValue,
	string::StringValue, struct_instance::StructInstance,
};

pub mod array;
pub mod boolean;
pub mod decimal;
pub mod function;
pub mod integer;
pub mod object;
pub mod string;
pub mod struct_instance;

#[derive(Clone, Debug)]
pub enum Value {
	Boolean(BooleanValue),
	Decimal(DecimalValue),
	Integer(IntegerValue),
	String(StringValue),
	Array(ArrayValue),
	Object(ObjectValue),
	Struct(StructInstance),
	Function(FunctionValue),
}

impl Parse for Value {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			map(FunctionValue::parse, |v| Self::Function(v)),
			map(BooleanValue::parse, |v| Self::Boolean(v)),
			map(DecimalValue::parse, |v| Self::Decimal(v)),
			map(IntegerValue::parse, |v| Self::Integer(v)),
			map(StringValue::parse, |v| Self::String(v)),
			map(ArrayValue::parse, |v| Self::Array(v)),
			map(ObjectValue::parse, |v| Self::Object(v)),
			map(StructInstance::parse, |v| Self::Struct(v)),
		))(input)
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Boolean(v) => write!(f, "{}", v),
			Value::Decimal(v) => write!(f, "{}", v),
			Value::Integer(v) => write!(f, "{}", v),
			Value::String(v) => write!(f, "{}", v),
			Value::Array(v) => write!(f, "{}", v),
			Value::Object(v) => write!(f, "{}", v),
			Value::Struct(v) => write!(f, "{}", v),
			Value::Function(v) => write!(f, "{}", v),
		}
	}
}

#[cfg(test)]
#[test]
fn parse_boolean() {
	if let Value::Boolean(_) = Value::parse("true").unwrap().1 {
	} else {
		panic!();
	}
}

#[test]
fn parse_decimal() {
	if let Value::Decimal(_) = Value::parse("3.14").unwrap().1 {
	} else {
		panic!();
	}
}

#[test]
fn parse_integer() {
	if let Value::Integer(_) = Value::parse("1234").unwrap().1 {
	} else {
		panic!();
	}
}

#[test]
fn parse_string() {
	if let Value::String(_) = Value::parse("\"this is a string\"").unwrap().1 {
	} else {
		panic!();
	}
}

#[test]
fn parse_array() {
	if let Value::Array(_) = Value::parse("[3, \"bogo\", bintered]").unwrap().1 {
	} else {
		panic!();
	}
}

#[test]
fn parse_object() {
	if let Value::Object(_) =
		Value::parse("{foo: 3, bar: 6, baz: false}").unwrap().1
	{
	} else {
		panic!();
	}
}

#[test]
fn parse_function() {
	if let Value::Function(_) =
		Value::parse("a b -> math.sqrt( a*a + b*b )").unwrap().1
	{
	} else {
		panic!();
	}
}
