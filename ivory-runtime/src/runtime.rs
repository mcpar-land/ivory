use crate::{
	expr::{into_prec, RolledOp},
	mod_loader::ModLoader,
	prec::{self, Token},
	prec::{Assoc, Climber},
	roll::Roll,
	value::Value,
	Result, RuntimeError,
};
use ivory_expression::{Expression, ExpressionComponent};
use ivory_tokenizer::{
	accessor::{Accessor, AccessorComponent, AccessorRoot},
	expression::{
		math::{ExprOpMath, ExprOpMathKind},
		ExpressionToken, Op,
	},
	istruct::StructDefinition,
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

type Component = ExpressionComponent<RolledOp, Value>;

pub type RolledExpression = prec::Expression<RolledOp, Component>;

pub struct Runtime<R: Rng, L: ModLoader = ()> {
	pub values: RuntimeValues,
	pub rng: RefCell<R>,
	pub climber: Climber<
		R,
		L,
		RolledOp,
		ExpressionComponent<RolledOp, Value>,
		Value,
		RuntimeError,
	>,
	pub mod_loader: L,
}

impl<R: Rng, L: ModLoader> Runtime<R, L> {
	fn prec_handler(
		lhs: Component,
		op: RolledOp,
		rhs: Component,
		runtime: &Self,
		ctx: &RuntimeContext,
	) -> Result<Component> {
		let lhs = lhs.convert(runtime, ctx)?;
		let rhs = rhs.convert(runtime, ctx)?;
		Ok(ExpressionComponent::Token(
			lhs.run_op(&rhs, &op, runtime, ctx)?,
		))
	}

	pub fn new(rng: R, mod_loader: L) -> Self {
		let climber = Climber::new(
			|op, _, _| match op {
				RolledOp::Ternary(_) => (0, Assoc::Right),
				RolledOp::Logic(_) => (1, Assoc::Left),
				RolledOp::Comparator(_) => (2, Assoc::Left),
				RolledOp::Math { kind, .. } => match kind {
					ExprOpMathKind::Add | ExprOpMathKind::Sub => (3, Assoc::Left),
					ExprOpMathKind::Mul | ExprOpMathKind::Div => (4, Assoc::Right),
				},
			},
			Self::prec_handler,
		);
		Self {
			values: RuntimeValues {
				structs: BTreeMap::new(),
				variables: BTreeMap::new(),
			},
			rng: RefCell::new(rng),
			climber,
			mod_loader,
		}
	}
	pub fn rng(&self) -> RefMut<R> {
		self.rng.borrow_mut()
	}
	pub fn load(&mut self, input: &str) -> Result<()> {
		let module = tokenize::<Module>(input)?;

		let mut structs = BTreeMap::new();
		let mut variables = BTreeMap::new();

		for command in module.0.into_iter() {
			match command {
				ivory_tokenizer::commands::Command::Variable(variable) => {
					variables.insert(variable.name.0.clone(), variable);
				}
				ivory_tokenizer::commands::Command::StructDefinition(d) => {
					structs.insert(d.name.0.clone(), d);
				}
				ivory_tokenizer::commands::Command::Use(_) => todo!(),
			}
		}
		self.values = RuntimeValues { structs, variables };

		Ok(())
	}

	pub fn load_module(&mut self, name: &str, input: &str) -> Result<()> {
		todo!();
	}

	pub fn run(&self, cmd: &str) -> Result<Expression<RolledOp, Value>> {
		let ex = tokenize::<Expression<Op, ExpressionToken>>(cmd)?;
		Ok(self.execute(&RuntimeContext::new(), &ex)?)
	}

	pub fn access(
		&self,
		ctx: &RuntimeContext,
		Accessor(var, components): &Accessor,
	) -> Result<Expression<Op, Value>> {
		let mut expr = match var {
			AccessorRoot::Variable(variable) => match ctx.params.get(&variable.0) {
				Some(param) => param.clone(),
				None => {
					let val =
						self.values.variables.get(&variable.0).ok_or_else(|| {
							RuntimeError::VariableNotFound(variable.0.clone())
						})?;
					self.valueify(&RuntimeContext::new(), &val.value)?
				}
			},
			AccessorRoot::Value(value) => {
				Expression::<Op, _>::new(Value::from_token(value, self, ctx)?)
			}
		};
		// let mut expr = match ctx.params.get(&var.0) {
		// 	Some(param) => param.clone(),
		// 	None => {
		// 		let val = self
		// 			.values
		// 			.variables
		// 			.get(&var.0)
		// 			.ok_or_else(|| RuntimeError::VariableNotFound(var.0.clone()))?;
		// 		self.valueify(&RuntimeContext::new(), &val.value)?
		// 	}
		// };
		for component in components {
			let previous_value = self.val_expr_collapse(ctx, &expr)?;
			match component {
				AccessorComponent::Property(prop) => {
					if let Value::Object(obj) = &previous_value {
						if let Some(p) = obj.get(&prop.0) {
							expr = Expression::new(p.clone());
						} else {
							return Err(RuntimeError::PropNotFound(prop.0.clone()));
						}
					} else {
						return Err(RuntimeError::NoPropertyOnKind(
							previous_value.kind(),
							prop.0.clone(),
						));
					}
				}
				AccessorComponent::Index(i) => {
					expr =
						Expression::new(previous_value.index(&self.evaluate(ctx, i)?)?);
				}
				AccessorComponent::Call(call) => {
					if let Value::Function(FunctionValue {
						args,
						expr: fn_expr,
					}) = &previous_value
					{
						let mut new_ctx = RuntimeContext::new();
						for (var, expr) in args.iter().zip(call.iter()) {
							new_ctx
								.params
								.insert(var.0.clone(), self.valueify(ctx, expr)?);
						}
						expr = self.valueify(&new_ctx, fn_expr)?;
					} else {
						return Err(RuntimeError::CannotCallKind(previous_value.kind()));
					}
				}
			}
		}
		Ok(expr.un_nest())
	}

	pub fn evaluate(
		&self,
		ctx: &RuntimeContext,
		expr: &Expression<Op, ExpressionToken>,
	) -> Result<Value> {
		self.math_to_value(self.execute(ctx, expr)?, ctx)
	}

	pub fn execute(
		&self,
		ctx: &RuntimeContext,
		expr: &Expression<Op, ExpressionToken>,
	) -> Result<Expression<RolledOp, Value>> {
		self.roll(ctx, &self.valueify(ctx, expr)?)
	}

	pub fn valueify(
		&self,
		ctx: &RuntimeContext,
		expr: &Expression<Op, ExpressionToken>,
	) -> Result<Expression<Op, Value>> {
		expr.try_map_tokens_components::<Value, _, RuntimeError>(
			|ExpressionToken(accessor)| {
				Ok(self.access(ctx, accessor)?.to_token_or_paren())
			},
		)
	}

	pub fn roll(
		&self,
		ctx: &RuntimeContext,
		expr: &Expression<Op, Value>,
	) -> Result<Expression<RolledOp, Value>> {
		let rolled = expr.collapse::<_, RuntimeError>(|lhs, op, rhs| match op {
			Op::Dice => {
				let count = self.val_expr_component_collapse(ctx, lhs)?;
				let sides = self.val_expr_component_collapse(ctx, rhs)?;

				let roll = Roll::create(self, &count, &sides)?;
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
							roll.apply_op(self, op, &rhs)?;
						}
						ExpressionComponent::Paren(paren) => {
							let rhs = self.val_expr_component_collapse(ctx, rhs)?;
							paren.run_mut(|val| match val {
								Value::Roll(roll) => roll.apply_op(self, op, &rhs),
								_ => Ok(()),
							})?;
						}
					}
					Ok(false)
				}
				Op::Dice => unreachable!(),
				_ => Ok(true),
			})?;

		let converted_ops = handled_ops
			.map_operators(|op| match op {
				Op::Math(ExprOpMath::Binary { kind, round }) => {
					std::result::Result::<RolledOp, RuntimeError>::Ok(RolledOp::Math {
						kind: kind.clone(),
						round: round.clone(),
					})
				}
				Op::Math(ExprOpMath::Ternary(expr)) => Ok(RolledOp::Ternary(Box::new(
					self.execute(ctx, expr.as_ref())?,
				))),
				Op::Comparator(c) => Ok(RolledOp::Comparator(c.clone())),
				Op::Logic(l) => Ok(RolledOp::Logic(l.clone())),
				_ => unreachable!(),
			})
			.ok_op()?;

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
		self.math_to_value(self.roll(ctx, expr)?, ctx)
	}

	pub fn math_to_value(
		&self,
		expr: Expression<RolledOp, Value>,
		ctx: &RuntimeContext,
	) -> Result<Value> {
		Ok(self.climber.process(&into_prec(expr), &self, ctx)?)
	}
}

pub struct RuntimeValues {
	// TODO: look into making these into radix trees instead
	pub structs: BTreeMap<String, StructDefinition>,
	pub variables: BTreeMap<String, Variable>,
}

/// For handling context inside of functions
pub struct RuntimeContext {
	pub params: BTreeMap<String, Expression<Op, Value>>,
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
		(Runtime::new(rand::thread_rng(), ()), RuntimeContext::new())
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
		let mut runtime = Runtime::new(rand::thread_rng(), ());
		runtime
			.load(
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
		println!("{}", runtime.run("1d20").unwrap());
		println!("{}", runtime.run("1d square(5)").unwrap());
	}

	#[test]
	fn respect_variable_scope() {
		let mut runtime = Runtime::new(rand::thread_rng(), ());
		runtime
			.load(
				r#"
		a = 10;
		b = param -> param + 20 + c;
		c = param + 5;
		d = param -> param + 20;
		"#,
			)
			.unwrap();
		println!("{:#?}", runtime.values.variables);
		runtime.run("a").unwrap();
		assert!(runtime.run("c").is_err());
		assert!(runtime.run("b(3)").is_err());
		runtime.run("d(3)").unwrap();
	}
}
