use std::{
	collections::HashMap,
	ops::{Index, Range},
};

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
		fns.insert("len".to_string(), len);

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

fn enforce_len(
	args: &Vec<Expression<Op, ExpressionToken>>,
	len: usize,
) -> Result<()> {
	if len == args.len() {
		Ok(())
	} else {
		Err(RuntimeError::BadStdFnCall(format!(
			"std function requires {} arguments",
			len
		)))
	}
}

// ========================================================================== //

pub fn index_of<R: Rng, L: ModLoader>(
	runtime: &Runtime<R, L>,
	ctx: &RuntimeContext,
	args: &Vec<Expression<Op, ExpressionToken>>,
	val: &Value,
) -> Result<Value> {
	enforce_len(args, 1)?;
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

pub fn len<R: Rng, L: ModLoader>(
	_: &Runtime<R, L>,
	_: &RuntimeContext,
	args: &Vec<Expression<Op, ExpressionToken>>,
	val: &Value,
) -> Result<Value> {
	enforce_len(args, 0)?;
	Ok(Value::Integer(match val {
		Value::String(s) => s.len(),
		Value::Roll(r) => r.rolls.len(),
		Value::Array(a) => a.len(),
		Value::Object(o) => o.len(),
		_ => {
			return Err(no_fn_err("len", val));
		}
	} as i32))
}

#[cfg(test)]
mod test {
	use rand::prelude::ThreadRng;

	use super::*;

	fn test_runtime() -> (Runtime<ThreadRng>, RuntimeContext) {
		let mut r = Runtime::new(rand::thread_rng(), ());
		r.load(
			r#"
			x = [100, 200, 300, 400, 500, 600];
			y = "123456789";
			z = { foo: 10, bar: 100, baz: 1000, child: ["first", "second"] };
			i = 45;
			"#,
		)
		.unwrap();
		(r, RuntimeContext::new())
	}

	#[test]
	fn len() {
		let (runtime, _) = test_runtime();

		assert_eq!(runtime.run_val("x.len()").unwrap(), Value::Integer(6));
		assert_eq!(runtime.run_val("y.len()").unwrap(), Value::Integer(9));
		assert_eq!(runtime.run_val("z.len()").unwrap(), Value::Integer(4));
		assert_eq!(runtime.run_val("z.child.len()").unwrap(), Value::Integer(2));
		assert!(runtime.run_val("z.foo.len()").is_err());
		assert!(runtime.run_val("i.len()").is_err());
		assert_eq!(
			runtime.run_val("z[\"child\"].len()").unwrap(),
			Value::Integer(2)
		);

		assert!(runtime.run_val("x.len(12)").is_err());
	}

	#[test]
	fn index_of() {
		let (runtime, _) = test_runtime();

		assert_eq!(
			runtime.run_val("x.index_of(200)").unwrap(),
			Value::Integer(1)
		);
		assert_eq!(
			runtime.run_val("x.index_of(\"bingus\")").unwrap(),
			Value::Integer(-1)
		);
		assert_eq!(
			runtime.run_val("x.index_of(200)").unwrap(),
			Value::Integer(1)
		);
	}
}
