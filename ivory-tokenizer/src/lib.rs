use std::fmt::{Debug, Display};

pub use module::Module;
pub use nom::error::ErrorKind;

use nom::IResult;

pub mod accessor;
pub mod commands;
pub mod comment;
pub mod expression;
pub mod istruct;
pub mod itype;
pub mod module;
pub mod util;
pub mod values;
pub mod variable;

pub trait Parse: Sized + Clone + Debug + Display {
	fn parse(input: &str) -> IResult<&str, Self>;
}

/// Tokenize a string into a module.
pub fn tokenize<T: Parse>(input: &str) -> Result<T, TokenizerError> {
	use nom::Finish;
	let (remainder, res) = T::parse(input)
		.finish()
		.map_err(|e| TokenizerError(format!("{}", e)))?;

	if remainder.len() > 0 {
		Err(TokenizerError(format!(
			"Incomplete input! Could not parse: \"{}\"",
			remainder
		)))
	} else {
		Ok(res)
	}
}

#[derive(Clone, Debug)]
pub struct TokenizerError(pub String);

impl Display for TokenizerError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
