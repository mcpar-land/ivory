use super::dice::Roll;
use crate::{syntax, Parse};
use lazy_static::lazy_static;
use nom::{
	bytes::complete::tag,
	character::complete::multispace0,
	sequence::{delimited, pair, separated_pair},
	IResult,
};
use prec::{Assoc, Climber, Rule};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};
use syntax::{dice::Dice, function::FunctionCall, number::Number};

lazy_static! {
	static ref CLIMBER: Climber<ExpressionOperator, ExpressionItem<Dice>, f64> =
		Climber::new(vec![], climber_handler);
}

fn climber_handler(
	lhs: ExpressionItem<Dice>,
	op: ExpressionOperator,
	rhs: ExpressionItem<Dice>,
) -> ExpressionItem<Dice> {
	use ExpressionOperatorOp::*;
	use ExpressionOperatorRound::*;
	match op.op {
		Add => {}
		Sub => {}
		Mul => {}
		Div => {}
		Exp => {}
	}
	if let Some(round) = op.round {
		match round {
			Round => {}
			Ceil => {}
			Floor => {}
		}
	}
	todo!();
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Expression<R> {
	pub first: ExpressionItem<R>,
	pub sequence: Vec<(ExpressionOperator, ExpressionItem<R>)>,
	roll_type: PhantomData<R>,
}

impl<R: Parse> Expression<R> {
	pub fn new(
		first: ExpressionItem<R>,
		sequence: Vec<(ExpressionOperator, ExpressionItem<R>)>,
	) -> Self {
		Expression {
			first,
			sequence,
			roll_type: PhantomData,
		}
	}
}

impl<R: Roll> Expression<R> {
	pub fn eval(&self) -> Expression<R::Result> {
		todo!();
	}
}

impl<R: Debug + Clone + Parse> Parse for Expression<R> {
	fn parse(input: &str) -> IResult<&str, Self> {
		use nom::multi::many0;
		let (input, first) = ExpressionItem::parse(input)?;
		let (input, _) = multispace0(input)?;
		let (input, sequence) = many0(delimited(
			multispace0,
			separated_pair(
				ExpressionOperator::parse,
				multispace0,
				ExpressionItem::parse,
			),
			multispace0,
		))(input)?;

		Ok((input, Expression::new(first, sequence)))
	}
}

fn expression_paren<R: Parse>(input: &str) -> IResult<&str, Expression<R>> {
	delimited(
		pair(tag("("), multispace0),
		Expression::<R>::parse,
		pair(multispace0, tag(")")),
	)(input)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExpressionItem<R> {
	Number(Number),
	Dice(Dice),
	Parens(Box<Expression<R>>),
	FunctionCall(FunctionCall),
}

impl<R: Debug + Clone + Parse> Parse for ExpressionItem<R> {
	fn parse(input: &str) -> IResult<&str, Self> {
		use nom::{branch::alt, combinator::map};

		alt((
			map(Dice::parse, |v| ExpressionItem::Dice(v)),
			map(Number::parse, |v| ExpressionItem::Number(v)),
			map(FunctionCall::parse, |v| ExpressionItem::FunctionCall(v)),
			map(expression_paren, |v: Expression<R>| {
				ExpressionItem::Parens(Box::new(v))
			}),
		))(input)
	}
}

impl Into<f64> for ExpressionItem<Dice> {
	fn into(self) -> f64 {
		match self {
			Self::Number(Number(val)) => val,
			Self::Dice(dice) => todo!(),
			Self::Parens(parens) => todo!(),
			Self::FunctionCall(call) => todo!(),
		}
	}
}

#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExpressionOperator {
	pub round: Option<ExpressionOperatorRound>,
	pub op: ExpressionOperatorOp,
}

impl ExpressionOperator {
	pub fn new(
		round: Option<ExpressionOperatorRound>,
		op: ExpressionOperatorOp,
	) -> Self {
		Self { round, op }
	}

	pub fn rule(
		op: ExpressionOperatorOp,
		assoc: Assoc,
	) -> Rule<ExpressionOperator> {
		use ExpressionOperatorRound::*;
		Rule::new(Self::new(None, op), assoc)
			| Rule::new(Self::new(Some(Round), op), assoc)
			| Rule::new(Self::new(Some(Ceil), op), assoc)
			| Rule::new(Self::new(Some(Floor), op), assoc)
	}
}

impl Parse for ExpressionOperator {
	fn parse(input: &str) -> IResult<&str, Self> {
		use nom::{branch::alt, combinator::map};
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

#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq)]
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
	use syntax::dice::DiceBase;

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

	#[test]
	fn parse_expression() {
		use ExpressionOperatorOp::*;
		use ExpressionOperatorRound::*;
		assert_eq!(
			Expression::<()>::parse("108.3 * 77").unwrap().1,
			Expression::new(
				ExpressionItem::Number(Number(108.3)),
				vec![(
					ExpressionOperator::new(None, Mul),
					ExpressionItem::Number(Number(77.0))
				)]
			)
		);
		let raw = "2d10\n_/ 5 * (-3 - pagman) + (2 / function(arg1, 2 + arg2))";
		println!("{:#?}", Expression::<Dice>::parse(raw).unwrap().1);
		assert_eq!(
			Expression::<Dice>::parse(raw).unwrap().1,
			Expression::new(
				ExpressionItem::Dice(Dice {
					base: DiceBase { amt: 2, sides: 10 },
					modifiers: vec![]
				}),
				vec![
					(
						ExpressionOperator::new(Some(Floor), Div),
						ExpressionItem::Number(Number(5.0))
					),
					(
						ExpressionOperator::new(None, Mul),
						ExpressionItem::Parens(Box::new(Expression::new(
							ExpressionItem::Number(Number(-3.0)),
							vec![(
								ExpressionOperator::new(None, Sub),
								ExpressionItem::FunctionCall(FunctionCall::new(
									"pagman",
									vec![]
								))
							)]
						)))
					),
					(
						ExpressionOperator::new(None, Add),
						ExpressionItem::Parens(Box::new(Expression::new(
							ExpressionItem::Number(Number(2.0)),
							vec![(
								ExpressionOperator::new(None, Div),
								ExpressionItem::FunctionCall(FunctionCall::new(
									"function",
									vec![
										Expression::new(
											ExpressionItem::FunctionCall(FunctionCall::new(
												"arg1",
												vec![]
											)),
											vec![]
										),
										Expression::new(
											ExpressionItem::Number(Number(2.0)),
											vec![(
												ExpressionOperator::new(None, Add),
												ExpressionItem::FunctionCall(FunctionCall::new(
													"arg2",
													vec![]
												))
											)]
										)
									]
								))
							)]
						)))
					)
				]
			)
		);
	}
}
