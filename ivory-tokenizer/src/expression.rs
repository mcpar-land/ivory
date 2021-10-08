pub mod dice_ops;
pub mod math;

use std::fmt::Display;

use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{char, multispace0},
	combinator::{map, value},
	multi::{many0, separated_list0},
	sequence::{delimited, pair, preceded, separated_pair},
};

use crate::{
	accessor::Accessor,
	expression::math::ExprOpMathKind,
	values::{integer::IntegerValue, Value},
	Parse,
};

use self::{dice_ops::DiceOp, math::ExprOpMath};

#[derive(Clone, Debug)]
pub struct Expression {
	pub first: ExpressionComponent,
	pub pairs: Vec<ExpressionPair>,
}

impl Expression {
	pub fn zero() -> Self {
		Self {
			first: ExpressionComponent::Value(Value::Integer(IntegerValue(0))),
			pairs: vec![],
		}
	}

	pub fn iter(&self) -> ExpressionIterator<'_> {
		ExpressionIterator {
			first: Some(&self.first),
			pairs: self.pairs.as_slice(),
			parent: None,
		}
	}

	pub fn iter_accessors(&self) -> impl Iterator<Item = &Accessor> {
		self.iter().filter_map(|c| {
			if let ExpressionComponent::Accessor(acc) = c {
				Some(acc)
			} else {
				None
			}
		})
	}

	pub fn iter_operations(&self) -> OperationIterator<'_> {
		OperationIterator {
			first: Some(&self.first),
			pairs: self.pairs.as_slice(),
			parent: None,
			..Default::default()
		}
	}
}

impl Parse for Expression {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, first) = ExpressionComponent::parse(input)?;
		let (input, pairs) =
			many0(preceded(multispace0, ExpressionPair::parse))(input)?;

		Ok((input, Self { first, pairs }))
	}
}

impl Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}{}",
			self.first,
			self
				.pairs
				.iter()
				.fold(String::new(), |s, v| { format!("{} {}", s, v) })
		)
	}
}

#[derive(Clone, Debug)]
pub enum Op {
	Dice,
	Math(ExprOpMath),
	DiceOp(DiceOp),
}

impl Parse for Op {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			value(Op::Dice, tag("d")),
			map(ExprOpMath::parse, |m| Op::Math(m)),
			map(DiceOp::parse, |d| Op::DiceOp(d)),
		))(input)
	}
}

impl Display for Op {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Op::Dice => write!(f, "d"),
			Op::Math(op) => write!(f, "{}", op),
			Op::DiceOp(op) => write!(f, "{}", op),
		}
	}
}

#[derive(Clone, Debug)]
pub enum ExpressionComponent {
	Value(Value),
	Accessor(Accessor),
	Paren(Box<Expression>),
}

impl ExpressionComponent {
	pub fn is_accessor(&self) -> bool {
		match self {
			Self::Accessor(_) => true,
			_ => false,
		}
	}
}

impl Parse for ExpressionComponent {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			map(Value::parse, |r| Self::Value(r)),
			map(Accessor::parse, |r| Self::Accessor(r)),
			map(delimited(char('('), Expression::parse, char(')')), |r| {
				Self::Paren(Box::new(r))
			}),
		))(input)
	}
}

impl Display for ExpressionComponent {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ExpressionComponent::Value(v) => write!(f, "{}", v),
			ExpressionComponent::Accessor(a) => write!(f, "{}", a),
			ExpressionComponent::Paren(p) => write!(f, "({})", p),
		}
	}
}

#[derive(Clone, Debug)]
pub struct ExpressionPair {
	pub op: Op,
	pub component: ExpressionComponent,
}

impl Parse for ExpressionPair {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			separated_pair(Op::parse, multispace0, ExpressionComponent::parse),
			|(op, component)| ExpressionPair { op, component },
		)(input)
	}
}

impl Display for ExpressionPair {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {}", self.op, self.component)
	}
}

pub struct ExpressionIterator<'a> {
	first: Option<&'a ExpressionComponent>,
	pairs: &'a [ExpressionPair],
	parent: Option<Box<ExpressionIterator<'a>>>,
}

