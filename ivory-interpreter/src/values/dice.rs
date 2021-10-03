use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{digit1, multispace0},
	combinator::{map, value},
	multi::many0,
	sequence::{delimited, pair, preceded, separated_pair, tuple},
	IResult,
};

use crate::Parse;

use crate::expression::Expression;

#[derive(Clone, Debug)]
pub struct Dice {
	pub count: DiceNumber,
	pub sides: DiceNumber,
	pub ops: Vec<DiceOp>,
}

impl Parse for Dice {
	fn parse(input: &str) -> IResult<&str, Self> {
		map(
			pair(
				separated_pair(DiceNumber::parse, tag("d"), DiceNumber::parse),
				many0(DiceOp::parse),
			),
			|((count, sides), ops)| Self { count, sides, ops },
		)(input)
	}
}

#[derive(Clone, Debug)]
pub enum DiceOp {
	Success(DiceCondition),            // s (condition)
	Failure(DiceCondition),            // f (condition)
	KeepLow(DiceNumber),               // kl (int)
	KeepHigh(DiceNumber),              // kh (int)
	DropLow(DiceNumber),               // dl (int)
	DropHigh(DiceNumber),              // dh (int)
	Explode(DiceCondition),            // ! (condition)
	CompoundingExplode(DiceCondition), // !! (condition)
	Reroll(DiceCondition),             // r (condition)
	RerollOnce(DiceCondition),         // ro (condition)
}

impl Parse for DiceOp {
	fn parse(input: &str) -> IResult<&str, Self> {
		alt((
			map(preceded(tag("s"), DiceCondition::parse), |c| {
				Self::Success(c)
			}),
			map(preceded(tag("f"), DiceCondition::parse), |c| {
				Self::Failure(c)
			}),
			map(preceded(tag("kl"), DiceNumber::parse), |c| Self::KeepLow(c)),
			map(preceded(tag("kh"), DiceNumber::parse), |c| {
				Self::KeepHigh(c)
			}),
			map(preceded(tag("dl"), DiceNumber::parse), |c| Self::DropLow(c)),
			map(preceded(tag("dh"), DiceNumber::parse), |c| {
				Self::DropHigh(c)
			}),
			map(preceded(tag("!"), DiceCondition::parse), |c| {
				Self::Explode(c)
			}),
			map(preceded(tag("!!"), DiceCondition::parse), |c| {
				Self::CompoundingExplode(c)
			}),
			map(preceded(tag("r"), DiceCondition::parse), |c| {
				Self::Reroll(c)
			}),
			map(preceded(tag("ro"), DiceCondition::parse), |c| {
				Self::RerollOnce(c)
			}),
		))(input)
	}
}

#[derive(Clone, Debug)]
pub struct DiceCondition {
	pub kind: DiceOpConditionKind,
	pub value: DiceNumber,
}

impl Parse for DiceCondition {
	fn parse(input: &str) -> IResult<&str, Self> {
		map(
			pair(DiceOpConditionKind::parse, DiceNumber::parse),
			|(kind, value)| Self { kind, value },
		)(input)
	}
}

#[derive(Clone, Debug)]
pub enum DiceOpConditionKind {
	Gt,
	Lt,
	Eq,
	GtEq,
	LtEq,
}

impl Parse for DiceOpConditionKind {
	fn parse(input: &str) -> IResult<&str, DiceOpConditionKind> {
		alt((
			value(Self::GtEq, tag(">=")),
			value(Self::LtEq, tag("<=")),
			value(Self::Eq, tag("=")),
			value(Self::Gt, tag(">")),
			value(Self::Gt, tag("<")),
		))(input)
	}
}

#[derive(Clone, Debug)]
pub enum DiceNumber {
	Literal(u32),
	Interpolate(Box<Expression>),
}

impl Parse for DiceNumber {
	fn parse(input: &str) -> IResult<&str, Self> {
		alt((
			map(digit1, |s: &str| Self::Literal(s.parse::<u32>().unwrap())),
			map(
				delimited(
					pair(tag("{"), multispace0),
					Expression::parse,
					pair(multispace0, tag("}")),
				),
				|expr| Self::Interpolate(Box::new(expr)),
			),
		))(input)
	}
}

#[cfg(test)]
#[test]
fn parse_dice_number() {
	use crate::util::{test_multiple, test_multiple_should_fail};

	test_multiple::<DiceNumber>(&["69", "0", "4", "{34}", "{bar+baz+(33/3)}"]);
	test_multiple_should_fail::<DiceNumber>(&["{}", "-34"]);
}

#[test]
fn test_dice_op_condition_kind() {
	use self::DiceOpConditionKind as D;
	assert!(D::parse(">=").is_ok());
	assert!(D::parse(">=").is_ok());
	assert!(D::parse("=").is_ok());
	assert!(D::parse(">").is_ok());
	assert!(D::parse("<").is_ok());
	assert!(D::parse("aklsjd").is_err());
}

#[test]
fn parse_dice() {
	use crate::util::test_multiple;
	test_multiple::<Dice>(&[
		"1d20",
		"4d100",
		"3d6",
		"{level}d20",
		"{level}d{hit_die}",
		"1d20r=1!>=19",
	]);
}
