use nom::{
	branch::alt,
	character::complete::{char, digit1},
	combinator::{map, not, recognize},
	sequence::pair,
};

use crate::Parse;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegerValue(pub i64);

impl Parse for IntegerValue {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, res) = map(
			alt((digit1, recognize(pair(char('-'), digit1)))),
			|input: &str| IntegerValue(input.parse::<i64>().unwrap()),
		)(input)?;

		let (input, _) = not(char('.'))(input)?;

		Ok((input, res))
	}
}

#[cfg(test)]
#[test]
fn parse_int_value() {
	assert_eq!(IntegerValue::parse("1234").unwrap().1, IntegerValue(1234));
	assert_eq!(IntegerValue::parse("-1234").unwrap().1, IntegerValue(-1234));
	assert!(IntegerValue::parse("12.3").is_err());
}
