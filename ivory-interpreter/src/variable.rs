use crate::{util::variable_name, Parse};

#[derive(Clone, Debug)]
pub struct Variable {
	pub name: VariableName,
	pub value: (), // TODO
}

#[derive(Clone, Debug)]
pub struct VariableName(pub String);

impl Parse for VariableName {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, val) = variable_name(input)?;
		Ok((input, Self(val.to_string())))
	}
}
