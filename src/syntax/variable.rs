use crate::Parse;
use nom::IResult;
use serde::{Deserialize, Serialize};

use super::{dice::Dice, expression::Expression};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableCall(String);

impl Parse for VariableCall {
	fn parse(input: &str) -> IResult<&str, Self> {
		todo!()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableAssignment {
	pub name: String,
	pub initial: f64,
	pub range: Option<VariableRange>,
}

impl Parse for VariableAssignment {
	fn parse(input: &str) -> IResult<&str, Self> {
		todo!()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableRange {
	// Ranges need to be able to evaluate without rolling.
	pub min: Option<Expression<()>>,
	pub max: Option<Expression<()>>,
}

impl Parse for VariableRange {
	fn parse(input: &str) -> IResult<&str, Self> {
		todo!()
	}
}
