use std::fmt::{Debug, Display, Formatter};

pub mod iter;

pub struct Expression<O, T> {
	pub first: ExpressionComponent<O, T>,
	pub pairs: Vec<Pair<O, T>>,
}

pub struct Pair<O, T>(O, ExpressionComponent<O, T>);

pub enum ExpressionComponent<O, T> {
	Token(T),
	Paren(Box<Expression<O, T>>),
}

impl<O: Clone, T: Clone> ExpressionComponent<O, T> {
	pub fn map<F, Nt>(&self, f: F) -> ExpressionComponent<O, Nt>
	where
		F: Fn(&T) -> Nt + Copy,
	{
		match self {
			ExpressionComponent::Token(t) => ExpressionComponent::Token(f(t)),
			ExpressionComponent::Paren(p) => ExpressionComponent::Paren(Box::new(
				p.map(f, |_, _, rhs| (None, Some(rhs.map(f)))),
			)),
		}
	}
}

impl<O: Clone, T: Clone> Expression<O, T> {
	/// Returns self.first if i is 0
	pub fn get_component(&self, i: usize) -> Option<&ExpressionComponent<O, T>> {
		if i == 0 {
			Some(&self.first)
		} else {
			self.pairs.get(i - 1).map(|Pair(_, v)| v)
		}
	}
	pub fn get_component_mut(
		&mut self,
		i: usize,
	) -> Option<&mut ExpressionComponent<O, T>> {
		if i == 0 {
			Some(&mut self.first)
		} else {
			self.pairs.get_mut(i - 1).map(|Pair(_, v)| v)
		}
	}
	/// Returns None if i is 0
	pub fn get_op(&self, i: usize) -> Option<&O> {
		if i == 0 {
			None
		} else {
			self.pairs.get(i - 1).map(|Pair(o, _)| o)
		}
	}

	/// The Op is None if i is 0
	pub fn get(
		&self,
		i: usize,
	) -> Option<(Option<&O>, &ExpressionComponent<O, T>)> {
		if i == 0 {
			Some((None, &self.first))
		} else {
			self.pairs.get(i - 1).map(|Pair(o, v)| (Some(o), v))
		}
	}

	/// Run a map over every pair.
	/// You can edit the component that comes before each pair.
	/// By returning `None`, a pair isn't added to the new pair.
	pub fn map<Nt, F, M>(&self, f: F, m: M) -> Expression<O, Nt>
	where
		F: Fn(&T) -> Nt + Copy,
		M: Fn(
			&ExpressionComponent<O, T>,
			&O,
			&ExpressionComponent<O, T>,
		) -> (
			Option<ExpressionComponent<O, Nt>>,
			Option<ExpressionComponent<O, Nt>>,
		),
	{
		let mut new_expr = Expression {
			first: self.first.map(f),
			pairs: Vec::new(),
		};
		let mut new_expr_len = 0usize;
		for i in 0..self.pairs.len() {
			let lhs = if i == 0 {
				&self.first
			} else {
				&self.pairs[i - 1].1
			};
			let (lhs, rhs) = m(lhs, &self.pairs[i].0, &self.pairs[i].1);

			if let Some(lhs) = lhs {
				let a = if new_expr_len == 0 {
					&mut new_expr.first
				} else {
					&mut new_expr.pairs[new_expr_len - 1].1
				};
				*a = lhs;
			}

			if let Some(rhs) = rhs {
				new_expr.pairs.push(Pair(self.pairs[i].0.clone(), rhs));
				new_expr_len += 1;
			}
		}
		new_expr
	}
}

impl<O, T: Default> Default for Expression<O, T> {
	fn default() -> Self {
		Self {
			first: ExpressionComponent::Token(T::default()),
			pairs: Vec::new(),
		}
	}
}

impl<O: Clone, T: Clone> Clone for Expression<O, T> {
	fn clone(&self) -> Self {
		Self {
			first: self.first.clone(),
			pairs: self.pairs.clone(),
		}
	}
}

impl<O: Clone, T: Clone> Clone for ExpressionComponent<O, T> {
	fn clone(&self) -> Self {
		match self {
			Self::Token(arg0) => Self::Token(arg0.clone()),
			Self::Paren(arg0) => Self::Paren(arg0.clone()),
		}
	}
}

impl<O: Clone, T: Clone> Clone for Pair<O, T> {
	fn clone(&self) -> Self {
		Self(self.0.clone(), self.1.clone())
	}
}

// display and debug impls

impl<O: Display, T: Display> Display for ExpressionComponent<O, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		match self {
			ExpressionComponent::Token(token) => write!(f, "{}", token),
			ExpressionComponent::Paren(paren) => write!(f, "( {} )", paren),
		}
	}
}

impl<O: Display, T: Display> Display for Pair<O, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "{} {}", self.0, self.1)
	}
}

impl<O: Display, T: Display> Display for Expression<O, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(
			f,
			"{}",
			self
				.pairs
				.iter()
				.fold(format!("{}", self.first), |s, component| {
					format!("{} {}", s, component)
				})
		)
	}
}

impl<O: Debug, T: Debug> Debug for ExpressionComponent<O, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ExpressionComponent::Token(token) => write!(f, "{:?}", token),
			ExpressionComponent::Paren(paren) => write!(f, "( {:?} )", paren),
		}
	}
}

impl<O: Debug, T: Debug> Debug for Pair<O, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?} {:?}", self.0, self.1)
	}
}

impl<O: Debug, T: Debug> Debug for Expression<O, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			self
				.pairs
				.iter()
				.fold(format!("{:?}", self.first), |s, component| {
					format!("{} {:?}", s, component)
				})
		)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_expr_map() {
		#[derive(Clone, Debug)]
		enum Op {
			A, // does nothing
			B, // adds one to previous value, deletes itself
			C, // adds one to the previous value, does not delete itself
		}

		let expr: Expression<Op, i32> = Expression {
			first: ExpressionComponent::Token(0),
			pairs: vec![
				Pair(Op::A, ExpressionComponent::Token(1)),
				Pair(Op::B, ExpressionComponent::Token(2)),
				Pair(Op::C, ExpressionComponent::Token(69)),
				Pair(Op::A, ExpressionComponent::Token(4)),
				Pair(Op::A, ExpressionComponent::Token(5)),
				Pair(Op::A, ExpressionComponent::Token(6)),
			],
		};

		let new_expr: Expression<Op, f32> = expr.map(
			|f| *f as f32,
			|lhs, op, rhs| match op {
				Op::A => (None, Some(rhs.map(|i| *i as f32))),
				Op::B => match lhs {
					ExpressionComponent::Token(prev) => {
						(Some(ExpressionComponent::Token(*prev as f32 + 1.0)), None)
					}
					ExpressionComponent::Paren(_) => (None, None),
				},
				Op::C => match lhs {
					ExpressionComponent::Token(prev) => (
						Some(ExpressionComponent::Token(*prev as f32 + 1.0)),
						Some(rhs.map(|i| *i as f32)),
					),
					_ => (None, Some(rhs.map(|i| *i as f32))),
				},
			},
		);

		println!("{:?}", expr);
		println!("{:?}", new_expr);
	}
}
