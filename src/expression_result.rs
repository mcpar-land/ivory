use crate::{
	syntax::{
		dice::Dice,
		expression::{Expression, ExpressionItem, ExpressionOperator},
	},
	Parse,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionResult {
	expression: Expression<RollResult>,
}

impl ExpressionResult {
	pub fn total(&self) -> f64 {
		todo!();
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollResult {
	dice: Dice,
	rolls: Vec<u32>,
}

impl RollResult {
	pub fn total(&self) -> f64 {
		todo!();
	}
}
