use std::fmt::Debug;

use nom::IResult;

pub mod accessor;
pub mod expression;
pub mod function;
pub mod istruct;
pub mod itype;
pub mod util;
pub mod values;
pub mod variable;

trait Parse: Sized + Clone + Debug {
	fn parse(input: &str) -> IResult<&str, Self>;
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
