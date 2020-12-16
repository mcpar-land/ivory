use nom::{error::Error as NomError, Err};
use quick_error::quick_error;

quick_error! {
	#[derive(Debug)]
	pub enum IvoryError {
		Syntax(err: Err<NomError<String>>) {
			source(err)
		}
		ParseNumber(err: core::num::ParseIntError){
			source(err)
			from()
		}
		NameNotFound(name: String) {}
		Serialization(err: Box<bincode::ErrorKind>) {
			source(err)
			from()
		}
	}
}

impl From<Err<NomError<&str>>> for IvoryError {
	fn from(_: Err<NomError<&str>>) -> Self {
		todo!()
	}
}

pub type Result<T> = std::result::Result<T, IvoryError>;
