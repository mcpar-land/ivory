use crate::istruct::StructName;

#[derive(Clone, Debug)]
pub enum Type {
	Integer,
	Decimal,
	Boolean,
	String,
	Struct(StructName),
	Array(Box<Type>),
}
