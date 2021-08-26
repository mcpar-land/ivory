use crate::{function::FunctionName, Parse};

use super::Expression;

#[derive(Clone, Debug)]
pub struct FunctionCall {
	pub name: FunctionName,
	pub params: Vec<Expression>,
}

impl Parse for FunctionCall {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		todo!()
	}
}
