use colored::*;
use ivory_tokenizer::{
	expression::{
		logic::{Comparator, LogicOp},
		math::{ExprOpMathKind, ExprOpMathRound},
	},
	itype::Type,
	values::{
		array::ArrayValue, boolean::BooleanValue, decimal::DecimalValue,
		function::FunctionValue, integer::IntegerValue, object::ObjectValue,
		string::StringValue,
	},
};
use rand::Rng;

use crate::{
	error::RuntimeError,
	expr::RolledOp,
	mod_loader::ModLoader,
	roll::Roll,
	runtime::{Runtime, RuntimeContext},
};
use crate::{struct_value::StructValue, Result};

use std::{collections::HashMap, fmt::Display};

static K_INTEGER: &'static str = "int";
static K_DECIMAL: &'static str = "decimal";
static K_BOOLEAN: &'static str = "bool";
static K_STRING: &'static str = "string";
static K_ROLL: &'static str = "roll";
static K_ARRAY: &'static str = "array";
static K_OBJECT: &'static str = "object";
static K_FUNCTION: &'static str = "function";

#[derive(Clone, Debug)]
pub enum Value {
	Integer(i32),
	Decimal(f32),
	Boolean(bool),
	String(String),
	Roll(Roll),
	Array(Vec<Value>),
	Object(HashMap<String, Value>),
	Function(FunctionValue),
	Struct(StructValue),
}

impl Value {
	pub fn val_type(&self) -> Type {
		match self {
			Value::Integer(_) => Type::Integer,
			Value::Decimal(_) => Type::Decimal,
			Value::Boolean(_) => Type::Boolean,
			Value::String(_) => Type::String,
			Value::Roll(_) => Type::Roll,
			Value::Array(vals) => Type::Array({
				todo!();
			}),
			Value::Object(_) => Type::Object,
			Value::Function(_) => todo!(),
			Value::Struct(_) => todo!(),
		}
	}

	/// Less specific than val_type
	pub fn kind(&self) -> ValueKind {
		match self {
			Value::Integer(_) => ValueKind::Integer,
			Value::Decimal(_) => ValueKind::Decimal,
			Value::Boolean(_) => ValueKind::Boolean,
			Value::String(_) => ValueKind::String,
			Value::Roll(_) => ValueKind::Roll,
			Value::Array(_) => ValueKind::Array,
			Value::Object(_) => ValueKind::Object,
			Value::Function(_) => ValueKind::Function,
			Value::Struct(_) => ValueKind::Object,
		}
	}

	pub fn index(&self, i: &Value) -> Result<Value> {
		match self {
			Value::String(s) => {
				let i = *i.to_integer()? as usize;
				if let Some(c) = s.chars().nth(i) {
					Ok(Value::String(c.to_string()))
				} else {
					Err(RuntimeError::IndexOutOfBounds(i, s.len()))
				}
			}
			Value::Roll(r) => {
				let i = *i.to_integer()? as usize;
				if let Some(dice) = r.rolls.get(i) {
					Ok(Value::Integer(dice.val() as i32))
				} else {
					Err(RuntimeError::IndexOutOfBounds(i, r.rolls.len()))
				}
			}
			Value::Array(a) => {
				let i = *i.to_integer()? as usize;
				if let Some(v) = a.get(i) {
					Ok(v.clone())
				} else {
					Err(RuntimeError::IndexOutOfBounds(i, a.len()))
				}
			}
			Value::Object(o) => {
				let i = i.to_string()?;
				if let Some(v) = o.get(i) {
					Ok(v.clone())
				} else {
					Err(RuntimeError::PropNotFound(i.clone()))
				}
			}
			_ => Err(RuntimeError::CannotIndexKind(self.kind())),
		}
	}

