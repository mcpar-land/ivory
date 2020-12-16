use crate::{syntax::expression::Expression, Parse, Result};
use nom::IResult;
use serde::{Deserialize, Serialize};

use super::dice::Dice;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
	pub name: String,
	pub arguments: Vec<String>,
	pub expression: Expression<Dice>,
}

impl Parse for Function {
	fn parse(input: &str) -> IResult<&str, Self> {
		todo!()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
	pub name: String,
	pub arguments: Vec<Expression<Dice>>,
}

impl Parse for FunctionCall {
	fn parse(input: &str) -> IResult<&str, Self> {
		todo!()
	}
}
