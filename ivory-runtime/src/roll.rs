use crate::{error::RuntimeError, Result};
use ivory_tokenizer::values::dice::{Dice, DiceOp};

pub struct Roll {
	count: u32,
	sides: u32,
}

impl Roll {
	pub fn roll<R: rand::Rng>(source: &Dice, rng: &mut R) -> Result<Self> {
		let mut success = None;
		let mut failure = None;
		let mut keep_low = None;
		let mut keep_high = None;
		let mut drop_low = None;
		let mut drop_high = None;
		let mut explode = None;
		let mut compounding_explode = None;
		let mut reroll = None;
		let mut reroll_once = None;

		fn load<T>(store: &mut Option<T>, val: T) -> Result<()> {
			if store.is_some() {
				Err(RuntimeError::Syntax(format!("Duplicate dice operator")))
			} else {
				*store = Some(val);
				Ok(())
			}
		}

		for op in &source.ops {
			match op {
				DiceOp::Success(v) => load(&mut success, v)?,
				DiceOp::Failure(v) => load(&mut failure, v)?,
				DiceOp::KeepLow(v) => load(&mut keep_low, v)?,
				DiceOp::KeepHigh(v) => load(&mut keep_high, v)?,
				DiceOp::DropLow(v) => load(&mut drop_low, v)?,
				DiceOp::DropHigh(v) => load(&mut drop_high, v)?,
				DiceOp::Explode(v) => load(&mut explode, v)?,
				DiceOp::CompoundingExplode(v) => load(&mut compounding_explode, v)?,
				DiceOp::Reroll(v) => load(&mut reroll, v)?,
				DiceOp::RerollOnce(v) => load(&mut reroll_once, v)?,
			}
		}
		todo!();
	}
}
