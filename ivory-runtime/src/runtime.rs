use crate::{roll::Roll, value::Value, Result, RuntimeError};
use ivory_expression::{Expression, ExpressionComponent};
use ivory_tokenizer::{
	accessor::{Accessor, AccessorComponent},
	expression::{math::ExprOpMath, ExpressionToken, Op},
	tokenize,
	values::function::FunctionValue,
	variable::Variable,
	Module, Parse,
};
use rand::{prelude::StdRng, Rng};
use std::{
	cell::{RefCell, RefMut},
	collections::BTreeMap,
	convert::TryInto,
};

pub struct Runtime<R: Rng> {
	pub structs: BTreeMap<String, ()>,
	pub variables: BTreeMap<String, Variable>,
	pub rng: RefCell<R>,
}

impl<R: Rng> Runtime<R> {
	pub fn new(rng: R) -> Self {
		Self {
			structs: BTreeMap::new(),
			variables: BTreeMap::new(),
			rng: RefCell::new(rng),
		}
	}
	pub fn rng(&self) -> RefMut<R> {
		self.rng.borrow_mut()
	}
	pub fn load(rng: R, input: &str) -> Result<Self> {
		let module = tokenize::<Module>(input)?;

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

		Ok(Self {
			structs,
			variables,
			rng: RefCell::new(rng),
		})
	}

	pub fn run(&self, cmd: &str) -> Result<Expression<ExprOpMath, Value>> {
		let ex = tokenize::<Expression<Op, ExpressionToken>>(cmd)?;
		Ok(self.execute(&RuntimeContext::new(), &ex)?)
	}

	pub fn access(
		&self,
		ctx: &RuntimeContext,
		Accessor(var, components): &Accessor,
	) -> Result<Value> {
		let mut value = match ctx.params.get(&var.0) {
			Some(param) => param.clone(),
			None => {
				let val = self
					.variables
					.get(&var.0)
					.ok_or_else(|| RuntimeError::VariableNotFound(var.0.clone()))?;
				self.evaluate(ctx, &val.value)?
			}
		};
		for component in components {
			match component {
				AccessorComponent::Property(prop) => {
					if let Value::Object(obj) = &value {
						if let Some(p) = obj.get(&prop.0) {
							value = p.clone();
						} else {
							return Err(RuntimeError::PropNotFound(prop.0.clone()));
						}
					} else {
						return Err(RuntimeError::NoPropertyOnKind(
							value.kind(),
							prop.0.clone(),
						));
					}
				}
				AccessorComponent::Index(i) => {
					value = value.index(&self.evaluate(ctx, i)?)?;
				}
				AccessorComponent::Call(call) => {
					if let Value::Function(FunctionValue { args, expr }) = &value {
						let mut new_ctx = RuntimeContext::new();
						for (var, expr) in args.iter().zip(call.iter()) {
							new_ctx
								.params
								.insert(var.0.clone(), self.evaluate(ctx, expr)?);
						}
						value = self.evaluate(&new_ctx, expr)?;
					} else {
						return Err(RuntimeError::CannotCallKind(value.kind()));
					}
				}
			}
		}
		Ok(value)
	}

	pub fn evaluate(
		&self,
		ctx: &RuntimeContext,
		expr: &Expression<Op, ExpressionToken>,
	) -> Result<Value> {
		self.execute(ctx, expr)?.try_into()
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
		let rolled = expr.collapse::<_, RuntimeError>(|lhs, op, rhs| match op {
			Op::Dice => {
				let count = self.val_expr_component_collapse(ctx, lhs)?;
				let sides = self.val_expr_component_collapse(ctx, rhs)?;

				let roll = Roll::create(self.rng(), &count, &sides)?;
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

	fn val_expr_component_collapse(
		&self,
		ctx: &RuntimeContext,
		expr: &ExpressionComponent<Op, Value>,
	) -> Result<Value> {
		match expr {
			ExpressionComponent::Token(val) => Ok(val.clone()),
			ExpressionComponent::Paren(paren) => {
				self.val_expr_collapse(ctx, paren.as_ref())
			}
		}
	}

	pub fn val_expr_collapse(
		&self,
		ctx: &RuntimeContext,
		expr: &Expression<Op, Value>,
	) -> Result<Value> {
		self.roll(ctx, expr)?.try_into()
	}
}

/// For handling context inside of functions
pub struct RuntimeContext {
	pub params: BTreeMap<String, Value>,
}

impl RuntimeContext {
	pub fn new() -> Self {
		Self {
			params: BTreeMap::new(),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use ivory_tokenizer::Parse;

	fn test_runtime() -> (Runtime<impl Rng>, RuntimeContext) {
		(Runtime::new(rand::thread_rng()), RuntimeContext::new())
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

	#[test]
	fn load_module() {
		let runtime = Runtime::load(
			rand::thread_rng(),
			r#"
		foo = 900;
		bar = 33;
		ooer = foo + bar;
		square = a -> a * a;
		modifier = score -> (score /_ 2) - 5;
		"#,
		)
		.unwrap();
		println!("{}", runtime.run("ooer + 1").unwrap());
		println!("{}", runtime.run("square(5)").unwrap());
		println!(
			"{}",
			runtime
				.run(
					r#"[
			modifier(3),
			modifier(4),
			modifier(5),
			modifier(6),
			modifier(7),
			modifier(8),
			modifier(9),
			modifier(10)
		]"#
				)
				.unwrap()
		);
	}
}
