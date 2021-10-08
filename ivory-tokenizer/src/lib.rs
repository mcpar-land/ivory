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
pub mod util;
pub mod values;
pub mod variable;

pub trait Parse: Sized + Clone + Debug + Display {
	fn parse(input: &str) -> IResult<&str, Self>;
}

/// Tokenize a string into a module.
pub fn tokenize(input: &str) -> Result<Module, String> {
	use nom::Finish;
	Module::parse(input)
		.finish()
		.map(|(_, m)| m)
		.map_err(|e| format!("{}", e))
}
