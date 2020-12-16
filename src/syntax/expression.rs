use crate::{syntax, Parse};
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use syntax::{dice::Dice, function::FunctionCall, number::Number};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expression<R: Parse> {
	first: ExpressionItem<R>,
	sequence: Vec<(ExpressionOperator, ExpressionItem<R>)>,
	roll_type: std::marker::PhantomData<R>,
}

impl<R: Debug + Clone + Parse> Parse for Expression<R> {
	fn parse(input: &str) -> IResult<&str, Self> {
		todo!()
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
		todo!()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionOperator {
	pub round: Option<ExpressionOperatorRound>,
	pub op: ExpressionOperatorOp,
}

impl Parse for ExpressionOperator {
	fn parse(input: &str) -> IResult<&str, Self> {
		todo!()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
