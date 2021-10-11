use crate::{roll::Roll, value::Value, Result, RuntimeError};
use ivory_expression::{Expression, ExpressionComponent};
use ivory_tokenizer::{
	accessor::Accessor,
	expression::{math::ExprOpMath, ExpressionToken, Op},
	tokenize,
	variable::Variable,
};
use rand::{prelude::StdRng, Rng};
use std::{
	cell::{RefCell, RefMut},
	collections::BTreeMap,
	convert::TryInto,
};

pub struct Runtime {
	pub structs: BTreeMap<String, ()>,
	pub variables: BTreeMap<String, Variable>,
}

impl Runtime {
	pub fn load(input: &str) -> Result<Self> {
		let module = tokenize(input)?;

		let mut structs = BTreeMap::new();
		let mut variables = BTreeMap::new();

		for command in module.0.into_iter() {
			match command {
				ivory_tokenizer::commands::Command::Variable(variable) => {
					variables.insert(variable.name.0.clone(), variable);
				}
				ivory_tokenizer::commands::Command::StructDefinition => todo!(),
			}
		}

		Ok(Self { structs, variables })
	}

	pub fn access<R: Rng>(
		&self,
		ctx: &RuntimeContext<R>,
		Accessor(var, components): &Accessor,
	) -> Result<Value> {
		let param_value = ctx.params.get(&var.0);
		todo!();
	}

	pub fn evaluate<R: Rng>(
		&self,
		ctx: &RuntimeContext<R>,
		expr: &Expression<Op, ExpressionToken>,
	) -> Result<Value> {
		self.execute(ctx, expr)?.try_into()
	}

	pub fn execute<R: Rng>(
		&self,
		ctx: &RuntimeContext<R>,
		expr: &Expression<Op, ExpressionToken>,
	) -> Result<Expression<ExprOpMath, Value>> {
		self.roll(ctx, &self.valueify(ctx, expr)?)
	}

	pub fn valueify<R: Rng>(
		&self,
		ctx: &RuntimeContext<R>,
		expr: &Expression<Op, ExpressionToken>,
	) -> Result<Expression<Op, Value>> {
		expr
			.map_tokens(|token| match token {
				ExpressionToken::Value(val) => Value::from_token(val, &self, ctx),
				ExpressionToken::Accessor(accessor) => self.access(ctx, accessor),
			})
			.ok()
	}

	pub fn roll<R: Rng>(
		&self,
		ctx: &RuntimeContext<R>,
		expr: &Expression<Op, Value>,
	) -> Result<Expression<ExprOpMath, Value>> {
		let rolled = expr.collapse::<_, RuntimeError>(|lhs, op, rhs| match op {
			Op::Dice => {
				let count = self.val_expr_component_collapse(ctx, lhs)?;
				let sides = self.val_expr_component_collapse(ctx, rhs)?;
				let roll = Roll::create(ctx, &count, &sides)?;
				*lhs = ExpressionComponent::Token(Value::Roll(roll));
				Ok(false)
			}
			_ => Ok(true),
		})?;

		let handled_ops =
			rolled.collapse::<_, RuntimeError>(|lhs, op, rhs| match op {
				Op::DiceOp(op) => {
					match lhs {
						ExpressionComponent::Token(token) => {
							let roll = token.mut_roll()?;
							let rhs = self.val_expr_component_collapse(ctx, rhs)?;
							roll.apply_op(ctx, op, &rhs)?;
						}
						ExpressionComponent::Paren(paren) => {
							let rhs = self.val_expr_component_collapse(ctx, rhs)?;
							paren.run_mut(|val| match val {
								Value::Roll(roll) => roll.apply_op(ctx, op, &rhs),
								_ => Ok(()),
							})?;
						}
					}
					Ok(false)
				}
				Op::Math(_) => Ok(true),
				Op::Dice => unreachable!(),
			})?;

		let converted_ops = handled_ops.map_operators(|op| match op {
			Op::Math(op) => *op,
			_ => unreachable!(),
		});

		Ok(converted_ops)
	}

	fn val_expr_component_collapse<R: Rng>(
		&self,
		ctx: &RuntimeContext<R>,
		expr: &ExpressionComponent<Op, Value>,
	) -> Result<Value> {
		match expr {
			ExpressionComponent::Token(val) => Ok(val.clone()),
			ExpressionComponent::Paren(paren) => {
				self.val_expr_collapse(ctx, paren.as_ref())
			}
		}
	}

	pub fn val_expr_collapse<R: Rng>(
		&self,
		ctx: &RuntimeContext<R>,
		expr: &Expression<Op, Value>,
	) -> Result<Value> {
		self.roll(ctx, expr)?.try_into()
	}
}

/// For handling context inside of functions
pub struct RuntimeContext<R: Rng> {
	pub params: BTreeMap<String, Value>,
	pub rng: RefCell<R>,
}

impl<R: Rng> RuntimeContext<R> {
	pub fn rng(&self) -> RefMut<R> {
		self.rng.borrow_mut()
	}
	pub fn new(rng: R) -> Self {
		Self {
			params: BTreeMap::new(),
			rng: RefCell::new(rng),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use ivory_tokenizer::Parse;

	fn test_runtime() -> (Runtime, RuntimeContext<impl Rng>) {
		(
			Runtime {
				structs: BTreeMap::new(),
				variables: BTreeMap::new(),
			},
			RuntimeContext::new(rand::thread_rng()),
		)
	}

	#[test]
	fn roll_1_d_20() {
		let (runtime, ctx) = test_runtime();

		let res = runtime
			.evaluate(&ctx, &Expression::parse("1d20").unwrap().1)
			.unwrap();
		println!("{:?}", res);
		let res = runtime
			.evaluate(&ctx, &Expression::parse("3d6").unwrap().1)
			.unwrap();
		println!("{:?}", res);
	}
}
