use std::collections::HashMap;

use ivory_expression::Expression;
use ivory_tokenizer::{
	expression::{ExpressionToken, Op},
	values::function::FunctionValue,
};

use crate::{
	runtime::{Runtime, RuntimeContext},
	value::Value,
	Result, RuntimeError,
};

type StdFn = fn(
	runtime: &Runtime,
	ctx: &RuntimeContext,
	args: &Vec<Expression<Op, ExpressionToken>>,
	val: &Value,
) -> Result<Value>;

pub struct StdFnLibrary {
	pub fns: HashMap<String, StdFn>,
}

impl StdFnLibrary {
	pub fn new() -> Self {
		let mut fns = HashMap::<String, StdFn>::new();

		fns.insert("index_of".to_string(), index_of);
		fns.insert("len".to_string(), len);
		fns.insert("map".to_string(), map);
		fns.insert("fold".to_string(), fold);

		Self { fns }
	}

	pub fn call(
		&self,
		runtime: &Runtime,
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

fn get_arg(
	runtime: &Runtime,
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

pub fn index_of(
	runtime: &Runtime,
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

pub fn len(
	_: &Runtime,
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

pub fn map(
	runtime: &Runtime,
	ctx: &RuntimeContext,
	args: &Vec<Expression<Op, ExpressionToken>>,
	val: &Value,
) -> Result<Value> {
	enforce_len(args, 1)?;
	if let Value::Function(FunctionValue {
		args: fn_args,
		expr,
	}) = get_arg(runtime, ctx, &args, 0)?
	{
		if fn_args.len() != 1 && fn_args.len() != 2 {
			return Err(RuntimeError::BadStdFnCall(
				".map() function parameter needs 1 or 2 parameters itself".to_string(),
			));
		}
		let has_i = fn_args.len() == 2;
		match val {
			// Run map over array
			Value::Array(array) => {
				let mut new_array = Vec::<Value>::new();
				for (i, old_val) in array.iter().enumerate() {
					let mut map_ctx = RuntimeContext::one(
						fn_args[0].0.as_str(),
						Expression::new(old_val.clone()),
					);
					if has_i {
						map_ctx.params.insert(
							fn_args[1].0.clone(),
							Expression::new(Value::Integer(i as i32)),
						);
					}
					new_array.push(runtime.evaluate(&map_ctx, &expr)?);
				}
				Ok(Value::Array(new_array))
			}
			// run map over object
			Value::Object(object) => {
				let mut new_map = HashMap::<String, Value>::new();
				for (k, old_val) in object.iter() {
					let mut map_ctx = RuntimeContext::one(
						fn_args[0].0.as_str(),
						Expression::new(old_val.clone()),
					);
					if has_i {
						map_ctx.params.insert(
							fn_args[1].0.clone(),
							Expression::new(Value::String(k.clone())),
						);
					}
					new_map.insert(k.clone(), runtime.evaluate(&map_ctx, &expr)?);
				}
				Ok(Value::Object(new_map))
			}
			_ => Err(no_fn_err("map", val)),
		}
	} else {
		Err(RuntimeError::BadStdFnCall(
			".map() takes only a function as a parameter".to_string(),
		))
	}
}

pub fn fold(
	runtime: &Runtime,
	ctx: &RuntimeContext,
	args: &Vec<Expression<Op, ExpressionToken>>,
	val: &Value,
) -> Result<Value> {
	enforce_len(args, 2)?;
	let mut initial = get_arg(runtime, ctx, &args, 0)?;
	let func = get_arg(runtime, ctx, &args, 1)?;
	if let Value::Function(FunctionValue {
		args: fn_args,
		expr,
	}) = func
	{
		if fn_args.len() != 2 {
			return Err(RuntimeError::BadStdFnCall(
				".fold()'s function parameter must have two parameters".to_string(),
			));
		}
		if let Value::Array(vals) = val {
			for val in vals {
				let mut map_ctx =
					RuntimeContext::one(&fn_args[0].0, Expression::new(initial.clone()));
				map_ctx
					.params
					.insert(fn_args[1].0.clone(), Expression::new(val.clone()));
				initial = runtime.evaluate(&map_ctx, &expr)?;
			}
			Ok(initial)
		} else {
			Err(no_fn_err("fold", val))
		}
	} else {
		Err(RuntimeError::BadStdFnCall(
			".fold() only takes an initial value, and a function as its arguments"
				.to_string(),
		))
	}
}

#[cfg(test)]
mod test {

	use super::*;

	fn test_runtime() -> (Runtime, RuntimeContext) {
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

	#[test]
	fn map_array() {
		let (runtime, _) = test_runtime();
		assert_eq!(
			runtime.run_val("[1, 2, 3].map(v -> v + 1)").unwrap(),
			Value::Array(vec![
				Value::Integer(2),
				Value::Integer(3),
				Value::Integer(4)
			])
		);
		assert_eq!(
			runtime.run_val("[1, 2, 3].map(v -> v + \"\")").unwrap(),
			Value::Array(vec![
				Value::String("1".to_string()),
				Value::String("2".to_string()),
				Value::String("3".to_string())
			])
		);
		assert_eq!(
			runtime.run_val("[1, 2, 3].map(v i -> v + i)").unwrap(),
			Value::Array(vec![
				Value::Integer(1),
				Value::Integer(3),
				Value::Integer(5)
			])
		);
	}
	#[test]
	fn map_object() {
		let (runtime, _) = test_runtime();
		assert_eq!(
			runtime
				.run_val("{a: 1, b: 2, c: 3}.map(v -> v + 1)")
				.unwrap(),
			Value::Object(
				[
					("a".to_string(), Value::Integer(2)),
					("b".to_string(), Value::Integer(3)),
					("c".to_string(), Value::Integer(4)),
				]
				.iter()
				.cloned()
				.collect::<HashMap<String, Value>>()
			)
		);
		assert_eq!(
			runtime
				.run_val("{a: 1, b: 2, c: 3}.map(v i -> v + i)")
				.unwrap(),
			Value::Object(
				[
					("a".to_string(), Value::String("1a".to_string())),
					("b".to_string(), Value::String("2b".to_string())),
					("c".to_string(), Value::String("3c".to_string())),
				]
				.iter()
				.cloned()
				.collect::<HashMap<String, Value>>()
			)
		);
	}
}
