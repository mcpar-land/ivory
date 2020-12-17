use crate::Parse;
use nom::IResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Number(pub f64);

impl Parse for Number {
	fn parse(input: &str) -> IResult<&str, Self> {
		use nom::{combinator::map, number::complete::recognize_float};
		// let (input, _) = not(pair(digit1, tag("..")))(input)?;
		map(recognize_float, |s: &str| Number(s.parse().unwrap()))(input)
	}
}
