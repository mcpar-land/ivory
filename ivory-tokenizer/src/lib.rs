use std::fmt::{Debug, Display};

pub use module::Module;
pub use nom::error::ErrorKind;

use nom::IResult;

pub mod accessor;
pub mod commands;
pub mod expression;
pub mod istruct;
pub mod itype;
pub mod module;
pub mod ternary;
pub mod util;
pub mod values;
pub mod variable;

pub trait Parse: Sized + Clone + Debug + Display {
	fn parse(input: &str) -> IResult<&str, Self>;
}

/// Tokenize a string into a module.
pub fn tokenize<T: Parse>(input: &str) -> Result<T, TokenizerError> {
	use nom::Finish;
	T::parse(input)
		.finish()
		.map(|(_, m)| m)
		.map_err(|e| TokenizerError(format!("{}", e)))
}

#[derive(Clone, Debug)]
pub struct TokenizerError(String);

impl Display for TokenizerError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