impl<'a> Iterator for ExpressionIterator<'a> {
	type Item = &'a ExpressionComponent;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(first) = self.first {
			self.first = None;
			Some(first)
		} else {
			match &self.pairs.get(0) {
				None => match self.parent.take() {
					Some(parent) => {
						*self = *parent;
						self.next()
					}
					None => None,
				},
				Some(ExpressionPair { op: _, component }) => match component {
					ExpressionComponent::Paren(paren) => {
						self.pairs = &self.pairs[1..];
						*self = ExpressionIterator {
							first: Some(&paren.first),
							pairs: paren.pairs.as_slice(),
							parent: Some(Box::new(std::mem::take(self))),
						};
						self.next()
					}
					component => {
						self.pairs = &self.pairs[1..];
						Some(component)
					}
				},
			}
		}
	}
}

impl<'a> Default for ExpressionIterator<'a> {
	fn default() -> Self {
		Self {
			first: None,
			pairs: &[],
			parent: None,
		}
	}
}

#[derive(Debug)]
pub struct OperationIterator<'a> {
	first: Option<&'a ExpressionComponent>,
	pairs: &'a [ExpressionPair],
	parent: Option<Box<OperationIterator<'a>>>,
	did_first_paren: bool,
}

impl<'a> OperationIterator<'a> {
	fn move_into_paren(&mut self, first: bool) -> bool {
		let possible_paren = if first {
			self.pairs.get(0)
		} else {
			self.pairs.get(1)
		};
		if let Some(ExpressionPair {
			component: ExpressionComponent::Paren(paren),
			..
		}) = possible_paren
		{
			println!("Moving into a paren...");
			self.pairs = &self.pairs[1..];
			*self = OperationIterator {
				first: Some(&paren.first),
				pairs: paren.pairs.as_slice(),
				parent: Some(Box::new(std::mem::take(self))),
				..Default::default()
			};
			true
		} else {
			false
		}
	}
}

