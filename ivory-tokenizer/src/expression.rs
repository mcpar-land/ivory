pub mod math;

use nom::{
	branch::alt,
	character::complete::{char, multispace0},
	combinator::map,
	multi::{many0, separated_list0},
	sequence::{delimited, pair, preceded, separated_pair},
};

use crate::{
	accessor::Accessor,
	values::{dice::Dice, integer::IntegerValue, Value},
	Parse,
};

use self::math::ExprOpMath;

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
			in_op: false,
			parent: None,
		}
	}

	pub fn iter_dice(&self) -> impl Iterator<Item = &Dice> {
		self.iter().filter_map(|c| {
			if let ExpressionComponent::Value(Value::Dice(dice)) = c {
				Some(dice)
			} else {
				None
			}
		})
	}

	/// If there's a single accessor anywhere in here
	pub fn iter_accessors(&self) -> impl Iterator<Item = &Accessor> {
		self.iter().filter_map(|c| {
			if let ExpressionComponent::Accessor(acc) = c {
				Some(acc)
			} else {
				None
			}
		})
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

#[derive(Clone, Debug)]
pub struct ExpressionPair {
	pub op: ExprOpMath,
	pub component: ExpressionComponent,
}

impl Parse for ExpressionPair {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			separated_pair(
				ExprOpMath::parse,
				multispace0,
				ExpressionComponent::parse,
			),
			|(op, component)| ExpressionPair { op, component },
		)(input)
	}
}

pub enum ExpressionItem<'a> {
	First(&'a ExpressionComponent),
	Pair(&'a ExpressionComponent),
}

pub struct ExpressionIterator<'a> {
	first: Option<&'a ExpressionComponent>,
	pairs: &'a [ExpressionPair],
	in_op: bool,
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
							in_op: false,
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
			in_op: false,
			parent: None,
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
