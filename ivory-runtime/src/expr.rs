use std::fmt::Display;

use crate::mod_loader::ModLoader;
use crate::prec;
use ivory_expression::{Expression, ExpressionComponent, Pair};

use ivory_tokenizer::expression::logic::{Comparator, LogicOp};
use ivory_tokenizer::expression::math::{ExprOpMathKind, ExprOpMathRound};

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
	Comparator(Comparator),
	Logic(LogicOp),
}

impl Display for RolledOp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Math { kind, round } => match round {
				Some(round) => write!(f, "{}{}", kind, round),
				None => write!(f, "{}", kind),
			},
			Self::Ternary(expr) => write!(f, "? {} :", expr),

			RolledOp::Comparator(c) => write!(f, "{}", c),
			RolledOp::Logic(l) => write!(f, "{}", l),
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
	fn convert<R: Rng, L: ModLoader>(
		self,
		runtime: &Runtime<R, L>,
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
