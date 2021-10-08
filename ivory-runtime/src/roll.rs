use crate::{
	error::RuntimeError,
	runtime::{Runtime, RuntimeContext},
	Result,
};
use ivory_tokenizer::values::dice::{
	Dice, DiceCondition, DiceNumber, DiceOp, DiceOpConditionKind,
};
use rand::Rng;

pub struct Roll {
	count: u32,
	sides: u32,
	rerolls: Vec<i32>,
}

pub struct SingleRoll {
	pub val: u32,
	pub rerolls: Vec<u32>,
	pub explodes: Vec<u32>,
	pub kept: Option<bool>,
	pub success: Option<bool>,
}

impl Roll {
	pub fn roll<R: Rng>(
		source: &Dice,
		runtime: &Runtime,
		ctx: &RuntimeContext,
		rng: &mut R,
	) -> Result<Self> {
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

		fn incompatible_ops<A, B>(a: &Option<A>, b: &Option<B>) -> Result<()> {
			if a.is_some() && b.is_some() {
				Err(RuntimeError::IncompatibleDiceOps)
			} else {
				Ok(())
			}
		}

		incompatible_ops(&explode, &compounding_explode)?;
		incompatible_ops(&reroll, &reroll_once)?;

		let count = match &source.count {
			DiceNumber::Literal(n) => *n as usize,
			DiceNumber::Interpolate(expr) => {
				runtime.execute(ctx, expr)?.to_integer()? as usize
			}
		};
		let sides = match &source.sides {
			DiceNumber::Literal(n) => *n,
			DiceNumber::Interpolate(expr) => {
				runtime.execute(ctx, expr)?.to_integer()? as u32
			}
		};

		static MAX_TRIES: usize = 128;

		// let mut rolls = vec![];
		for i in 0..count {
			let mut roll = SingleRoll {
				val: rng.gen_range(1..=sides),
				rerolls: vec![],
				explodes: vec![],
				kept: None,
				success: None,
			};
			// Reroll Once
			if let Some(reroll_once) = reroll_once {
				if run_dice_condition(runtime, ctx, reroll_once, roll.val, None)? {
					roll.rerolls.push(rng.gen_range(1..=sides));
				}
			}

			// Reroll Continuous
			if let Some(reroll) = reroll {
				// check initial value to start reroll loop
				let condition_num = dice_number(runtime, ctx, &reroll.value)?;
				if run_dice_condition(
					runtime,
					ctx,
					reroll,
					roll.val,
					Some(condition_num),
				)? {
					let mut val = rng.gen_range(1..=sides);
					for _ in 0..MAX_TRIES {
						if run_dice_condition(
							runtime,
							ctx,
							reroll,
							val,
							Some(condition_num),
						)? {
							roll.rerolls.push(val);
							val = rng.gen_range(1..=sides);
							roll.val = val;
						} else {
							break;
						}
					}
				}
			}

			// Explode
			if let Some(explode) = explode {
				let condition_num = dice_number(runtime, ctx, &explode.value)?;
				if run_dice_condition(runtime, ctx, explode, roll.val, None)? {
					todo!();
				}
			}
		}
		todo!();
	}
}

pub fn dice_number(
	r: &Runtime,
	ctx: &RuntimeContext,
	n: &DiceNumber,
) -> Result<u32> {
	match n {
		DiceNumber::Literal(v) => Ok(*v),
		DiceNumber::Interpolate(expr) => {
			let v = r.execute(ctx, expr)?.to_integer()?;
			if v >= 0 {
				Ok(v as u32)
			} else {
				Err(RuntimeError::NegativeDiceNumber)
			}
		}
	}
}

pub fn run_dice_condition<V: Into<u32>>(
	r: &Runtime,
	ctx: &RuntimeContext,
	c: &DiceCondition,
	val: V,
	numb: Option<u32>,
) -> Result<bool> {
	let cond = numb
		.map(|n| Ok(n))
		.unwrap_or_else(|| dice_number(r, ctx, &c.value))?;
	Ok(match c.kind {
		DiceOpConditionKind::Gt => val.into() > cond,
		DiceOpConditionKind::Lt => val.into() < cond,
		DiceOpConditionKind::Eq => val.into() == cond,
		DiceOpConditionKind::GtEq => val.into() >= cond,
		DiceOpConditionKind::LtEq => val.into() <= cond,
	})
}