	pub fn run_op<R: Rng, L: ModLoader>(
		&self,
		rhs: &Value,
		op: &RolledOp,
		runtime: &Runtime<R, L>,
		ctx: &RuntimeContext,
	) -> Result<Value> {
		use Value::*;
		match op {
			RolledOp::Ternary(ternary) => {
				if *self.to_boolean()? {
					runtime.math_to_value(ternary.as_ref().clone(), ctx)
				} else {
					Ok(rhs.clone())
				}
			}
			op => match (self, rhs) {
				(Integer(a), Integer(b)) => a.op(b, op, runtime, ctx),
				(Integer(a), Decimal(b)) => (*a as f32).op(b, op, runtime, ctx),
				(Integer(a), Boolean(b)) => a.op(&(*b as i32), op, runtime, ctx),
				(Integer(a), String(b)) => a.to_string().op(b, op, runtime, ctx),
				(Integer(a), Roll(b)) => a.op(&(b.value() as i32), op, runtime, ctx),
				(Decimal(a), Integer(b)) => a.op(&(*b as f32), op, runtime, ctx),
				(Decimal(a), Decimal(b)) => a.op(b, op, runtime, ctx),
				(Decimal(a), Boolean(b)) => a.op(&(*b as i32 as f32), op, runtime, ctx),
				(Decimal(a), String(b)) => a.to_string().op(b, op, runtime, ctx),
				(Decimal(a), Roll(b)) => a.op(&(b.value() as f32), op, runtime, ctx),
				(Boolean(a), Integer(b)) => (*a as i32).op(b, op, runtime, ctx),
				(Boolean(a), Decimal(b)) => (*a as i32 as f32).op(b, op, runtime, ctx),
				(Boolean(a), Boolean(b)) => {
					(*a as i32).op(&(*b as i32), op, runtime, ctx)
				}
				(Boolean(a), String(b)) => a.to_string().op(b, op, runtime, ctx),
				(Boolean(a), Roll(b)) => {
					(*a as i32).op(&(b.value() as i32), op, runtime, ctx)
				}
				(String(a), Integer(b)) => a.op(&b.to_string(), op, runtime, ctx),
				(String(a), Decimal(b)) => a.op(&b.to_string(), op, runtime, ctx),
				(String(a), Boolean(b)) => a.op(&b.to_string(), op, runtime, ctx),
				(String(a), String(b)) => a.op(b, op, runtime, ctx),
				(String(a), Roll(b)) => a.op(&format!("{}", b), op, runtime, ctx),
				(Roll(a), Integer(b)) => (a.value() as i32).op(b, op, runtime, ctx),
				(Roll(a), Decimal(b)) => (a.value() as f32).op(b, op, runtime, ctx),
				(Roll(a), Boolean(b)) => {
					(a.value() as i32).op(&(*b as i32), op, runtime, ctx)
				}
				(Roll(a), String(b)) => format!("{}", a).op(b, op, runtime, ctx),
				(Roll(a), Roll(b)) => {
					(a.value() as i32).op(&(b.value() as i32), op, runtime, ctx)
				}
				(Array(a), Array(b)) => a.op(b, op, runtime, ctx),
				(Array(a), b) => append(op, a, b),
				(a, Array(b)) => prepend(op, a, b),
				(a, b) => {
					Err(RuntimeError::CannotRunOp(a.kind(), op.clone(), b.kind()))
				}
			},
		}
	}

	pub fn to_integer(&self) -> Result<&i32> {
		if let Self::Integer(v) = self {
			Ok(v)
		} else {
			Err(RuntimeError::WrongExpectedValue(
				ValueKind::Integer,
				self.kind(),
			))
		}
	}

	pub fn to_uint(&self) -> Result<u32> {
		let i = *self.to_integer()?;
		if i < 0 {
			Err(RuntimeError::NegativeDiceNumber)
		} else {
			Ok(i as u32)
		}
	}

	pub fn to_decimal(&self) -> Result<&f32> {
		if let Self::Decimal(v) = self {
			Ok(v)
		} else {
			Err(RuntimeError::WrongExpectedValue(
				ValueKind::Decimal,
				self.kind(),
			))
		}
	}

	pub fn to_boolean(&self) -> Result<&bool> {
		if let Self::Boolean(v) = self {
			Ok(v)
		} else {
			Err(RuntimeError::WrongExpectedValue(
				ValueKind::Boolean,
				self.kind(),
			))
		}
	}

	pub fn to_string(&self) -> Result<&String> {
		if let Self::String(v) = self {
			Ok(v)
		} else {
			Err(RuntimeError::WrongExpectedValue(
				ValueKind::String,
				self.kind(),
			))
		}
	}

	pub fn to_roll(&self) -> Result<&Roll> {
		if let Self::Roll(v) = self {
			Ok(v)
		} else {
			Err(RuntimeError::WrongExpectedValue(
				ValueKind::Roll,
				self.kind(),
			))
		}
	}

	pub fn to_array(&self) -> Result<&Vec<Value>> {
		if let Self::Array(v) = self {
			Ok(v)
		} else {
			Err(RuntimeError::WrongExpectedValue(
				ValueKind::Array,
				self.kind(),
			))
		}
	}

	pub fn to_object(&self) -> Result<&HashMap<String, Value>> {
		if let Self::Object(v) = self {
			Ok(v)
		} else {
			Err(RuntimeError::WrongExpectedValue(
				ValueKind::Object,
				self.kind(),
			))
		}
	}

