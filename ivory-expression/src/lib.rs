use std::fmt::{Debug, Display, Formatter};

pub mod iter;

pub struct Expression<O: Clone, T: Clone> {
	pub first: ExpressionComponent<O, T>,
	pub pairs: Vec<Pair<O, T>>,
}

pub struct Pair<O: Clone, T: Clone>(pub O, pub ExpressionComponent<O, T>);

pub enum ExpressionComponent<O: Clone, T: Clone> {
	Token(T),
	Paren(Box<Expression<O, T>>),
}

impl<O: Clone, T: Clone> ExpressionComponent<O, T> {
	pub fn map_tokens<F, Nt>(&self, f: F) -> ExpressionComponent<O, Nt>
	where
		F: Fn(&T) -> Nt + Copy,
		Nt: Clone,
	{
		match self {
			ExpressionComponent::Token(t) => ExpressionComponent::Token(f(t)),
			ExpressionComponent::Paren(p) => {
				ExpressionComponent::Paren(Box::new(p.map_tokens(f)))
			}
		}
	}

	pub fn map_operators<No, F>(&self, f: F) -> ExpressionComponent<No, T>
	where
		F: Fn(&O) -> No + Copy,
		No: Clone,
	{
		match self {
			ExpressionComponent::Token(t) => ExpressionComponent::Token(t.clone()),
			ExpressionComponent::Paren(p) => {
				ExpressionComponent::Paren(Box::new(p.map_operators(f)))
			}
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

	pub fn map<No: Clone, Nt: Clone, Fo, Ft>(
		&self,
		fo: Fo,
		ft: Ft,
	) -> Expression<No, Nt>
	where
		Fo: Fn(&O) -> No + Copy,
		Ft: Fn(&T) -> Nt + Copy,
	{
		self.map_operators(fo).map_tokens(ft)
	}

	pub fn map_tokens<Nt, F>(&self, f: F) -> Expression<O, Nt>
	where
		F: Fn(&T) -> Nt + Copy,
		Nt: Clone,
	{
		let mut new_expr = Expression {
			first: self.first.map_tokens(f),
			pairs: Vec::new(),
		};
		for Pair(op, component) in &self.pairs {
			new_expr
				.pairs
				.push(Pair(op.clone(), component.map_tokens(f)))
		}
		new_expr
	}

	pub fn map_operators<No, F>(&self, f: F) -> Expression<No, T>
	where
		F: Fn(&O) -> No + Copy,
		No: Clone,
	{
		let mut new_expr = Expression {
			first: self.first.map_operators(f),
			pairs: Vec::new(),
		};
		for Pair(op, component) in &self.pairs {
			new_expr.pairs.push(Pair(f(op), component.map_operators(f)))
		}
		new_expr
	}

	/// run a closure over every pair.
	/// return true to keep the pair in the result
	/// return false to drop it from the result
	/// you can also modify the LHS value
	pub fn collapse<M, E>(&self, m: M) -> Result<Expression<O, T>, E>
	where
		M: Fn(
				&mut ExpressionComponent<O, T>,
				&O,
				&ExpressionComponent<O, T>,
			) -> Result<bool, E>
			+ Copy,
	{
		let mut first = match &self.first {
			ExpressionComponent::Token(_) => self.first.clone(),
			ExpressionComponent::Paren(paren) => {
				ExpressionComponent::Paren(Box::new(paren.collapse(m)?))
			}
		};
		let mut pairs: Vec<Option<Pair<O, T>>> = Vec::new();

		let parens_collapsed = self
			.pairs
			.iter()
			.map(|pair| {
				Ok(match &pair.1 {
					ExpressionComponent::Token(_) => pair.clone(),
					ExpressionComponent::Paren(paren) => Pair(
						pair.0.clone(),
						ExpressionComponent::Paren(Box::new(paren.collapse(m)?)),
					),
				})
			})
			.collect::<Result<Vec<Pair<O, T>>, E>>()?;
		for (i, pair) in parens_collapsed.iter().enumerate() {
			let mut lhs = if i == 0 {
				&self.first
			} else {
				&parens_collapsed[i - 1].1
			}
			.clone();
			if m(&mut lhs, &pair.0, &pair.1)? {
				pairs.push(Some(pair.clone()));
			} else {
				pairs.push(None);
			}

			if i == 0 {
				first = lhs;
			} else {
				if let Some(Some(prev)) = pairs.get_mut(i - 1) {
					prev.1 = lhs;
				}
			}
		}

		Ok(Expression {
			first,
			pairs: pairs.into_iter().filter_map(|v| v).collect(),
		})
	}
}

impl<O: Clone, T: Clone, E: Clone> Expression<O, Result<T, E>> {
	pub fn ok(self) -> Result<Expression<O, T>, E> {
		todo!();
	}
}

impl<O: Clone, T: Default + Clone> Default for Expression<O, T> {
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

impl<O: Display + Clone, T: Display + Clone> Display
	for ExpressionComponent<O, T>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		match self {
			ExpressionComponent::Token(token) => write!(f, "{}", token),
			ExpressionComponent::Paren(paren) => write!(f, "( {} )", paren),
		}
	}
}

impl<O: Display + Clone, T: Display + Clone> Display for Pair<O, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "{} {}", self.0, self.1)
	}
}

