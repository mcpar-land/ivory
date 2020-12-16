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
	pub range: Option<std::ops::Range<f64>>,
}

impl Parse for VariableAssignment {
	fn parse(input: &str) -> IResult<&str, Self> {
		todo!()
	}
}

pub struct VariableRange {
	// Ranges need to be able to evaluate without rolling.
	min: Option<Expression<()>>,
	max: Option<Expression<()>>,
}