impl<'a> Iterator for OperationIterator<'a> {
	type Item = (&'a ExpressionComponent, &'a Op, &'a ExpressionComponent);

	fn next(&mut self) -> Option<Self::Item> {
		println!(
			"RUNNING NEXT ON: {:?} {}",
			self.first,
			self
				.pairs
				.iter()
				.fold(String::new(), |s, v| { format!("{} {}", s, v) })
		);
		if let Some(first) = self.first {
			if matches!(first, ExpressionComponent::Paren(_)) && !self.did_first_paren
			{
				if let ExpressionComponent::Paren(first_paren) = first {
					self.did_first_paren = true;
					*self = OperationIterator {
						first: Some(&first_paren.first),
						pairs: first_paren.pairs.as_slice(),
						parent: Some(Box::new(std::mem::take(self))),
						..Default::default()
					};
					self.next()
				} else {
					unreachable!()
				}
			} else {
				self.first = None;
				if let Some(pair) = self.pairs.get(0) {
					println!("got ({} {} {})", first, pair.op, pair.component);
					self.move_into_paren(true);
					Some((first, &pair.op, &pair.component))
				} else {
					match self.parent.take() {
						Some(parent) => {
							*self = *parent;
							self.next()
						}
						None => None,
					}
				}
			}
		} else {
			match (&self.pairs.get(0), &self.pairs.get(1)) {
				// two consecutive pairs
				(
					Some(ExpressionPair {
						op: _,
						component: lhs,
					}),
					Some(ExpressionPair { op, component: rhs }),
				) => {
					println!("got ({} {} {})", lhs, op, rhs);
					if !self.move_into_paren(false) {
						self.pairs = &self.pairs[1..];
					}
					Some((lhs, op, rhs))
				}
				(None, Some(_)) => unreachable!(),
				// end of expression
				_ => match self.parent.take() {
					Some(parent) => {
						*self = *parent;
						self.next()
					}
					None => None,
				},
			}
		}
	}
}

impl<'a> Default for OperationIterator<'a> {
	fn default() -> Self {
		Self {
			first: None,
			pairs: &[],
			parent: None,
			did_first_paren: false,
		}
	}
}

#[cfg(test)]
#[test]
fn parse_expression() {
	use crate::util::test_multiple;

	test_multiple::<Expression>(&[
		"11 + 3",
		"11+3",
		"5 * 4 + 13 - 2",
		"5\n *4+ 13    -2 ",
		"(11 + 3) / (3 + 11)",
		"38 /^ 3",
		"18 + bogos[34] / binted[8 * 8]",
		"((((((((((((69))))))))))))",
	]);
}

#[test]
fn expression_iterator() {
	let expr =
		Expression::parse("5 * 4 + 13 - (6 * 6 * 6) / 4 + (3 + (2 + (1))) + 69")
			.unwrap()
			.1;
	let mut iter = expr.iter();
	fn assert(o: Option<&ExpressionComponent>, v: i64) {
		if let Some(&ExpressionComponent::Value(Value::Integer(IntegerValue(
			value,
		)))) = o
		{
			if value != v {
				panic!("Expected {}, got {}", v, value);
			} else {
				println!("got {}", value);
			}
		} else {
			panic!("Expected {}, got {:?}", v, o);
		}
	}
	assert(iter.next(), 5);
	assert(iter.next(), 4);
	assert(iter.next(), 13);
	assert(iter.next(), 6);
	assert(iter.next(), 6);
	assert(iter.next(), 6);
	assert(iter.next(), 4);
	assert(iter.next(), 3);
	assert(iter.next(), 2);
	assert(iter.next(), 1);
	assert(iter.next(), 69);
}

#[test]
fn operation_iterator() {
	use ExprOpMathKind::*;

	fn test_op_iterator(
		expr: &str,
		checks: &[(Option<i64>, ExprOpMathKind, Option<i64>)],
	) {
		let expr = Expression::parse(expr).unwrap().1;
		let mut iter = expr.iter_operations();
		for (lhs, op, rhs) in checks {
			match iter.next() {
				Some((
					&ExpressionComponent::Value(Value::Integer(IntegerValue(v_lhs))),
					&Op::Math(ExprOpMath { kind: v_op, .. }),
					&ExpressionComponent::Value(Value::Integer(IntegerValue(v_rhs))),
				)) => {
					if (*lhs, *op, *rhs) != (Some(v_lhs), v_op, Some(v_rhs)) {
						panic!(
							"Expected {:?} {:?} {:?}, got {} {:?} {}",
							lhs, op, rhs, v_lhs, v_op, v_rhs
						);
					}
				}
				Some((
					&ExpressionComponent::Value(Value::Integer(IntegerValue(v_lhs))),
					&Op::Math(ExprOpMath { kind: v_op, .. }),
					&ExpressionComponent::Paren(_),
				)) => {}
				Some((
					&ExpressionComponent::Paren(_),
					&Op::Math(ExprOpMath { kind: v_op, .. }),
					&ExpressionComponent::Value(Value::Integer(IntegerValue(v_rhs))),
				)) => {}
				_ => {
					panic!("Expected {:?} {:?} {:?}, got", lhs, op, rhs);
				}
			};
		}
		assert!(matches!(iter.next(), None));
	}

	test_op_iterator(
		"(3 * 3) * 4 + 13 - (6 * 6 * 6) / 4 + (3 + (2 + (1))) + 69",
		&[
			(Some(3), Mul, Some(3)),
			(None, Mul, Some(4)),
			(Some(4), Add, Some(13)),
			(Some(13), Sub, None),
			(Some(6), Mul, Some(6)),
			(Some(6), Mul, Some(6)),
			(None, Div, Some(4)),
			(Some(4), Add, None),
			(Some(3), Add, None),
			(Some(2), Add, None),
			(None, Add, Some(69)),
		],
	);
	test_op_iterator("((((((((((2 + 2))))))))))", &[(Some(2), Add, Some(2))]);
	test_op_iterator("99 / 99", &[(Some(99), Div, Some(99))]);
}
