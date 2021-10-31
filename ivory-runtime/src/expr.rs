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
use rand::prelude::ThreadRng;
use rand::Rng;

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

pub fn into_prec(expr: Expression<RolledOp, Value>) -> RolledExpression {
	prec::Expression::new(
		expr.first,
		expr.pairs.into_iter().map(|Pair(a, b)| (a, b)).collect(),
	)
}

impl prec::Token<Value, RuntimeError> for Component {
	fn convert<R: Rng>(
		self,
		runtime: &Runtime<R>,
		ctx: &RuntimeContext,
	) -> Result<Value> {
		Ok(match self {
			Self::Paren(expr) => {
				runtime.climber.process(&into_prec(*expr), runtime, ctx)?
			}
			Self::Token(v) => v,
		})
	}
}
