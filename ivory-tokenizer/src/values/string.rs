use std::fmt::Display;

use nom::{
	branch::alt,
	bytes::complete::is_not,
	character::complete::char,
	combinator::{map, value, verify},
	multi::fold_many0,
	sequence::{delimited, preceded},
};

use crate::{util::ws1, Parse};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StringValue(pub String);

impl Parse for StringValue {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		fn parse_escaped_char(input: &str) -> nom::IResult<&str, char> {
			preceded(
				char('\\'),
				alt((
					value('\n', char('n')),
					value('\t', char('t')),
					value('\\', char('\\')),
					value('/', char('/')),
					value('"', char('"')),
				)),
			)(input)
		}

		fn parse_escaped_whitespace(input: &str) -> nom::IResult<&str, &str> {
			preceded(char('\\'), ws1)(input)
		}

		fn parse_literal(input: &str) -> nom::IResult<&str, &str> {
			let not_quote_slash = is_not("\"\\");
			verify(not_quote_slash, |s: &str| !s.is_empty())(input)
		}

		#[derive(Clone, Copy, PartialEq, Eq)]
		enum StringFragment<'a> {
			Literal(&'a str),
			EscapedChar(char),
			EscapedWS,
		}

		fn parse_fragment<'a>(
			input: &'a str,
		) -> nom::IResult<&str, StringFragment<'a>> {
			alt((
				map(parse_literal, StringFragment::Literal),
				map(parse_escaped_char, StringFragment::EscapedChar),
				value(StringFragment::EscapedWS, parse_escaped_whitespace),
			))(input)
		}

		let build_string =
			fold_many0(parse_fragment, String::new, |mut string, fragment| {
				match fragment {
					StringFragment::Literal(s) => string.push_str(s),
					StringFragment::EscapedChar(c) => string.push(c),
					StringFragment::EscapedWS => {}
				}
				string
			});

		map(delimited(char('"'), build_string, char('"')), |s| {
			StringValue(s)
		})(input)
	}
}

impl Display for StringValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// TODO escapify string
		write!(f, "\"{}\"", self.0)
	}
}

#[cfg(test)]
#[test]
fn parse_string_value() {
	let strongs = [
		(r#""I am a cool string""#, "I am a cool string"),
		(r#""I am also a cool string.""#, "I am also a cool string."),
		(
			r#""Look at this! -> \" <- wow""#,
			"Look at this! -> \" <- wow",
		),
		(r#""this has a \\ backslash""#, "this has a \\ backslash"),
		(r#""this has a \n newline""#, "this has a \n newline"),
	];

	// let strongs_err = [
	// 	(r#""I am a cool string""#, "I am a cool string"),
	// 	(r#""I am also a cool string.""#, "I am also a cool string."),
	// ];

	for (s, r) in strongs.iter() {
		println!("{}    ->    {}", s, r);
		assert_eq!(
			StringValue::parse(*s).unwrap().1,
			StringValue(r.to_string())
		);
	}
}
