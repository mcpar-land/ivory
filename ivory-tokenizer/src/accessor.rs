use nom::{
	branch::alt,
	character::complete::{char, multispace0},
	combinator::map,
	multi::{many0, separated_list0},
	sequence::{delimited, pair, preceded},
};

use crate::{expression::Expression, variable::VariableName, Parse};

#[derive(Clone, Debug)]
pub struct Accessor(VariableName, Vec<AccessorComponent>);

impl Parse for Accessor {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let first = VariableName::parse;

		let afters = many0(preceded(multispace0, AccessorComponent::parse));

		map(pair(first, afters), |(first, afters)| {
			Accessor(first, afters)
		})(input)
	}
}

#[derive(Clone, Debug)]
pub enum AccessorComponent {
	Property(VariableName),
	Index(Expression),
	Call(Vec<Expression>),
}

impl Parse for AccessorComponent {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let property = map(
			preceded(pair(char('.'), multispace0), VariableName::parse),
			|res| AccessorComponent::Property(res),
		);
		let index = map(
			delimited(
				pair(char('['), multispace0),
				Expression::parse,
				pair(multispace0, char(']')),
			),
			|e| AccessorComponent::Index(e),
		);
		let call = map(
			delimited(
				pair(char('('), multispace0),
				separated_list0(
					delimited(multispace0, char(','), multispace0),
					Expression::parse,
				),
				pair(multispace0, char(')')),
			),
			|vals| AccessorComponent::Call(vals),
		);

		alt((property, index, call))(input)
	}
}

#[cfg(test)]
#[test]
fn parse_accessor() {
	let vs = &[
		"foo.bar.baz",
		"bix[234].quaaludes[3][5][7].ooo[8]",
		"foobar()",
		"fizzlord(123,\n456\n,biggler)",
		"math.square_root(5 * 5)",
	];
	for v in vs {
		match Accessor::parse(v) {
			Ok(v) => {
				println!("{:?}", v)
			}
			Err(err) => panic!("Error parsing \"{}\" -> {:?}", v, err),
		};
	}
}
