use crate::{value::Value, Result, RuntimeError};
use ivory_expression::Expression;
use ivory_tokenizer::{
	accessor::Accessor,
	expression::{math::ExprOpMath, ExpressionToken, Op},
	tokenize,
	variable::Variable,
};
use rand::Rng;
use std::collections::BTreeMap;

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

	pub fn access(
		&self,
		ctx: &RuntimeContext,
		Accessor(var, components): &Accessor,
	) -> Result<Value> {
		let param_value = ctx.params.get(&var.0);
		todo!();
	}

	pub fn execute(
		&self,
		ctx: &RuntimeContext,
		expr: &Expression<Op, ExpressionToken>,
	) -> Result<Expression<ExprOpMath, Value>> {
		self.roll(ctx, &self.valueify(ctx, expr)?)
	}

	pub fn valueify(
		&self,
		ctx: &RuntimeContext,
		expr: &Expression<Op, ExpressionToken>,
	) -> Result<Expression<Op, Value>> {
		expr
			.map_tokens(|token| match token {
				ExpressionToken::Value(val) => Value::from_token(val, &self, ctx),
				ExpressionToken::Accessor(accessor) => self.access(ctx, accessor),
			})
			.ok()
	}

	pub fn roll(
		&self,
		ctx: &RuntimeContext,
		expr: &Expression<Op, Value>,
	) -> Result<Expression<ExprOpMath, Value>> {
		let rolled = expr.collapse::<_, RuntimeError>(|lhs, op, rhs| {
			if let Op::Dice = op {
				let lhs = match lhs {
					ivory_expression::ExpressionComponent::Token(val) => Ok(val.clone()),
					ivory_expression::ExpressionComponent::Paren(paren) => {
						self.evaluate(ctx, paren.as_ref())
					}
				};
				todo!();
			} else {
				Ok(true)
			}
		})?;

		todo!();
	}

	pub fn evaluate(
		&self,
		ctx: &RuntimeContext,
		expr: &Expression<Op, Value>,
	) -> Result<Value> {
		todo!();
	}
}

/// For handling context inside of functions
pub struct RuntimeContext {
	pub params: BTreeMap<String, Value>,
}
