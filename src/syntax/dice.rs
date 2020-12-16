use crate::Parse;
use nom::{
	bytes::complete::tag, character::complete::digit1, combinator::map, IResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Dice {
	pub base: DiceBase,
	pub modifiers: Vec<DiceModifier>,
}

impl Parse for Dice {
	fn parse(input: &str) -> IResult<&str, Self> {
		use nom::multi::many0;
		let (input, base) = DiceBase::parse(input)?;
		let (input, modifiers) = many0(DiceModifier::parse)(input)?;
		Ok((input, Dice { base, modifiers }))
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiceModifier {
	pub mod_type: DiceModifierType,
	pub direction: DiceModifierDirection,
	pub value: Option<u32>,
}

impl Parse for DiceModifier {
	fn parse(input: &str) -> IResult<&str, Self> {
		use nom::{
			character::complete::digit0,
			character::complete::one_of,
			combinator::{opt, verify},
		};
		use DiceModifierDirection::*;
		use DiceModifierType::*;

		let (input, c) = one_of("kKdDeErRsSfF")(input)?;
		let (direction, mod_type) = match c {
			'k' => (Low, Keep),
			'K' => (High, Keep),
			'd' => (Low, Drop),
			'D' => (High, Drop),
			'e' => (Low, Explode),
			'E' => (High, Explode),
			'r' => (Low, Reroll),
			'R' => (High, Reroll),
			's' => (Low, Success),
			'S' => (High, Success),
			'f' => (Low, Failure),
			'F' => (High, Failure),
			_ => unreachable!(),
		};
		let (input, value) =
			opt(map(verify(digit0, |v: &str| v.len() > 0), |v: &str| {
				v.parse().unwrap()
			}))(input)?;

		Ok((
			input,
			DiceModifier {
				mod_type,
				direction,
				value,
			},
		))
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiceModifierDirection {
	Low,
	High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiceModifierType {
	Keep,
	Drop,
	Explode,
	Reroll,
	Success,
	Failure,
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

	#[test]
	fn parse_dice() {
		assert_eq!(
			Dice::parse("3d6").unwrap().1,
			Dice {
				base: DiceBase { amt: 3, sides: 6 },
				modifiers: vec![]
			}
		);
		assert_eq!(
			Dice::parse("1d20k3dR").unwrap().1,
			Dice {
				base: DiceBase { amt: 1, sides: 20 },
				modifiers: vec![
					DiceModifier {
						mod_type: DiceModifierType::Keep,
						direction: DiceModifierDirection::Low,
						value: Some(3)
					},
					DiceModifier {
						mod_type: DiceModifierType::Drop,
						direction: DiceModifierDirection::Low,
						value: None
					},
					DiceModifier {
						mod_type: DiceModifierType::Reroll,
						direction: DiceModifierDirection::High,
						value: None
					},
				]
			}
		);
	}
}
