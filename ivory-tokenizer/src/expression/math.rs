use colored::*;
use ivory_expression::Expression;
use std::fmt::Display;

use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::one_of,
	combinator::{map, opt},
	sequence::{delimited, pair},
};

use crate::{util::ws0, Parse};

use super::{ExpressionToken, Op};

#[derive(Clone, Debug)]
pub enum ExprOpMath {
	Binary {
		kind: ExprOpMathKind,
		round: Option<ExprOpMathRound>,
	},
	Ternary(Box<Expression<Op, ExpressionToken>>),
}

impl Parse for ExprOpMath {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			map(
				pair(ExprOpMathKind::parse, opt(ExprOpMathRound::parse)),
				|(kind, round)| Self::Binary { kind, round },
			),
			map(
				delimited(pair(tag("?"), ws0), Expression::parse, pair(ws0, tag(":"))),
				|expr| Self::Ternary(Box::new(expr)),
			),
		))(input)
	}
}

#[derive(Clone, Debug)]
pub enum ExprOpMathKind {
	Add,
	Sub,
	Mul,
	Div,
}

impl Parse for ExprOpMathKind {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, val) = one_of("+-*/")(input)?;
		Ok((
			input,
			match val {
				'+' => Self::Add,
				'-' => Self::Sub,
				'*' => Self::Mul,
				'/' => Self::Div,
				_ => unreachable!(),
			},
		))
	}
}

impl Display for ExprOpMathKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Add => "+",
				Self::Sub => "-",
				Self::Mul => "*",
				Self::Div => "/",
			}
			.red()
		)
	}
}

#[derive(Clone, Debug)]
pub enum ExprOpMathRound {
	Up,
	Down,
	Round,
}

impl Parse for ExprOpMathRound {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, val) = one_of("^_~")(input)?;
		Ok((
			input,
			match val {
				'^' => Self::Up,
				'_' => Self::Down,
				'~' => Self::Round,
				_ => unreachable!(),
			},
		))
	}
}

impl Display for ExprOpMathRound {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Up => "^",
				Self::Down => "_",
				Self::Round => "~",
			}
			.red()
		)
	}
}

impl Display for ExprOpMath {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ExprOpMath::Binary { kind, round } => match round {
				Some(round) => write!(f, "{}{}", kind, round),
				None => write!(f, "{}", kind),
			},
			ExprOpMath::Ternary(expr) => write!(f, "? {} :", expr),
		}
	}
}
