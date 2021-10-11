use ivory_tokenizer::expression::dice_ops::{DiceOp, DiceOpCmp};
use ivory_tokenizer::expression::math::{ExprOpMath, ExprOpMathKind};
use ivory_tokenizer::expression::{
	Expression, ExpressionComponent, ExpressionPair, Op,
};

use crate::runtime::{Runtime, RuntimeContext};
use crate::value::{Value, PREC_CLIMBER};
use crate::{Result, RuntimeError};

pub type RolledExpression = prec::Expression<Op, RolledExprVal>;

#[derive(Clone, Debug)]
pub enum RolledExprVal {
	Value(Value),
	Paren(Box<RolledExpression>),
}

impl prec::Token<Value, RuntimeError> for RolledExprVal {
	fn convert(self, ctx: &()) -> Result<Value> {
		Ok(match self {
			RolledExprVal::Paren(expr) => PREC_CLIMBER.process(expr.as_ref(), &())?,
			RolledExprVal::Value(v) => v,
		})
	}
}

pub fn roll_expression(
	expr: &Expression,
	runtime: &Runtime,
	ctx: &RuntimeContext,
) -> Result<RolledExpression> {
	fn convert(
		c: &ExpressionComponent,
		runtime: &Runtime,
		ctx: &RuntimeContext,
	) -> Result<RolledExprVal> {
		Ok(match c {
			ExpressionComponent::Value(val) => {
				RolledExprVal::Value(Value::from_token(val, runtime, ctx)?)
			}
			ExpressionComponent::Accessor(accessor) => {
				RolledExprVal::Value(runtime.access(ctx, &accessor)?)
			}
			ExpressionComponent::Paren(paren) => {
				RolledExprVal::Paren(Box::new(roll_expression(&paren, runtime, ctx)?))
			}
		})
	}

	let first = convert(&expr.first, runtime, ctx)?;
	let mut pairs = vec![];
	for ExpressionPair { op, component } in &expr.pairs {
		pairs.push((op.clone(), convert(&component, runtime, ctx)?));
	}

	Ok(prec::Expression::new(first, pairs))
}