impl<O: Display + Clone, T: Display + Clone> Display for Expression<O, T> {
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

impl<O: Debug + Clone, T: Debug + Clone> Debug for ExpressionComponent<O, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ExpressionComponent::Token(token) => write!(f, "{:?}", token),
			ExpressionComponent::Paren(paren) => write!(f, "( {:?} )", paren),
		}
	}
}

impl<O: Debug + Clone, T: Debug + Clone> Debug for Pair<O, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?} {:?}", self.0, self.1)
	}
}

impl<O: Debug + Clone, T: Debug + Clone> Debug for Expression<O, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			self
				.pairs
				.iter()
				.fold(format!("{:?}", self.first), |s, component| {
					format!("{}, {:?}", s, component)
				})
		)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[derive(Clone, Debug)]
	enum Op {
		A, // does nothing
		B, // adds one to previous value, deletes itself
		C, // adds one to the previous value, does not delete itself
		D, // panic instantly
	}

	fn sample_expression() -> Expression<Op, i32> {
		Expression {
			first: ExpressionComponent::Token(0),
			pairs: vec![
				Pair(Op::C, ExpressionComponent::Token(1)),
				Pair(Op::B, ExpressionComponent::Token(2)),
				Pair(Op::C, ExpressionComponent::Token(69)),
				Pair(Op::A, ExpressionComponent::Token(4)),
				Pair(
					Op::A,
					ExpressionComponent::Paren(Box::new(Expression {
						first: ExpressionComponent::Token(60),
						pairs: vec![
							Pair(Op::B, ExpressionComponent::Token(9)),
							Pair(Op::C, ExpressionComponent::Token(9)),
						],
					})),
				),
				Pair(Op::C, ExpressionComponent::Token(5)),
				Pair(Op::A, ExpressionComponent::Token(6)),
			],
		}
	}

	#[test]
	fn test_expr_map() {
		let expr = sample_expression();

		let new_expr = expr.map(|_| "a cool op", |t| *t as f32);

		println!("{:?}\n{:?}", expr, new_expr);
	}

	#[test]
	fn test_expr_collapse() {
		let expr: Expression<Op, i32> = sample_expression();

		let add: i32 = 1;

		let new_expr: Expression<Op, i32> = expr
			.collapse::<_, &'static str>(|lhs, op, _| match op {
				Op::A => Ok(true),
				Op::B => {
					println!("RUNNING B ON {:?}", lhs);
					*lhs = lhs.map_tokens(|v| *v + add);
					Ok(false)
				}
				Op::C => {
					println!("RUNNING C ON {:?}", lhs);
					*lhs = lhs.map_tokens(|v| *v + add);
					Ok(true)
				}
				Op::D => Err("Used op d which causes an error"),
			})
			.unwrap();

		println!("{:?}", expr);
		println!("{:?}", new_expr);
	}
}
