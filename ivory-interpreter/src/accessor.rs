use nom::{
	branch::alt,
	character::complete::{char, multispace0},
	combinator::map,
	multi::separated_list0,
	sequence::{delimited, pair, preceded},
};

use crate::{expression::Expression, variable::VariableName, Parse};

#[derive(Clone, Debug)]
pub struct Accessor(VariableName, Vec<AccessorComponent>);

impl Parse for Accessor {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let first = VariableName::parse;

		let afters = preceded(
			multispace0,
			separated_list0(multispace0, AccessorComponent::parse),
		);

		map(pair(first, afters), |(first, afters)| {
			Accessor(first, afters)
		})(input)
	}
}

#[derive(Clone, Debug)]
pub enum AccessorComponent {
	Property(VariableName),
	Index(Expression),
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

		alt((property, index))(input)
	}
}
