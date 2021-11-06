pub mod dice_ops;
pub mod logic;
pub mod math;

use colored::*;
use std::fmt::Display;

use ivory_expression::{Expression, ExpressionComponent, Pair};
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::char,
	combinator::{map, value},
	multi::many0,
	sequence::{delimited, pair, preceded, separated_pair},
};

use crate::{accessor::Accessor, util::ws0, Parse};

use self::{
	dice_ops::DiceOp,
	logic::{Comparator, LogicOp},
	math::ExprOpMath,
};

impl<O: Parse, T: Parse> Parse for ivory_expression::ExpressionComponent<O, T> {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			map(T::parse, |r| Self::Token(r)),
			map(
				delimited(
					pair(char('('), ws0),
					Expression::parse,
					pair(ws0, char(')')),
				),
				|r| Self::Paren(Box::new(r)),
			),
		))(input)
	}
}

impl<O: Parse, T: Parse> Parse for Pair<O, T> {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			separated_pair(O::parse, ws0, ExpressionComponent::<O, T>::parse),
			|(op, component)| Pair(op, component),
		)(input)
	}
}

impl<O: Parse, T: Parse> Parse for Expression<O, T> {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, first) = ExpressionComponent::parse(input)?;
		let (input, pairs) = many0(preceded(ws0, Pair::parse))(input)?;

		Ok((input, Self { first, pairs }))
	}
}

#[derive(Clone, Debug)]
pub enum Op {
	Dice,
	Math(ExprOpMath),
	DiceOp(DiceOp),
	Comparator(Comparator),
	Logic(LogicOp),
}

impl Parse for Op {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			value(Op::Dice, tag("d")),
			map(ExprOpMath::parse, |m| Op::Math(m)),
			map(DiceOp::parse, |d| Op::DiceOp(d)),
			map(Comparator::parse, |c| Self::Comparator(c)),
			map(LogicOp::parse, |l| Self::Logic(l)),
		))(input)
	}
}

impl Display for Op {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Op::Dice => write!(f, "{}", "d".red()),
			Op::Math(op) => write!(f, "{}", op),
			Op::DiceOp(op) => write!(f, "{}", op),
			Op::Comparator(op) => write!(f, "{}", op),
			Op::Logic(op) => write!(f, "{}", op),
		}
	}
}

#[derive(Clone, Debug)]
pub struct ExpressionToken(pub Accessor);

impl Parse for ExpressionToken {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(Accessor::parse, |r| Self(r))(input)
	}
}

impl Display for ExpressionToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[cfg(test)]
#[test]
fn parse_expression() {
	use crate::util::test_multiple;

	test_multiple::<Expression<Op, ExpressionToken>>(&[
		"11 + 3",
		"11+3",
		"5 * 4 + 13 - 2",
		"5\n *4+ 13    -2",
		"(11 + 3) / (3 + 11)",
		"38 /^ 3",
		"18 + bogos[34] / binted[8 * 8]",
		"((((((((((((69))))))))))))",
		"1d20 r<= 5",
	]);
}
