pub mod dice_ops;
pub mod math;

use colored::*;
use std::fmt::Display;

use ivory_expression::{
	ternary::Ternary, Expression, ExpressionComponent, Pair, TernaryExpression,
	TernaryExpressionComponent, TernaryPair,
};
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{char, multispace0},
	combinator::{map, opt, value},
	multi::{many0, separated_list0},
	sequence::{delimited, pair, preceded, separated_pair, tuple},
};

use crate::{
	accessor::Accessor,
	expression::math::ExprOpMathKind,
	values::{integer::IntegerValue, Value},
	Parse,
};

use self::{dice_ops::DiceOp, math::ExprOpMath};

impl Parse for Ternary<Op, ExpressionToken> {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, first) = TernaryExpression::parse(input)?;

		let (input, conditions) = opt(map(
			preceded(
				pair(multispace0, tag("?")),
				separated_pair(
					Self::parse,
					tuple((multispace0, tag(":"), multispace0)),
					Self::parse,
				),
			),
			|both| Box::new(both),
		))(input)?;

		Ok((
			input,
			Self {
				condition: first,
				options: conditions,
			},
		))
	}
}

impl Parse
	for ivory_expression::TernaryExpressionComponent<Op, ExpressionToken>
{
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			map(ExpressionToken::parse, |r| Self::Token(r)),
			map(
				delimited(
					pair(char('('), multispace0),
					Ternary::parse,
					pair(multispace0, char(')')),
				),
				|r| Self::Paren(Box::new(r)),
			),
		))(input)
	}
}

impl Parse for TernaryPair<Op, ExpressionToken> {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			separated_pair(Op::parse, multispace0, TernaryExpressionComponent::parse),
			|(op, component)| TernaryPair(op, component),
		)(input)
	}
}

impl Parse for TernaryExpression<Op, ExpressionToken> {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, first) = TernaryExpressionComponent::parse(input)?;
		let (input, pairs) =
			many0(preceded(multispace0, TernaryPair::parse))(input)?;

		Ok((input, Self { first, pairs }))
	}
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
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
			Op::Dice => write!(f, "{}", "d".red()),
			Op::Math(op) => write!(f, "{}", op),
			Op::DiceOp(op) => write!(f, "{}", op),
		}
	}
}

#[derive(Clone, Debug)]
pub enum ExpressionToken {
	Value(Value),
	Accessor(Accessor),
}

impl ExpressionToken {
	pub fn is_accessor(&self) -> bool {
		match self {
			Self::Accessor(_) => true,
			_ => false,
		}
	}
}

impl Parse for ExpressionToken {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			map(Value::parse, |r| Self::Value(r)),
			map(Accessor::parse, |r| Self::Accessor(r)),
		))(input)
	}
}

impl Display for ExpressionToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ExpressionToken::Value(v) => write!(f, "{}", v),
			ExpressionToken::Accessor(a) => write!(f, "{}", a),
		}
	}
}

#[cfg(test)]
#[test]
fn parse_expression() {
	use crate::util::test_multiple;

	test_multiple::<TernaryExpression<Op, ExpressionToken>>(&[
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
