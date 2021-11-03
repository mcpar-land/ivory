use std::{collections::HashMap, ops::Index};

use ivory_expression::Expression;
use ivory_tokenizer::expression::{ExpressionToken, Op};
use rand::Rng;

use crate::{
	mod_loader::ModLoader,
	runtime::{Runtime, RuntimeContext},
	value::Value,
	Result, RuntimeError,
};

type StdFn<R, L> = fn(
	runtime: &Runtime<R, L>,
	ctx: &RuntimeContext,
	args: &Vec<Expression<Op, ExpressionToken>>,
	val: &Value,
) -> Result<Value>;

pub struct StdFnLibrary<R: Rng, L: ModLoader> {
	pub fns: HashMap<String, StdFn<R, L>>,
}

impl<R: Rng, L: ModLoader> StdFnLibrary<R, L> {
	pub fn new() -> Self {
		let mut fns = HashMap::<String, StdFn<R, L>>::new();

		fns.insert("index_of".to_string(), index_of);

		Self { fns }
	}

	pub fn call(
		&self,
		runtime: &Runtime<R, L>,
		ctx: &RuntimeContext,
		args: &Vec<Expression<Op, ExpressionToken>>,
		name: &str,
		val: &Value,
	) -> Result<Value> {
		if let Some(f) = self.fns.get(name) {
			f(runtime, ctx, args, val).map_err(|err| match err {
				RuntimeError::BadStdFnCall(info) => RuntimeError::BadStdFnCall(
					format!("Error calling {}: {}", name, info),
				),
				other => other,
			})
		} else {
			Err(RuntimeError::BadStdFnCall(format!(
				"Function {} not found for kind {}",
				name,
				val.kind()
			)))
		}
	}
	pub fn has(&self, name: &str) -> bool {
		self.fns.contains_key(name)
	}
}

fn no_fn_err(name: &str, val: &Value) -> RuntimeError {
	RuntimeError::NoStdFnForKind(name.to_string(), val.kind())
}

fn get_arg<R: Rng, L: ModLoader>(
	runtime: &Runtime<R, L>,
	ctx: &RuntimeContext,
	args: &Vec<Expression<Op, ExpressionToken>>,
	i: usize,
) -> Result<Value> {
	if let Some(expr) = args.get(i) {
		runtime.evaluate(ctx, expr)
	} else {
		Err(RuntimeError::BadStdFnCall(format!(
			"function requires at least {} arguments",
			i + 1
		)))
	}
}

pub fn index_of<R: Rng, L: ModLoader>(
	runtime: &Runtime<R, L>,
	ctx: &RuntimeContext,
	args: &Vec<Expression<Op, ExpressionToken>>,
	val: &Value,
) -> Result<Value> {
	let query = get_arg(runtime, ctx, &args, 0)?;
	match val {
		Value::Array(array) => Ok(Value::Integer(
			array
				.iter()
				.enumerate()
				.find_map(|(i, val)| if val == &query { Some(i as i32) } else { None })
				.unwrap_or(-1),
		)),
		_ => Err(no_fn_err("index_of", val)),
	}
}



