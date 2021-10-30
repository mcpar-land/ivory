use std::fmt::Display;

use crate::{Expression, ExpressionComponent};

#[derive(Clone, Debug)]
pub struct Ternary<O: Clone, T: Clone> {
	pub condition: Expression<O, T>,
	pub options: Option<Box<(Self, Self)>>,
}

pub struct TernaryExpression<O: Clone, T: Clone> {
	pub first: TernaryExpressionComponent<O, T>,
	pub pairs: Vec<TernaryPair<O, T>>,
}

pub struct TernaryPair<O: Clone, T: Clone>(
	pub O,
	pub TernaryExpressionComponent<O, T>,
);

pub enum TernaryExpressionComponent<O: Clone, T: Clone> {
	Token(T),
	Paren(Box<Ternary<O, T>>),
}

impl<O: Clone, T: Clone> Ternary<O, T> {
	pub fn map_tokens<F, Nt>(&self, f: F) -> Ternary<O, Nt>
	where
		F: Fn(&T) -> Nt + Copy,
		Nt: Clone,
	{
		Ternary {
			condition: self.condition.map_tokens(f),
			options: self.options.as_ref().map(|conditions| {
				Box::new((conditions.0.map_tokens(f), conditions.1.map_tokens(f)))
			}),
		}
	}

	pub fn map_operators<No, F>(&self, f: F) -> Ternary<No, T>
	where
		F: Fn(&O) -> No + Copy,
		No: Clone,
	{
		Ternary {
			condition: self.condition.map_operators(f),
			options: self.options.as_ref().map(|conditions| {
				Box::new((conditions.0.map_operators(f), conditions.1.map_operators(f)))
			}),
		}
	}

	pub fn try_map_tokens_components<F, Nt, E>(
		&self,
		f: F,
	) -> Result<Ternary<O, Nt>, E>
	where
		F: Fn(&T) -> Result<ExpressionComponent<O, Nt>, E> + Copy,
		Nt: Clone,
	{
		Ok(Ternary {
			condition: self.condition.try_map_tokens_components(f)?,
			options: match &self.options {
				Some(conditions) => Some(Box::new((
					conditions.0.try_map_tokens_components(f)?,
					conditions.1.try_map_tokens_components(f)?,
				))),
				None => None,
			},
		})
	}

	pub fn run_mut<M, E>(&mut self, m: M) -> Result<(), E>
	where
		M: Fn(&mut T) -> Result<(), E> + Copy,
	{
		self.condition.run_mut(m)?;
		if let Some(conditions) = self.options.as_mut() {
			let (a, b) = conditions.as_mut();
			a.run_mut(m)?;
			b.run_mut(m)?;
		}
		Ok(())
	}
}

impl<O: Clone, T: Clone, E: Clone> Ternary<O, Result<T, E>> {
	pub fn ok(self) -> Result<Ternary<O, T>, E> {
		Ok(Ternary {
			condition: self.condition.ok()?,
			options: match self.options {
				Some(conditions) => {
					Some(Box::new((conditions.0.ok()?, conditions.1.ok()?)))
				}
				None => None,
			},
		})
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
