use std::fmt::Display;

use crate::{EvaluatedTernary, Expression, ExpressionComponent, Pair};

type EvalFn<Ctx, O, T, E> =
	fn(&Ctx, expr: &Expression<O, T>) -> Result<bool, E>;

#[derive(Clone, Debug)]
pub struct Ternary<O: Clone, T: Clone> {
	pub condition: TernaryExpression<O, T>,
	pub options: Option<Box<(Self, Self)>>,
}

#[derive(Clone, Debug)]
pub struct TernaryExpression<O: Clone, T: Clone> {
	pub first: TernaryExpressionComponent<O, T>,
	pub pairs: Vec<TernaryPair<O, T>>,
}

#[derive(Clone, Debug)]
pub struct TernaryPair<O: Clone, T: Clone>(
	pub O,
	pub TernaryExpressionComponent<O, T>,
);

#[derive(Clone, Debug)]
pub enum TernaryExpressionComponent<O: Clone, T: Clone> {
	Token(T),
	Paren(Box<Ternary<O, T>>),
}

impl<O: Clone, T: Clone> Ternary<O, T> {
	pub fn evaluate<Ctx, E>(
		self,
		ctx: &Ctx,
		f: EvalFn<Ctx, O, T, E>,
	) -> Result<Expression<O, T>, E> {
		if let Some(options) = self.options {
			let e = self.condition.evaluate(ctx, f)?;
			if f(ctx, &e)? {
				Ok(Expression {
					ternary_condition: Some(EvaluatedTernary {
						condition: Box::new(e),
						result: true,
					}),
					..options.0.evaluate(ctx, f)?
				})
			} else {
				Ok(Expression {
					ternary_condition: Some(EvaluatedTernary {
						condition: Box::new(e),
						result: false,
					}),
					..options.1.evaluate(ctx, f)?
				})
			}
		} else {
			self.condition.evaluate(ctx, f)
		}
	}
}

impl<O: Clone, T: Clone> TernaryExpression<O, T> {
	pub fn evaluate<Ctx, E>(
		self,
		ctx: &Ctx,
		f: EvalFn<Ctx, O, T, E>,
	) -> Result<Expression<O, T>, E> {
		Ok(Expression {
			ternary_condition: None,
			first: self.first.evaluate(ctx, f)?,
			pairs: {
				let mut pairs = Vec::new();
				for pair in self.pairs {
					pairs.push(pair.evaluate(ctx, f)?);
				}
				pairs
			},
		})
	}
}

impl<O: Clone, T: Clone> TernaryExpressionComponent<O, T> {
	pub fn evaluate<Ctx, E>(
		self,
		ctx: &Ctx,
		f: EvalFn<Ctx, O, T, E>,
	) -> Result<ExpressionComponent<O, T>, E> {
		match self {
			TernaryExpressionComponent::Token(token) => {
				Ok(ExpressionComponent::Token(token))
			}
			TernaryExpressionComponent::Paren(ternary) => Ok(
				ExpressionComponent::Paren(Box::new(ternary.evaluate(ctx, f)?)),
			),
		}
	}
}

impl<O: Clone, T: Clone> TernaryPair<O, T> {
	pub fn evaluate<Ctx, E>(
		self,
		ctx: &Ctx,
		f: EvalFn<Ctx, O, T, E>,
	) -> Result<Pair<O, T>, E> {
		Ok(Pair(self.0, self.1.evaluate(ctx, f)?))
	}
}

// debug and display impls

impl<O: Display + Clone, T: Display + Clone> Display
	for TernaryExpressionComponent<O, T>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl<O: Display + Clone, T: Display + Clone> Display for TernaryPair<O, T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl<O: Clone + Display, T: Clone + Display> Display
	for TernaryExpression<O, T>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl<O: Clone + Display, T: Clone + Display> Display for Ternary<O, T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(conditions) = &self.options {
			write!(
				f,
				"{} ? {} : {}",
				self.condition, conditions.0, conditions.1
			)
		} else {
			write!(f, "{}", self.condition)
		}
	}
}