	pub fn to_function(&self) -> Result<&FunctionValue> {
		if let Self::Function(v) = self {
			Ok(v)
		} else {
			Err(RuntimeError::WrongExpectedValue(
				ValueKind::Function,
				self.kind(),
			))
		}
	}

	// mut edits

	pub fn mut_integer(&mut self) -> Result<&mut i32> {
		if let Self::Integer(v) = self {
			Ok(v)
		} else {
			Err(RuntimeError::WrongExpectedValue(
				ValueKind::Integer,
				self.kind(),
			))
		}
	}

	pub fn mut_decimal(&mut self) -> Result<&mut f32> {
		if let Self::Decimal(v) = self {
			Ok(v)
		} else {
			Err(RuntimeError::WrongExpectedValue(
				ValueKind::Decimal,
				self.kind(),
			))
		}
	}

	pub fn mut_boolean(&mut self) -> Result<&mut bool> {
		if let Self::Boolean(v) = self {
			Ok(v)
		} else {
			Err(RuntimeError::WrongExpectedValue(
				ValueKind::Boolean,
				self.kind(),
			))
		}
	}

	pub fn mut_string(&mut self) -> Result<&mut String> {
		if let Self::String(v) = self {
			Ok(v)
		} else {
			Err(RuntimeError::WrongExpectedValue(
				ValueKind::String,
				self.kind(),
			))
		}
	}

	pub fn mut_roll(&mut self) -> Result<&mut Roll> {
		if let Self::Roll(v) = self {
			Ok(v)
		} else {
			Err(RuntimeError::WrongExpectedValue(
				ValueKind::Roll,
				self.kind(),
			))
		}
	}

	pub fn mut_array(&mut self) -> Result<&mut Vec<Value>> {
		if let Self::Array(v) = self {
			Ok(v)
		} else {
			Err(RuntimeError::WrongExpectedValue(
				ValueKind::Array,
				self.kind(),
			))
		}
	}

	pub fn mut_object(&mut self) -> Result<&mut HashMap<String, Value>> {
		if let Self::Object(v) = self {
			Ok(v)
		} else {
			Err(RuntimeError::WrongExpectedValue(
				ValueKind::Object,
				self.kind(),
			))
		}
	}

	pub fn mut_function(&mut self) -> Result<&mut FunctionValue> {
		if let Self::Function(v) = self {
			Ok(v)
		} else {
			Err(RuntimeError::WrongExpectedValue(
				ValueKind::Function,
				self.kind(),
			))
		}
	}

	pub fn from_token<R: Rng, L: ModLoader>(
		token: &ivory_tokenizer::values::Value,
		runtime: &Runtime<R, L>,
		ctx: &RuntimeContext,
	) -> Result<Self> {
		Ok(match token {
			ivory_tokenizer::values::Value::Boolean(BooleanValue(v)) => {
				Self::Boolean(*v)
			}
			ivory_tokenizer::values::Value::Decimal(DecimalValue(v)) => {
				Self::Decimal(*v as f32)
			}
			ivory_tokenizer::values::Value::Integer(IntegerValue(v)) => {
				Value::Integer(*v as i32)
			}
			ivory_tokenizer::values::Value::String(StringValue(v)) => {
				Value::String(v.clone())
			}
			ivory_tokenizer::values::Value::Array(ArrayValue(v)) => Value::Array(
				v.iter()
					.map(|v| {
						let v = runtime.valueify(ctx, v)?;
						runtime.val_expr_collapse(ctx, &v)
					})
					.collect::<Result<Vec<Value>>>()?,
			),
			ivory_tokenizer::values::Value::Object(ObjectValue(v)) => Value::Object(
				v.iter()
					.map(|(n, v)| {
						let v = runtime.valueify(ctx, v)?;
						Ok((n.0.clone(), runtime.val_expr_collapse(ctx, &v)?))
					})
					.collect::<Result<HashMap<String, Value>>>()?,
			),
			ivory_tokenizer::values::Value::Struct(s) => todo!(),
			ivory_tokenizer::values::Value::Function(f) => Value::Function(f.clone()),
		})
	}

	/// Returns true if values have the same type
	pub fn eq_type(&self, other: &Value) -> bool {
		todo!();
	}
}

impl PartialEq for Value {
	fn eq(&self, other: &Self) -> bool {
		use Value::*;
		match (self, other) {
			(Integer(l0), Integer(r0)) => l0 == r0,
			(Decimal(l0), Decimal(r0)) => l0 == r0,
			(Boolean(l0), Boolean(r0)) => l0 == r0,
			(String(l0), String(r0)) => l0 == r0,
			(Roll(l0), Roll(r0)) => l0 == r0,
			(Array(l0), Array(r0)) => l0 == r0,
			(Object(l0), Object(r0)) => l0 == r0,
			(Function(_), Function(_)) => false,
			_ => todo!(),
		}
	}
}

