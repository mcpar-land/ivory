use ivory_tokenizer::{
	expression::{
		math::{ExprOpMath, ExprOpMathKind, ExprOpMathRound},
		Op,
	},
	values::{
		array::ArrayValue, boolean::BooleanValue, decimal::DecimalValue,
		function::FunctionValue, integer::IntegerValue, object::ObjectValue,
		string::StringValue, struct_instance::StructInstance,
	},
};

use crate::{
	error::RuntimeError,
	roll::Roll,
	runtime::{Runtime, RuntimeContext},
};
use crate::{expr::RolledExprVal, Result};
use lazy_static::lazy_static;
use prec::{Assoc, Climber, Rule, Token};
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
	Roll(()),
	Array(Vec<Value>),
	Object(HashMap<String, Value>),
	Function(FunctionValue),
}

impl Value {
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
		}
	}

	pub fn run_op(&self, rhs: &Value, op: &Op) -> Result<Value> {
		use Value::*;
		match (self, rhs) {
			(Integer(a), Integer(b)) => a.op(b, op),
			(Integer(a), Decimal(b)) => (*a as f32).op(b, op),
			(Integer(a), Boolean(b)) => a.op(&(*b as i32), op),
			(Integer(a), String(b)) => a.to_string().op(b, op),
			(Integer(a), Roll(b)) => todo!(),
			(Decimal(a), Integer(b)) => a.op(&(*b as f32), op),
			(Decimal(a), Decimal(b)) => a.op(b, op),
			(Decimal(a), Boolean(b)) => a.op(&(*b as i32 as f32), op),
			(Decimal(a), String(b)) => a.to_string().op(b, op),
			(Decimal(a), Roll(b)) => todo!(),
			(Boolean(a), Integer(b)) => (*a as i32).op(b, op),
			(Boolean(a), Decimal(b)) => (*a as i32 as f32).op(b, op),
			(Boolean(a), Boolean(b)) => (*a as i32).op(&(*b as i32), op),
			(Boolean(a), String(b)) => a.to_string().op(b, op),
			(Boolean(a), Roll(b)) => todo!(),
			(String(a), Integer(b)) => a.op(&b.to_string(), op),
			(String(a), Decimal(b)) => a.op(&b.to_string(), op),
			(String(a), Boolean(b)) => a.op(&b.to_string(), op),
			(String(a), String(b)) => a.op(b, op),
			(String(a), Roll(b)) => todo!(),
			(Roll(a), Integer(b)) => todo!(),
			(Roll(a), Decimal(b)) => todo!(),
			(Roll(a), Boolean(b)) => todo!(),
			(Roll(a), String(b)) => todo!(),
			(Roll(a), Roll(b)) => todo!(),
			(Array(a), Array(b)) => a.op(b, op),
			(a, b) => Err(RuntimeError::CannotRunOp(a.kind(), op.clone(), b.kind())),
		}
	}

	pub fn to_integer(&self) -> Result<i32> {
		todo!();
	}

	pub fn to_decimal(&self) -> Result<f32> {
		todo!();
	}

	pub fn to_boolean(&self) -> Result<bool> {
		todo!();
	}

	pub fn to_string(&self) -> Result<bool> {
		todo!();
	}

	pub fn to_roll(&self) -> Result<Roll> {
		todo!();
	}

	pub fn to_array(&self) -> Result<Vec<Value>> {
		todo!();
	}

	pub fn to_object(&self) -> Result<HashMap<String, Value>> {
		todo!();
	}

	pub fn to_function(&self) -> Result<FunctionValue> {
		todo!();
	}

	pub fn from_token(
		token: &ivory_tokenizer::values::Value,
		runtime: &Runtime,
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
					.map(|v| runtime.execute(ctx, v))
					.collect::<Result<Vec<Value>>>()?,
			),
			ivory_tokenizer::values::Value::Object(ObjectValue(v)) => Value::Object(
				v.iter()
					.map(|(n, v)| Ok((n.0.clone(), runtime.execute(ctx, v)?)))
					.collect::<Result<HashMap<String, Value>>>()?,
			),
			ivory_tokenizer::values::Value::Struct(s) => todo!(),
			ivory_tokenizer::values::Value::Function(f) => Value::Function(f.clone()),
		})
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

fn same_op_err(kind: ValueKind, op: &Op) -> Result<Value> {
	Err(RuntimeError::CannotRunOp(kind, op.clone(), kind))
}

trait RunOp {
	fn op(&self, other: &Self, op: &Op) -> Result<Value>;
}

