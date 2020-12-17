use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{alpha1, alphanumeric1, multispace0},
	combinator::recognize,
	error::ParseError,
	multi::many0,
	sequence::{delimited, pair},
	IResult,
};

pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
	inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
	F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
	delimited(multispace0, inner, multispace0)
}

pub fn paren<'a, F: 'a, O, E: ParseError<&'a str>>(
	inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
	F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
	delimited(tag("("), ws(inner), tag(")"))
}

pub fn identifier(input: &str) -> IResult<&str, &str> {
	recognize(pair(
		alt((alpha1, tag("_"))),
		many0(alt((alphanumeric1, tag("_")))),
	))(input)
}
