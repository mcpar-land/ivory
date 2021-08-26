use nom::{
	branch::alt,
	bytes::complete::tag,
	error::{Error, ErrorKind},
	IResult,
};

use crate::Parse;

use super::Expression;

#[derive(Clone, Debug)]
pub enum DiceOp {
	Success(DiceCondition),
	Failure(DiceCondition),
	KeepLow(Box<Expression>),
	KeepHigh(Box<Expression>),
	DropLow(Box<Expression>),
	DropHigh(Box<Expression>),
	Explode(DiceCondition),
	CompoundingExplode(DiceCondition),
	Reroll(DiceCondition),
	RerollOnce(DiceCondition),
}

impl Parse for DiceOp {
	fn parse(input: &str) -> IResult<&str, Self> {
		todo!()
	}
}

#[derive(Clone, Debug)]
pub struct DiceCondition {
	pub kind: DiceOpConditionKind,
	pub value: Box<Expression>,
}

impl Parse for DiceCondition {
	fn parse(input: &str) -> IResult<&str, Self> {
		todo!()
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
		let (input, grab) =
			alt((tag(">="), tag("<="), tag("="), tag(">"), tag("<")))(input)?;
		let res = match grab {
			">=" => Self::GtEq,
			"<=" => Self::LtEq,
			"=" => Self::Eq,
			">" => Self::Gt,
			"<" => Self::Lt,
			_ => unreachable!(),
		};
		Ok((input, res))
	}
}

#[cfg(test)]
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
