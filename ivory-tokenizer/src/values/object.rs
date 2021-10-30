use std::{collections::HashMap, fmt::Display};

use ivory_expression::{Expression, TernaryExpression};
use nom::{
	bytes::complete::tag,
	character::complete::multispace0,
	multi::separated_list0,
	sequence::{delimited, pair, separated_pair, tuple},
};

use crate::{
	expression::{ExpressionToken, Op},
	variable::VariableName,
	Parse,
};

#[derive(Clone, Debug)]
pub struct ObjectValue(
	pub HashMap<VariableName, TernaryExpression<Op, ExpressionToken>>,
);

impl Parse for ObjectValue {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, pairs): (
			&str,
			Vec<(VariableName, TernaryExpression<Op, ExpressionToken>)>,
		) = delimited(
			pair(tag("{"), multispace0),
			separated_list0(
				tuple((multispace0, tag(","), multispace0)),
				separated_pair(
					VariableName::parse,
					tuple((multispace0, tag(":"), multispace0)),
					TernaryExpression::parse,
				),
			),
			pair(multispace0, tag("}")),
		)(input)?;

		let mut map = HashMap::new();

		for (k, v) in pairs {
			if map.insert(k, v).is_some() {
				return Err(nom::Err::Error(nom::error::make_error(
					input,
					nom::error::ErrorKind::Verify,
				)));
			}
		}

		Ok((input, ObjectValue(map)))
	}
}

impl Display for ObjectValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{{object}}")
	}
}

#[cfg(test)]
#[test]
fn parse_object() {
	let o = r#"{
		foo_bar: 3 * 12,
		bar_baz: "okaychamp",
		pog_u: [false, false, false, true],
		nested: {
			goblin: true,
			bird: true,
			other_bird: true
		},
		faz: 3.4345,
		pythag: a b -> math.sqrt( a*a + b*b )
	}"#;

	let x = ObjectValue::parse(o).unwrap().1;
	println!("{:?}", x);

	let x = ObjectValue::parse("{}").unwrap().1;
	println!("{:?}", x);
}
