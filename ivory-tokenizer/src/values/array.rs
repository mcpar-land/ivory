use std::fmt::Display;

use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{char, multispace0},
	combinator::{map, value},
	multi::{many0, separated_list0},
	sequence::{delimited, pair, tuple},
};

use crate::{expression::Expression, util::comma_separated_display, Parse};

#[derive(Clone, Debug)]
pub struct ArrayValue(pub Vec<Expression>);

impl Parse for ArrayValue {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		fn empty_array(input: &str) -> nom::IResult<&str, ()> {
			value((), tuple((char('['), multispace0, char(']'))))(input)
		}
		alt((
			value(ArrayValue(Vec::new()), empty_array),
			map(
				delimited(
					pair(char('['), multispace0),
					separated_list0(
						tuple((multispace0, tag(","), multispace0)),
						Expression::parse,
					),
					pair(multispace0, char(']')),
				),
				|v| ArrayValue(v),
			),
		))(input)
	}
}

impl Display for ArrayValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}]", comma_separated_display(&self.0))
	}
}

#[cfg(test)]
#[test]
fn parse_array() {
	let a = ArrayValue::parse("[1, 2, true, \"spaghetti\", 4.4]")
		.unwrap()
		.1;
	println!("{:?}", a);

	let a = ArrayValue::parse("[]").unwrap().1;
	println!("{:?}", a);
}

#[test]
fn parse_nesting_array() {
	let a = ArrayValue::parse("[[1,2,3],[4,5,6],[7, 8, 9]]").unwrap().1;
	println!("{:?}", a);
	let a = ArrayValue::parse("[[[[]]]]").unwrap().1;
	println!("{:?}", a);
}
