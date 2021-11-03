use std::{
	cell::{RefCell, RefMut},
	fmt::Display,
};

use crate::{
	error::RuntimeError,
	mod_loader::ModLoader,
	runtime::{Runtime, RuntimeContext},
	value::Value,
	Result,
};
use ivory_tokenizer::expression::{
	dice_ops::{DiceOp, DiceOpCmp},
	logic::Comparator,
};
use rand::Rng;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Roll {
	pub count: u32,
	pub sides: u32,
	pub rolls: Vec<SingleRoll>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingleRoll {
	pub sides: u32,
	pub val: u32,
	pub rerolls: Vec<u32>,
	pub explodes: Vec<u32>,
	pub kept: Option<bool>,
}

impl SingleRoll {
	pub fn new(sides: u32, val: u32) -> Self {
		Self {
			sides,
			val,
			rerolls: Vec::new(),
			explodes: Vec::new(),
			kept: None,
		}
	}

	pub fn new_rolled(runtime: &Runtime, sides: u32) -> Self {
		let mut s = Self::new(sides, 0);
		s.roll(runtime);
		s
	}

	pub fn val(&self) -> u32 {
		self.val + self.explodes.iter().fold(0, |sum, v| sum + v)
	}

	pub fn roll(&mut self, runtime: &Runtime) {
		let mut rng = runtime.rng();
		self.val = rng.gen_range(1..=self.sides);
	}

	pub fn apply_op(
		&mut self,
		runtime: &Runtime,
		op: &DiceOp,
		rhs: &Value,
	) -> Result<()> {
		match &op.op {
			DiceOpCmp::Keep => {
				if do_cmp(self.val(), &op.cmp, rhs.to_uint()?) {
					self.kept = Some(true)
				} else {
					self.kept = Some(false)
				}
			}
			DiceOpCmp::Reroll => {
				if do_cmp(self.val(), &op.cmp, rhs.to_uint()?) {
					self.rerolls.push(self.val);
				}
			}
			DiceOpCmp::RerollContinuous => {
				todo!()
			}
			DiceOpCmp::Explode => {
				if do_cmp(self.val(), &op.cmp, rhs.to_uint()?) {
					self.explodes.push(runtime.rng().gen_range(1..=self.sides));
				}
			}
			DiceOpCmp::ExplodeContinuous => {
				todo!()
			}
		}
		Ok(())
	}
}

impl Roll {
	pub fn create(
		runtime: &Runtime,
		count: &Value,
		sides: &Value,
	) -> Result<Self> {
		let count = count.to_uint()?;
		let sides = sides.to_uint()?;

		let mut rolls = Vec::new();
		for _ in 0..count {
			rolls.push(SingleRoll::new_rolled(runtime, sides));
		}

		Ok(Roll {
			count,
			sides,
			rolls,
		})
	}

	pub fn apply_op(
		&mut self,
		runtime: &Runtime,
		op: &DiceOp,
		rhs: &Value,
	) -> Result<()> {
		for roll in self.rolls.iter_mut() {
			roll.apply_op(runtime, op, rhs)?;
		}
		Ok(())
	}

	pub fn value(&self) -> u32 {
		self.rolls.iter().fold(0, |sum, roll| sum + roll.val())
	}
}

impl Display for SingleRoll {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.val())
	}
}

impl Display for Roll {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "<{}d{}: {}>", self.count, self.sides, self.value())
	}
}

fn do_cmp(a: u32, cmp: &Comparator, b: u32) -> bool {
	match cmp {
		Comparator::Gt => a > b,
		Comparator::Lt => a < b,
		Comparator::Eq => a == b,
		Comparator::GtEq => a >= b,
		Comparator::LtEq => a <= b,
	}
}
