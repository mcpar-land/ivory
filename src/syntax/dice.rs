use crate::Parse;
use crate::Result;
use nom::{
	bytes::complete::tag, character::complete::digit1, combinator::map, IResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dice {
	pub base: DiceBase,
	pub modifiers: Vec<(DiceModifier, u32)>,
}

impl Parse for Dice {
	fn parse(input: &str) -> IResult<&str, Self> {
		todo!()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiceModifier {
	KeepHigher,
	KeepLower,
	DropHigher,
	DropLower,
	Explode,
}

impl Parse for DiceModifier {
	fn parse(input: &str) -> IResult<&str, Self> {
		todo!()
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiceBase {
	pub amt: u32,
	pub sides: u32,
}

impl Parse for DiceBase {
	fn parse(input: &str) -> IResult<&str, Self> {
		let (input, amt) = map(digit1, |s: &str| s.parse().unwrap())(input)?;
		let (input, _) = tag("d")(input)?;
		let (input, sides) = map(digit1, |s: &str| s.parse().unwrap())(input)?;

		Ok((input, DiceBase { amt, sides }))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn parse_dice_base() {
		assert_eq!(
			DiceBase::parse("1d20").unwrap(),
			("", DiceBase { amt: 1, sides: 20 })
		);
		assert_eq!(
			DiceBase::parse("10d6").unwrap(),
			("", DiceBase { amt: 10, sides: 6 })
		);
	}
}
