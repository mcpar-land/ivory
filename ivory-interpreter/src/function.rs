use crate::{
	expression::Expression, util::variable_name, variable::VariableName, Parse,
};

#[derive(Clone, Debug)]
pub struct Function {
	pub parameters: Vec<VariableName>,
	pub expr: Expression,
}

#[derive(Clone, Debug)]
pub struct FunctionName(pub String);

impl Parse for FunctionName {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, val) = variable_name(input)?;
		Ok((input, Self(val.to_string())))
	}
}
