use std::fmt::Display;

use nom::{
	branch::alt,
	character::complete::{char, digit1},
	combinator::{map, recognize},
	sequence::{pair, separated_pair},
};

use crate::Parse;

#[derive(Clone, Debug, PartialEq)]
pub struct DecimalValue(pub f64);

impl Parse for DecimalValue {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		fn decimal(input: &str) -> nom::IResult<&str, &str> {
			recognize(separated_pair(digit1, char('.'), digit1))(input)
		}

		map(
			alt((decimal, recognize(pair(char('-'), decimal)))),
			|input: &str| DecimalValue(input.parse::<f64>().unwrap()),
		)(input)
	}
}

impl Display for DecimalValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[cfg(test)]
#[test]
fn parse_decimal_value() {
	assert_eq!(DecimalValue::parse("123.4").unwrap().1, DecimalValue(123.4));
	assert_eq!(
		DecimalValue::parse("-126.2324").unwrap().1,
		DecimalValue(-126.2324)
	);
	assert!(DecimalValue::parse("69").is_err());
	assert!(DecimalValue::parse("-69").is_err());
	assert!(DecimalValue::parse(".69").is_err());
	assert!(DecimalValue::parse("69.").is_err());
	assert!(DecimalValue::parse("-.69").is_err());
	assert!(DecimalValue::parse("-69.").is_err());
}
