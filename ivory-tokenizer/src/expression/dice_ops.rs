use colored::*;
use std::fmt::Display;

use nom::{
	branch::alt,
	bytes::complete::tag,
	combinator::{map, value},
	sequence::pair,
};

use crate::Parse;

use super::logic::Comparator;

#[derive(Clone, Debug)]
pub struct DiceOp {
	pub op: DiceOpCmp,
	pub cmp: Comparator,
}

#[derive(Clone, Debug)]
pub enum DiceOpCmp {
	Keep,
	Reroll,
	RerollContinuous,
	Explode,
	ExplodeContinuous,
}

impl Parse for DiceOp {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(pair(DiceOpCmp::parse, Comparator::parse), |(op, cmp)| {
			DiceOp { op, cmp }
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
			.red()
		)
	}
}
