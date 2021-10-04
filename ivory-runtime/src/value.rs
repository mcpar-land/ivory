use ivory_tokenizer::{
	expression::math::{ExprOpMath, ExprOpMathKind, ExprOpMathRound},
	values::function::FunctionValue,
};

use crate::error::RuntimeError;
use crate::Result;
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
	pub fn kind_str(&self) -> &'static str {
		match self {
			Value::Integer(_) => K_INTEGER,
			Value::Decimal(_) => K_DECIMAL,
			Value::Boolean(_) => K_BOOLEAN,
			Value::String(_) => K_STRING,
			Value::Roll(_) => K_ROLL,
			Value::Array(_) => K_ARRAY,
			Value::Object(_) => K_OBJECT,
			Value::Function(_) => K_FUNCTION,
		}
	}

	pub fn run_op(&self, rhs: &Value, op: &ExprOpMath) -> Result<Value> {
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
			(a, b) => Err(RuntimeError::CannotRunOp(
				a.kind_str(),
				op.clone(),
				b.kind_str(),
			)),
		}
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

fn same_op_err(kind: &'static str, op: &ExprOpMath) -> Result<Value> {
	Err(RuntimeError::CannotRunOp(kind, op.clone(), kind))
}

trait RunOp {
	fn op(&self, other: &Self, op: &ExprOpMath) -> Result<Value>;
}

impl RunOp for i32 {
	fn op(&self, other: &Self, op: &ExprOpMath) -> Result<Value> {
		Ok(Value::Integer(match op.kind {
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
		}))
	}
}

impl RunOp for f32 {
	fn op(&self, other: &Self, op: &ExprOpMath) -> Result<Value> {
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
}

impl RunOp for String {
	fn op(&self, other: &Self, op: &ExprOpMath) -> Result<Value> {
		match op.kind {
			ExprOpMathKind::Add => Ok(Value::String(format!("{}{}", self, other))),
			_ => same_op_err(K_STRING, op),
		}
	}
}

impl RunOp for Vec<Value> {
	fn op(&self, other: &Self, op: &ExprOpMath) -> Result<Value> {
		match op.kind {
			ExprOpMathKind::Add => {
				Ok(Value::Array([self.as_slice(), other.as_slice()].concat()))
			}
			_ => same_op_err(K_ARRAY, op),
		}
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
				&ExprOpMath {
					kind: ExprOpMathKind::Add,
					round: None
				}
			)
			.unwrap(),
		Value::Decimal(16.9)
	);
	assert_eq!(
		Value::String("foo ".to_string())
			.run_op(
				&Value::Decimal(69.0),
				&ExprOpMath {
					kind: ExprOpMathKind::Add,
					round: None
				}
			)
			.unwrap(),
		Value::String("foo 69".to_string())
	);
	assert_eq!(
		Value::Array(vec![Value::Integer(69), Value::String("nice".to_string())])
			.run_op(
				&Value::Array(vec![Value::Boolean(true),]),
				&ExprOpMath {
					kind: ExprOpMathKind::Add,
					round: None
				}
			)
			.unwrap(),
		Value::Array(vec![
			Value::Integer(69),
			Value::String("nice".to_string()),
			Value::Boolean(true)
		])
	);
}
