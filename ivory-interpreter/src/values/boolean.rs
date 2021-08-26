use nom::{branch::alt, bytes::complete::tag, combinator::map};

use crate::Parse;

#[derive(Clone, Debug)]
pub struct BooleanValue(pub bool);

impl Parse for BooleanValue {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			map(tag("true"), |_| BooleanValue(true)),
			map(tag("false"), |_| BooleanValue(false)),
		))(input)
	}
}
