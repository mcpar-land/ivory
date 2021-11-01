use std::fmt::Display;

use nom::{branch::alt, bytes::complete::tag, combinator::value};

use crate::Parse;

#[derive(Clone, Debug)]
pub enum Comparator {
	Gt,
	Lt,
	GtEq,
	LtEq,
	Eq,
}

impl Parse for Comparator {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			value(Self::GtEq, tag(">=")),
			value(Self::LtEq, tag("<=")),
			value(Self::Gt, tag(">")),
			value(Self::Lt, tag("<")),
			value(Self::Eq, tag("==")),
		))(input)
	}
}

impl Display for Comparator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Comparator::Gt => ">",
				Comparator::Lt => "<",
				Comparator::GtEq => ">=",
				Comparator::LtEq => "<=",
				Comparator::Eq => "==",
			}
		)
	}
}

#[derive(Clone, Debug)]
pub enum LogicOp {
	And,
	Or,
}

impl Parse for LogicOp {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((value(Self::And, tag("&&")), value(Self::Or, tag("||"))))(input)
	}
}

impl Display for LogicOp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				LogicOp::And => "&&",
				LogicOp::Or => "||",
			}
		)
	}
}
