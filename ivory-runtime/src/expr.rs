use std::convert::TryInto;

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

type Component = ExpressionComponent<ExprOpMath, Value>;

pub type RolledExpression = prec::Expression<ExprOpMath, Component>;

fn into_prec(expr: Expression<ExprOpMath, Value>) -> RolledExpression {
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

impl TryInto<Value> for Expression<ExprOpMath, Value> {
	type Error = RuntimeError;

	fn try_into(self) -> Result<Value> {
		PREC_CLIMBER.process(&into_prec(self), &())
	}
}

fn prec_handler(
	lhs: Component,
	op: ExprOpMath,
	rhs: Component,
	_: &(),
) -> Result<Component> {
	let lhs = lhs.convert(&())?;
	let rhs = rhs.convert(&())?;
	Ok(ExpressionComponent::Token(lhs.run_op(&rhs, &Op::Math(op))?))
}

fn every_rule(src: ExprOpMathKind) -> Rule<ExprOpMath> {
	let mut r = Rule::new(
		ExprOpMath {
			kind: src,
			round: None,
		},
		Assoc::Left,
	);
	for round in [
		ExprOpMathRound::Down,
		ExprOpMathRound::Up,
		ExprOpMathRound::Round,
	] {
		r = r
			| Rule::new(
				ExprOpMath {
					kind: src,
					round: Some(round),
				},
				Assoc::Left,
			);
	}
	r
}

lazy_static! {
	pub static ref PREC_CLIMBER: Climber<ExprOpMath, Component, Value, RuntimeError> =
		Climber::new(
			|op, _| {
				match op.kind {
					ExprOpMathKind::Add | ExprOpMathKind::Sub => todo!(),
					ExprOpMathKind::Mul | ExprOpMathKind::Div => todo!(),
				}
			},
			prec_handler
		);
}
