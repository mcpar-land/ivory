use crate::{
	syntax::{dice::Dice, expression::Expression},
	Parse,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionResult {
	expression: Expression<RollResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollResult {
	dice: Dice,
	rolls: Vec<u32>,
}

impl Parse for RollResult {
	fn parse(input: &str) -> crate::Result<(&str, Self)> {
		todo!()
	}
}
