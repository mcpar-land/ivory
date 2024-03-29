use std::fmt::Display;

use nom::{branch::alt, bytes::complete::tag, combinator::map};

use crate::Parse;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BooleanValue(pub bool);

impl Parse for BooleanValue {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			map(tag("true"), |_| BooleanValue(true)),
			map(tag("false"), |_| BooleanValue(false)),
		))(input)
	}
}

impl Display for BooleanValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", if self.0 { "true" } else { "false" })
	}
}

#[cfg(test)]
#[test]
fn parse_boolean_value() {
	assert!(BooleanValue::parse("true").is_ok());
	assert!(BooleanValue::parse("false").is_ok());
	assert!(BooleanValue::parse("gronk").is_err());
}