fn same_op_err(kind: ValueKind, op: &RolledOp) -> Result<Value> {
	Err(RuntimeError::CannotRunOp(kind, op.clone(), kind))
}

fn append(op: &RolledOp, a: &Vec<Value>, b: &Value) -> Result<Value> {
	if let RolledOp::Math {
		kind: ExprOpMathKind::Add,
		..
	} = op
	{
		let mut a = a.clone();
		a.push(b.clone());
		Ok(Value::Array(a))
	} else {
		Err(RuntimeError::CannotRunOp(
			ValueKind::Array,
			op.clone(),
			b.kind(),
		))
	}
}

fn prepend(op: &RolledOp, a: &Value, b: &Vec<Value>) -> Result<Value> {
	if let RolledOp::Math {
		kind: ExprOpMathKind::Add,
		..
	} = op
	{
		let mut b = b.clone();
		b.insert(0, a.clone());
		Ok(Value::Array(b))
	} else {
		Err(RuntimeError::CannotRunOp(
			a.kind(),
			op.clone(),
			ValueKind::Array,
		))
	}
}

trait RunOp {
	fn op<R: Rng, L: ModLoader>(
		&self,
		other: &Self,
		op: &RolledOp,
		runtime: &Runtime<R, L>,
		ctx: &RuntimeContext,
	) -> Result<Value>;
}

impl RunOp for bool {
	fn op<R: Rng, L: ModLoader>(
		&self,
		other: &Self,
		op: &RolledOp,
		runtime: &Runtime<R, L>,
		ctx: &RuntimeContext,
	) -> Result<Value> {
		match op {
			RolledOp::Logic(l) => Ok(Value::Boolean(match l {
				LogicOp::And => *self && *other,
				LogicOp::Or => *self || *other,
			})),
			_ => return same_op_err(ValueKind::Boolean, op),
		}
	}
}

impl RunOp for i32 {
	fn op<R: Rng, L: ModLoader>(
		&self,
		other: &Self,
		op: &RolledOp,
		runtime: &Runtime<R, L>,
		ctx: &RuntimeContext,
	) -> Result<Value> {
		match op {
			RolledOp::Math { kind, round } => Ok(Value::Integer(match kind {
				ExprOpMathKind::Add => self + other,
				ExprOpMathKind::Sub => self - other,
				ExprOpMathKind::Mul => self * other,
				ExprOpMathKind::Div => match &round {
					Some(round) => match round {
						ExprOpMathRound::Up => (self + (other - 1)) / other,
						ExprOpMathRound::Down => self / other,
						ExprOpMathRound::Round => {
							(*self as f32 / *other as f32).round() as i32
						}
					},
					None => self / other,
				},
			})),
			RolledOp::Comparator(c) => Ok(Value::Boolean(match c {
				Comparator::Gt => *self > *other,
				Comparator::Lt => *self < *other,
				Comparator::GtEq => *self >= *other,
				Comparator::LtEq => *self <= *other,
				Comparator::Eq => *self == *other,
			})),
			_ => return same_op_err(ValueKind::Integer, op),
		}
	}
}

impl RunOp for f32 {
	fn op<R: Rng, L: ModLoader>(
		&self,
		other: &Self,
		op: &RolledOp,
		runtime: &Runtime<R, L>,
		ctx: &RuntimeContext,
	) -> Result<Value> {
		match op {
			RolledOp::Math { kind, round } => {
				let res = match kind {
					ExprOpMathKind::Add => self + other,
					ExprOpMathKind::Sub => self - other,
					ExprOpMathKind::Mul => self * other,
					ExprOpMathKind::Div => self / other,
				};
				Ok(Value::Decimal(match &round {
					Some(round) => match round {
						ExprOpMathRound::Up => res.ceil(),
						ExprOpMathRound::Down => res.floor(),
						ExprOpMathRound::Round => res.round(),
					},
					None => res,
				}))
			}
			RolledOp::Comparator(c) => Ok(Value::Boolean(match c {
				Comparator::Gt => *self > *other,
				Comparator::Lt => *self < *other,
				Comparator::GtEq => *self >= *other,
				Comparator::LtEq => *self <= *other,
				Comparator::Eq => *self == *other,
			})),
			_ => same_op_err(ValueKind::Decimal, op),
		}
	}
}

