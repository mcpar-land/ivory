use crate::syntax::{dice::Dice, expression::Expression};
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
