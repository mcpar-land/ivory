use nom::{
	bytes::complete::take_while1, character::is_digit, combinator::map,
	number::complete::be_i64,
};

use crate::Parse;

#[derive(Clone, Debug)]
pub struct IntegerValue(pub i64);

impl Parse for IntegerValue {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(take_while1(is_digit), |input: &str| {
			IntegerValue(input.parse())
		})(input)
	}
}
