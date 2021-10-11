use crate::{Expression, ExpressionComponent, Pair};

pub struct OpIterator<'a, O, T> {
	first: Option<&'a ExpressionComponent<O, T>>,
	pairs: &'a [Pair<O, T>],
	parent: Option<Box<OpIterator<'a, O, T>>>,
	did_first_paren: bool,
}

pub type OperatorSet<'a, O, T> = (
	&'a ExpressionComponent<O, T>,
	&'a O,
	&'a ExpressionComponent<O, T>,
);

impl<'a, O, T> OpIterator<'a, O, T> {
	fn move_into_paren(&mut self, first: bool) -> bool {
		let possible_paren = self.pairs.get(if first { 0 } else { 1 });
		if let Some(Pair(_, ExpressionComponent::Paren(paren))) = possible_paren {
			self.pairs = &self.pairs[1..];
			self.become_parent_of(&paren);
			true
		} else {
			false
		}
	}
	fn become_parent_of(&mut self, new: &'a Expression<O, T>) {
		*self = OpIterator {
			first: Some(&new.first),
			pairs: new.pairs.as_slice(),
			parent: Some(Box::new(std::mem::take(self))),
			..Default::default()
		};
	}
	fn become_parent(&mut self) -> Option<OperatorSet<'a, O, T>> {
		match self.parent.take() {
			Some(parent) => {
				*self = *parent;
				self.next()
			}
			None => None,
		}
	}
}

impl<'a, O, T> Iterator for OpIterator<'a, O, T> {
	type Item = OperatorSet<'a, O, T>;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(first) = self.first {
			if matches!(first, ExpressionComponent::Paren(_)) && !self.did_first_paren
			{
				if let ExpressionComponent::Paren(first_paren) = first {
					self.did_first_paren = true;
					self.become_parent_of(&first_paren);
					self.next()
				} else {
					unreachable!()
				}
			} else {
				self.first = None;
				if let Some(Pair(op, component)) = self.pairs.get(0) {
					self.move_into_paren(true);
					Some((first, op, component))
				} else {
					self.become_parent()
				}
			}
		} else {
			match (&self.pairs.get(0), &self.pairs.get(1)) {
				(Some(Pair(_, lhs)), Some(Pair(op, rhs))) => {
					if !self.move_into_paren(false) {
						self.pairs = &self.pairs[1..];
					}
					Some((lhs, op, rhs))
				}
				(None, Some(_)) => unreachable!(),
				_ => self.become_parent(),
			}
		}
	}
}

impl<'a, O, T> Default for OpIterator<'a, O, T> {
	fn default() -> Self {
		Self {
			first: None,
			pairs: &[],
			parent: None,
			did_first_paren: true,
		}
	}
}
