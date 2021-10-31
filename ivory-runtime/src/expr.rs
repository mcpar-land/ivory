use std::convert::TryInto;
use std::fmt::Display;

use crate::prec;
use crate::prec::{Assoc, Climber, Token};
use ivory_expression::{Expression, ExpressionComponent, Pair};
use ivory_tokenizer::expression::dice_ops::{DiceOp, DiceOpCmp};
use ivory_tokenizer::expression::math::{
	ExprOpMath, ExprOpMathKind, ExprOpMathRound,
};
use ivory_tokenizer::expression::{ExpressionToken, Op};
use lazy_static::lazy_static;

use crate::runtime::{Runtime, RuntimeContext};
use crate::value::Value;
use crate::{Result, RuntimeError};

#[derive(Clone, Debug)]
pub enum RolledOp {
	Math {
		kind: ExprOpMathKind,
		round: Option<ExprOpMathRound>,
	},
	Ternary(Box<Expression<RolledOp, Value>>),
}

impl Display for RolledOp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Math { kind, round } => match round {
				Some(round) => write!(f, "{}{}", kind, round),
				None => write!(f, "{}", kind),
			},
			Self::Ternary(expr) => write!(f, "? {} :", expr),
		}
	}
}

type Component = ExpressionComponent<RolledOp, Value>;

pub type RolledExpression = prec::Expression<RolledOp, Component>;

fn into_prec(expr: Expression<RolledOp, Value>) -> RolledExpression {
	prec::Expression::new(
		expr.first,
		expr.pairs.into_iter().map(|Pair(a, b)| (a, b)).collect(),
	)
}

impl prec::Token<Value, RuntimeError> for Component {
	fn convert(self, _: &()) -> Result<Value> {
		Ok(match self {
			Self::Paren(expr) => PREC_CLIMBER.process(&into_prec(*expr), &())?,
			Self::Token(v) => v,
		})
	}
}

impl TryInto<Value> for Expression<RolledOp, Value> {
	type Error = RuntimeError;

	fn try_into(self) -> Result<Value> {
		PREC_CLIMBER.process(&into_prec(self), &())
	}
}

fn prec_handler(
	lhs: Component,
	op: RolledOp,
	rhs: Component,
	_: &(),
) -> Result<Component> {
	let lhs = lhs.convert(&())?;
	let rhs = rhs.convert(&())?;
	Ok(ExpressionComponent::Token(lhs.run_op(&rhs, &op)?))
}

lazy_static! {
	pub static ref PREC_CLIMBER: Climber<RolledOp, Component, Value, RuntimeError> =
		Climber::new(
			|op, _| {
				match op {
					RolledOp::Ternary(inner) => (0, Assoc::Right),
					RolledOp::Math { kind, round } => match kind {
						ExprOpMathKind::Add | ExprOpMathKind::Sub => (1, Assoc::Left),
						ExprOpMathKind::Mul | ExprOpMathKind::Div => (2, Assoc::Right),
					},
				}
			},
			prec_handler
		);
}
