use std::fmt::Display;

use ivory_expression::{Expression, TernaryExpression};
use nom::{
	branch::alt,
	character::complete::{char, multispace0},
	combinator::map,
	multi::{many0, separated_list0},
	sequence::{delimited, pair, preceded},
};

use crate::{
	expression::{ExpressionToken, Op},
	util::comma_separated_display,
	variable::VariableName,
	Parse,
};

#[derive(Clone, Debug)]
pub struct Accessor(pub VariableName, pub Vec<AccessorComponent>);

impl Parse for Accessor {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let first = VariableName::parse;

		let afters = many0(preceded(multispace0, AccessorComponent::parse));

		map(pair(first, afters), |(first, afters)| {
			Accessor(first, afters)
		})(input)
	}
}

impl Display for Accessor {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}{}",
			self.0,
			self
				.1
				.iter()
				.fold(String::new(), |s, v| { format!("{}{}", s, v) })
		)
	}
}

#[derive(Clone, Debug)]
pub enum AccessorComponent {
	Property(VariableName),
	Index(TernaryExpression<Op, ExpressionToken>),
	Call(Vec<TernaryExpression<Op, ExpressionToken>>),
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
				TernaryExpression::<Op, ExpressionToken>::parse,
				pair(multispace0, char(']')),
			),
			|e| AccessorComponent::Index(e),
		);
		let call = map(
			delimited(
				pair(char('('), multispace0),
				separated_list0(
					delimited(multispace0, char(','), multispace0),
					TernaryExpression::<Op, ExpressionToken>::parse,
				),
				pair(multispace0, char(')')),
			),
			|vals| AccessorComponent::Call(vals),
		);

		alt((property, index, call))(input)
	}
}

impl Display for AccessorComponent {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			AccessorComponent::Property(p) => write!(f, ".{}", p),
			AccessorComponent::Index(i) => write!(f, "[{}]", i),
			AccessorComponent::Call(c) => write!(f, "{}", comma_separated_display(c)),
		}
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
