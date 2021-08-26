use crate::Parse;

use self::{
	array::ArrayValue, boolean::BooleanValue, decimal::DecimalValue,
	integer::IntegerValue, object::ObjectValue, string::StringValue,
	struct_instance::StructInstance,
};

pub mod array;
pub mod boolean;
pub mod decimal;
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
}

impl Parse for Value {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		todo!()
	}
}