impl RunOp for String {
	fn op<R: Rng, L: ModLoader>(
		&self,
		other: &Self,
		op: &RolledOp,
		runtime: &Runtime<R, L>,
		ctx: &RuntimeContext,
	) -> Result<Value> {
		match op {
			RolledOp::Math {
				kind: ExprOpMathKind::Add,
				..
			} => Ok(Value::String(format!("{}{}", self, other))),
			_ => same_op_err(ValueKind::String, op),
		}
	}
}

impl RunOp for Vec<Value> {
	fn op<R: Rng, L: ModLoader>(
		&self,
		other: &Self,
		op: &RolledOp,
		runtime: &Runtime<R, L>,
		ctx: &RuntimeContext,
	) -> Result<Value> {
		match op {
			RolledOp::Math {
				kind: ExprOpMathKind::Add,
				..
			} => Ok(Value::Array([self.as_slice(), other.as_slice()].concat())),
			_ => same_op_err(ValueKind::Array, op),
		}
	}
}

#[derive(Clone, Debug, Copy)]
pub enum ValueKind {
	Integer,
	Decimal,
	Boolean,
	String,
	Roll,
	Array,
	Object,
	Function,
}

impl ValueKind {
	pub fn to_str(&self) -> &'static str {
		match self {
			ValueKind::Integer => K_INTEGER,
			ValueKind::Decimal => K_DECIMAL,
			ValueKind::Boolean => K_BOOLEAN,
			ValueKind::String => K_STRING,
			ValueKind::Roll => K_ROLL,
			ValueKind::Array => K_ARRAY,
			ValueKind::Object => K_OBJECT,
			ValueKind::Function => K_FUNCTION,
		}
	}
}

impl Display for ValueKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.to_str())
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Integer(v) => {
				write!(f, "{}", v.to_string().cyan())
			}
			Value::Decimal(v) => {
				write!(f, "{}", v.to_string().cyan())
			}
			Value::Boolean(v) => {
				write!(f, "{}", v.to_string().cyan())
			}
			Value::String(v) => {
				write!(f, "\"{}\"", v.to_string().cyan())
			}
			Value::Roll(v) => {
				write!(f, "{}", v.to_string().cyan())
			}
			Value::Array(v) => {
				write!(
					f,
					"[{}]",
					v.iter()
						.map(|v| format!("{}", v))
						.collect::<Vec<String>>()
						.join(", ")
				)
			}
			Value::Object(v) => {
				let mut v = v.iter().collect::<Vec<(&String, &Value)>>();
				v.sort_by(|a, b| a.0.cmp(b.0));
				write!(
					f,
					"{{{}}}",
					v.iter()
						.map(|(k, v)| format!("{}: {}", k, v))
						.collect::<Vec<String>>()
						.join(", ")
				)
			}
			Value::Function(_) => {
				write!(f, "<function>")
			}
			Value::Struct(s) => {
				write!(f, "{}", s)
			}
		}
	}
}

pub enum ValueRef<'a> {
	Owned(Value),
	Ref(&'a Value),
}

impl<'a> ValueRef<'a> {
	pub fn val(&'a self) -> &'a Value {
		match self {
			ValueRef::Owned(v) => &v,
			ValueRef::Ref(v) => v,
		}
	}
	pub fn set(&'a mut self, v: &'a Value) {
		*self = ValueRef::Ref(v)
	}
	pub fn cloned(&self) -> Value {
		self.val().clone()
	}
}

#[cfg(test)]
#[test]
fn auto_converting_ops() {
	use rand::thread_rng;

	let runtime = Runtime::new(thread_rng(), ());
	let ctx = RuntimeContext::new();
	assert_eq!(
		Value::Integer(10)
			.run_op(
				&Value::Decimal(6.9),
				&RolledOp::Math {
					kind: ExprOpMathKind::Add,
					round: None
				},
				&runtime,
				&ctx
			)
			.unwrap(),
		Value::Decimal(16.9)
	);
	assert_eq!(
		Value::String("foo ".to_string())
			.run_op(
				&Value::Decimal(69.0),
				&RolledOp::Math {
					kind: ExprOpMathKind::Add,
					round: None
				},
				&runtime,
				&ctx
			)
			.unwrap(),
		Value::String("foo 69".to_string())
	);
	assert_eq!(
		Value::Array(vec![Value::Integer(69), Value::String("nice".to_string())])
			.run_op(
				&Value::Array(vec![Value::Boolean(true),]),
				&RolledOp::Math {
					kind: ExprOpMathKind::Add,
					round: None
				},
				&runtime,
				&ctx
			)
			.unwrap(),
		Value::Array(vec![
			Value::Integer(69),
			Value::String("nice".to_string()),
			Value::Boolean(true)
		])
	);
}
