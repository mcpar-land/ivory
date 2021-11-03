use crate::{
	expr::{into_prec, RolledOp},
	mod_loader::ModLoader,
	prec::{self, Token},
	prec::{Assoc, Climber},
	roll::Roll,
	std_fns::StdFnLibrary,
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
	Module,
};
use rand::RngCore;
use std::{
	cell::{RefCell, RefMut},
	collections::BTreeMap,
};

type Component = ExpressionComponent<RolledOp, Value>;

pub type RolledExpression = prec::Expression<RolledOp, Component>;

pub struct Runtime {
	pub values: RuntimeValues,
	pub rng: RefCell<Box<dyn RngCore>>,
	pub climber: Climber<
		RolledOp,
		ExpressionComponent<RolledOp, Value>,
		Value,
		RuntimeError,
	>,
	pub mod_loader: Box<dyn ModLoader>,
	pub std_fns: StdFnLibrary,
}

impl Runtime {
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

	pub fn new<R: 'static + RngCore, L: 'static + ModLoader>(
		rng: R,
		mod_loader: L,
	) -> Self {
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
			rng: RefCell::new(Box::new(rng)),
			climber,
			mod_loader: Box::new(mod_loader),
			std_fns: StdFnLibrary::new(),
		}
	}
	pub fn rng(&self) -> RefMut<Box<dyn RngCore>> {
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

	pub fn run_val(&self, cmd: &str) -> Result<Value> {
		let res = self.run(cmd)?;
		self.math_to_value(res, &RuntimeContext::new())
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
		// This is a check that is set to something when the accessor is:
		// a variable that is not found on the parent value
		// is the name of a standard function
		let mut std_fn_call: Option<String> = None;
		for component in components {
			// make sure that a call always comes right after a std function name
			if std_fn_call.is_some()
				&& !matches!(component, AccessorComponent::Call(_))
			{
				return Err(RuntimeError::BadStdFnCall(format!(
					"{} is a standard function, not a value, and must be called",
					std_fn_call.unwrap()
				)));
			}
			let previous_value = self.val_expr_collapse(ctx, &expr)?;
			match component {
				AccessorComponent::Property(prop) => {
					if let Value::Object(obj) = &previous_value {
						if let Some(p) = obj.get(&prop.0) {
							expr = Expression::new(p.clone());
						} else {
							if self.std_fns.has(&prop.0) {
								std_fn_call = Some(prop.0.clone());
							} else {
								return Err(RuntimeError::PropNotFound(prop.0.clone()));
							}
						}
					} else {
						// object props override std function names.
						if self.std_fns.has(&prop.0) {
							std_fn_call = Some(prop.0.clone());
						} else {
							return Err(RuntimeError::NoPropertyOnKind(
								previous_value.kind(),
								prop.0.clone(),
							));
						}
					}
				}
				AccessorComponent::Index(i) => {
					expr =
						Expression::new(previous_value.index(&self.evaluate(ctx, i)?)?);
				}
				AccessorComponent::Call(call) => {
					if let Some(fn_name) = &std_fn_call {
						let std_call_res = self.std_fns.call(
							&self,
							ctx,
							call,
							fn_name.as_str(),
							&previous_value,
						)?;
						expr = Expression::new(std_call_res);
						std_fn_call = None;
					} else {
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
		}
		if let Some(uncalled_std_fn_name) = std_fn_call {
			return Err(RuntimeError::BadStdFnCall(format!(
				"{} is a standard function, not a value, and must be called",
				uncalled_std_fn_name
			)));
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

	fn test_runtime() -> (Runtime, RuntimeContext) {
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

	#[test]
	fn std_functions() {
		let mut runtime = Runtime::new(rand::thread_rng(), ());
		runtime
			.load(
				r#"
			x = [10, 20, 30, 40, 50, 60];
			y = 5;
			"#,
			)
			.unwrap();
		println!("{}", runtime.run("x.index_of(40)").unwrap());
		assert!(runtime.run("y.index_of(4)").is_err());
		assert!(runtime.run("x.index_of").is_err());
		assert!(runtime.run("x.index_of[200]").is_err());
	}
}
