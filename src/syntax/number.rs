use crate::{Parse, Result};
use nom::{character::complete::digit1, IResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Number(f64);

impl Parse for Number {
	fn parse(input: &str) -> IResult<&str, Self> {
		todo!()
	}
}
