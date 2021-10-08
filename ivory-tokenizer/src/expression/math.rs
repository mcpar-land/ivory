use std::fmt::Display;

use nom::{character::complete::one_of, combinator::opt};

use crate::Parse;

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub struct ExprOpMath {
	pub kind: ExprOpMathKind,
	pub round: Option<ExprOpMathRound>,
}

impl Parse for ExprOpMath {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, kind) = ExprOpMathKind::parse(input)?;
		let (input, round) = opt(ExprOpMathRound::parse)(input)?;
		Ok((input, Self { kind, round }))
	}
}

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
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

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
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

impl Display for ExprOpMath {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}{}",
			match self.kind {
				ExprOpMathKind::Add => "+",
				ExprOpMathKind::Sub => "-",
				ExprOpMathKind::Mul => "*",
				ExprOpMathKind::Div => "/",
			},
			match &self.round {
				Some(round) => match round {
					ExprOpMathRound::Up => "^",
					ExprOpMathRound::Down => "_",
					ExprOpMathRound::Round => "~",
				},
				None => "",
			}
		)
	}
}