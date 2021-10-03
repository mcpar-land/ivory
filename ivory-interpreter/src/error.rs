use nom::error::{ErrorKind, ParseError};

pub enum Error<I> {
	DuplicateKeysInObject,
	Nom(I, ErrorKind),
}

impl<I> ParseError<I> for Error<I> {
	fn from_error_kind(input: I, kind: ErrorKind) -> Self {
		Self::Nom(input, kind)
	}

	fn append(_: I, _: ErrorKind, other: Self) -> Self {
		other
	}
}
