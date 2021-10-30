use ivory_expression::{ternary::Ternary, Expression};
use nom::{
	bytes::complete::tag,
	character::complete::multispace0,
	combinator::{map, opt},
	sequence::{pair, preceded, separated_pair, tuple},
};

use crate::{
	expression::{ExpressionToken, Op},
	Parse,
};

impl Parse for Ternary<Op, ExpressionToken> {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, first) = Expression::parse(input)?;

		let (input, conditions) = opt(map(
			preceded(
				pair(multispace0, tag("?")),
				separated_pair(
					Self::parse,
					tuple((multispace0, tag(":"), multispace0)),
					Self::parse,
				),
			),
			|both| Box::new(both),
		))(input)?;

		Ok((
			input,
			Self {
				condition: first,
				options: conditions,
			},
		))
	}
}
