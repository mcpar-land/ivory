use std::fmt::Display;

use crate::{
	error::RuntimeError,
	runtime::{Runtime, RuntimeContext},
	value::Value,
	Result,
};
use ivory_tokenizer::expression::dice_ops::DiceOp;
use rand::Rng;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Roll {
	pub count: u32,
	pub sides: u32,
	pub rolls: Vec<SingleRoll>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingleRoll {
	pub val: u32,
	pub rerolls: Vec<u32>,
	pub explodes: Vec<u32>,
	pub kept: Option<bool>,
}

impl SingleRoll {
	pub fn new(val: u32) -> Self {
		Self {
			val,
			rerolls: Vec::new(),
			explodes: Vec::new(),
			kept: None,
		}
	}
}

impl Roll {
	pub fn create<R: Rng>(
		ctx: &RuntimeContext<R>,
		count: &Value,
		sides: &Value,
	) -> Result<Self> {
		fn to_dice_num(num: i32) -> Result<u32> {
			if num < 0 {
				Err(RuntimeError::NegativeDiceNumber)
			} else {
				Ok(num as u32)
			}
		}
		let count = to_dice_num(*count.to_integer()?)?;
		let sides = to_dice_num(*sides.to_integer()?)?;
		let mut rng = ctx.rng();
		let mut rolls = Vec::new();
		for _ in 0..count {
			rolls.push(SingleRoll::new(rng.gen_range(1..=sides)));
		}

		Ok(Roll {
			count,
			sides,
			rolls,
		})
	}

	pub fn apply_op<R: Rng>(
		&mut self,
		ctx: &RuntimeContext<R>,
		op: &DiceOp,
		rhs: &Value,
	) -> Result<()> {
		todo!();
	}

	pub fn value(&self) -> u32 {
		todo!();
	}
}

impl Display for Roll {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}