impl RunOp for i32 {
	fn op(&self, other: &Self, op: &Op) -> Result<Value> {
		Ok(Value::Integer(match op {
			Op::Math(op) => match op.kind {
				ExprOpMathKind::Add => self + other,
				ExprOpMathKind::Sub => self - other,
				ExprOpMathKind::Mul => self * other,
				ExprOpMathKind::Div => match &op.round {
					Some(round) => match round {
						ExprOpMathRound::Up => (self + (other - 1)) / other,
						ExprOpMathRound::Down => self / other,
						ExprOpMathRound::Round => {
							(*self as f32 / *other as f32).round() as i32
						}
					},
					None => self / other,
				},
			},
			_ => return same_op_err(ValueKind::Integer, op),
		}))
	}
}

impl RunOp for f32 {
	fn op(&self, other: &Self, op: &Op) -> Result<Value> {
		match op {
			Op::Math(op) => {
				let res = match op.kind {
					ExprOpMathKind::Add => self + other,
					ExprOpMathKind::Sub => self - other,
					ExprOpMathKind::Mul => self * other,
					ExprOpMathKind::Div => self / other,
				};
				Ok(Value::Decimal(match &op.round {
					Some(round) => match round {
						ExprOpMathRound::Up => res.ceil(),
						ExprOpMathRound::Down => res.floor(),
						ExprOpMathRound::Round => res.round(),
					},
					None => res,
				}))
			}
			_ => same_op_err(ValueKind::Decimal, op),
		}
	}
}

impl RunOp for String {
	fn op(&self, other: &Self, op: &Op) -> Result<Value> {
		match op {
			Op::Math(ExprOpMath {
				kind: ExprOpMathKind::Add,
				..
			}) => Ok(Value::String(format!("{}{}", self, other))),
			_ => same_op_err(ValueKind::String, op),
		}
	}
}

impl RunOp for Vec<Value> {
	fn op(&self, other: &Self, op: &Op) -> Result<Value> {
		match op {
			Op::Math(ExprOpMath {
				kind: ExprOpMathKind::Add,
				..
			}) => Ok(Value::Array([self.as_slice(), other.as_slice()].concat())),
			_ => same_op_err(ValueKind::Array, op),
		}
	}
}

fn prec_handler(
	lhs: RolledExprVal,
	op: Op,
	rhs: RolledExprVal,
	_: &(),
) -> Result<RolledExprVal> {
	let lhs = lhs.convert(&())?;
	let rhs = rhs.convert(&())?;
	Ok(RolledExprVal::Value(lhs.run_op(&rhs, &op)?))
}

fn every_rule(src: ExprOpMathKind) -> Rule<Op> {
	let mut r = Rule::new(
		Op::Math(ExprOpMath {
			kind: src,
			round: None,
		}),
		Assoc::Left,
	);
	for round in [
		ExprOpMathRound::Down,
		ExprOpMathRound::Up,
		ExprOpMathRound::Round,
	] {
		r = r
			| Rule::new(
				Op::Math(ExprOpMath {
					kind: src,
					round: Some(round),
				}),
				Assoc::Left,
			);
	}
	r
}

lazy_static! {
	pub static ref PREC_CLIMBER: Climber<Op, RolledExprVal, Value, RuntimeError> =
		Climber::new(
			vec![
				every_rule(ExprOpMathKind::Add) | every_rule(ExprOpMathKind::Sub),
				every_rule(ExprOpMathKind::Mul) | every_rule(ExprOpMathKind::Div)
			],
			prec_handler
		);
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
				write!(f, "{}", v)
			}
			Value::Decimal(v) => {
				write!(f, "{}", v)
			}
			Value::Boolean(v) => {
				write!(f, "{}", v)
			}
			Value::String(v) => {
				write!(f, "\"{}\"", v)
			}
			Value::Roll(v) => {
				todo!();
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
		}
	}
}

#[cfg(test)]
#[test]
fn auto_converting_ops() {
	assert_eq!(
		Value::Integer(10)
			.run_op(
				&Value::Decimal(6.9),
				&Op::Math(ExprOpMath {
					kind: ExprOpMathKind::Add,
					round: None
				})
			)
			.unwrap(),
		Value::Decimal(16.9)
	);
	assert_eq!(
		Value::String("foo ".to_string())
			.run_op(
				&Value::Decimal(69.0),
				&Op::Math(ExprOpMath {
					kind: ExprOpMathKind::Add,
					round: None
				})
			)
			.unwrap(),
		Value::String("foo 69".to_string())
	);
	assert_eq!(
		Value::Array(vec![Value::Integer(69), Value::String("nice".to_string())])
			.run_op(
				&Value::Array(vec![Value::Boolean(true),]),
				&Op::Math(ExprOpMath {
					kind: ExprOpMathKind::Add,
					round: None
				})
			)
			.unwrap(),
		Value::Array(vec![
			Value::Integer(69),
			Value::String("nice".to_string()),
			Value::Boolean(true)
		])
	);
}
