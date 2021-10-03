use std::fmt::Debug;

use nom::IResult;

pub mod accessor;
pub mod error;
pub mod expression;
pub mod function;
pub mod istruct;
pub mod itype;
pub mod util;
pub mod values;
pub mod variable;

pub trait Parse: Sized + Clone + Debug {
	fn parse(input: &str) -> IResult<&str, Self>;
}
