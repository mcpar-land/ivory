use crate::Result;
use nom::IResult;
pub trait Parse: Sized + std::fmt::Debug + Clone {
	fn parse(input: &str) -> Result<(&str, Self)>;
}
