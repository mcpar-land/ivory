use std::fmt::Display;

use nom::{
	branch::alt,
	bytes::complete::{tag, take_until, take_while},
	character::complete::{none_of, not_line_ending},
	combinator::{eof, map, success},
	sequence::delimited,
};

use crate::Parse;

#[derive(Clone, Debug)]
pub struct SingleComment(pub String);

impl Parse for SingleComment {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			delimited(tag("#"), not_line_ending, alt((tag("\n"), eof))),
			|v: &str| SingleComment(v.to_string()),
		)(input)
	}
}

impl Display for SingleComment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "# {}", self.0)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn comment() {
		assert_eq!(
			SingleComment::parse("# this is a single comment")
				.unwrap()
				.1
				 .0,
			" this is a single comment".to_string()
		);
		assert_eq!(
			SingleComment::parse("# this is a single comment\nthis is other stuff")
				.unwrap()
				.1
				 .0,
			" this is a single comment".to_string()
		);
	}
}
