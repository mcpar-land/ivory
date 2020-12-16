use crate::{syntax, Parse};
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use syntax::{dice::Dice, function::FunctionCall, number::Number};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expression<R: Parse> {
	pub first: ExpressionItem<R>,
	pub sequence: Vec<(ExpressionOperator, ExpressionItem<R>)>,
	roll_type: std::marker::PhantomData<R>,
}

impl<R: Debug + Clone + Parse> Parse for Expression<R> {
	fn parse(input: &str) -> IResult<&str, Self> {
		use nom::{multi::many0, sequence::pair};
		let (input, first) = ExpressionItem::parse(input)?;
		let (input, sequence) =
			many0(pair(ExpressionOperator::parse, ExpressionItem::parse))(input)?;

		Ok((
			input,
			Expression {
				first,
				sequence,
				roll_type: std::marker::PhantomData,
			},
		))
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpressionItem<R: Parse> {
	Number(Number),
	Dice(Dice),
	Parens(Box<Expression<R>>),
	FunctionCall(FunctionCall),
}

impl<R: Debug + Clone + Parse> Parse for ExpressionItem<R> {
	fn parse(input: &str) -> IResult<&str, Self> {
		use nom::{branch::alt, combinator::map};

		alt((
			map(Number::parse, |v| ExpressionItem::Number(v)),
			map(Dice::parse, |v| ExpressionItem::Dice(v)),
			map(Expression::parse, |v| ExpressionItem::Parens(Box::new(v))),
			map(FunctionCall::parse, |v| ExpressionItem::FunctionCall(v)),
		))(input)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExpressionOperator {
	pub round: Option<ExpressionOperatorRound>,
	pub op: ExpressionOperatorOp,
}

impl Parse for ExpressionOperator {
	fn parse(input: &str) -> IResult<&str, Self> {
		use nom::{branch::alt, combinator::map, sequence::pair};
		let (input, (round, op)) = alt((
			map(
				pair(ExpressionOperatorRound::parse, ExpressionOperatorOp::parse),
				|(round, op)| (Some(round), op),
			),
			map(ExpressionOperatorOp::parse, |op| (None, op)),
		))(input)?;
		Ok((input, ExpressionOperator { round, op }))
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExpressionOperatorRound {
	Round,
	Ceil,
	Floor,
}

impl ExpressionOperatorRound {
	pub fn parse(input: &str) -> IResult<&str, Self> {
		use nom::character::complete::one_of;
		let (input, t) = one_of("~`_")(input)?;
		Ok((
			input,
			match t {
				'~' => ExpressionOperatorRound::Round,
				'`' => ExpressionOperatorRound::Ceil,
				'_' => ExpressionOperatorRound::Floor,
				_ => unreachable!(),
			},
		))
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExpressionOperatorOp {
	Add,
	Sub,
	Mul,
	Div,
	Exp,
}

impl ExpressionOperatorOp {
	pub fn parse(input: &str) -> IResult<&str, Self> {
		use nom::character::complete::one_of;
		let (input, t) = one_of("+-*/^")(input)?;
		Ok((
			input,
			match t {
				'+' => ExpressionOperatorOp::Add,
				'-' => ExpressionOperatorOp::Sub,
				'*' => ExpressionOperatorOp::Mul,
				'/' => ExpressionOperatorOp::Div,
				'^' => ExpressionOperatorOp::Exp,
				_ => unreachable!(),
			},
		))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn parse_operators() {
		assert_eq!(
			ExpressionOperator::parse("+").unwrap().1,
			ExpressionOperator {
				round: None,
				op: ExpressionOperatorOp::Add
			}
		);
		assert_eq!(
			ExpressionOperator::parse("_/").unwrap().1,
			ExpressionOperator {
				round: Some(ExpressionOperatorRound::Floor),
				op: ExpressionOperatorOp::Div
			}
		);
	}
}
