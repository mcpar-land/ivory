use std::fmt::Display;

use nom::{
	branch::alt,
	bytes::complete::tag,
	combinator::{map, value},
	sequence::pair,
};

use crate::Parse;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
pub struct DiceOp {
	pub op: DiceOpCmp,
	pub cmp: DiceCmp,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
pub enum DiceOpCmp {
	Keep,
	Reroll,
	RerollContinuous,
	Explode,
	ExplodeContinuous,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
pub enum DiceCmp {
	Gt,
	Lt,
	Eq,
	GtEq,
	LtEq,
}

impl Parse for DiceOp {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(pair(DiceOpCmp::parse, DiceCmp::parse), |(op, cmp)| DiceOp {
			op,
			cmp,
		})(input)
	}
}

impl Display for DiceOp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}", self.op, self.cmp)
	}
}

impl Parse for DiceOpCmp {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			value(Self::Keep, tag("k")),
			value(Self::Reroll, tag("r")),
			value(Self::RerollContinuous, tag("rr")),
			value(Self::Explode, tag("!")),
			value(Self::ExplodeContinuous, tag("!!")),
		))(input)
	}
}

impl Parse for DiceCmp {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			value(DiceCmp::GtEq, tag(">=")),
			value(DiceCmp::LtEq, tag("<=")),
			value(DiceCmp::Eq, tag("==")),
			value(DiceCmp::Gt, tag(">")),
			value(DiceCmp::Lt, tag("<")),
		))(input)
	}
}

impl Display for DiceOpCmp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				DiceOpCmp::Keep => "k",
				DiceOpCmp::Reroll => "r",
				DiceOpCmp::RerollContinuous => "rr",
				DiceOpCmp::Explode => "!",
				DiceOpCmp::ExplodeContinuous => "!!",
			}
		)
	}
}

impl Display for DiceCmp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				DiceCmp::Gt => ">",
				DiceCmp::Lt => "<",
				DiceCmp::Eq => "==",
				DiceCmp::GtEq => ">=",
				DiceCmp::LtEq => "<=",
			}
		)
	}
}
