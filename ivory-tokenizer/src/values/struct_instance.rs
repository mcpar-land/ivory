use std::{collections::HashMap, fmt::Display};

use ivory_expression::Expression;
use nom::{
	bytes::complete::tag,
	multi::separated_list0,
	sequence::{delimited, pair, separated_pair, tuple},
};

use crate::{
	expression::{ExpressionToken, Op},
	istruct::StructName,
	util::ws0,
	variable::VariableName,
	Parse,
};

#[derive(Clone, Debug)]
pub struct StructInstance {
	pub name: StructName,
	pub values: HashMap<VariableName, Expression<Op, ExpressionToken>>,
}

impl Parse for StructInstance {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		fn parse_values(
			input: &str,
		) -> nom::IResult<
			&str,
			HashMap<VariableName, Expression<Op, ExpressionToken>>,
		> {
			let (input, pairs): (
				&str,
				Vec<(VariableName, Expression<Op, ExpressionToken>)>,
			) = delimited(
				pair(tag("{"), ws0),
				separated_list0(
					tuple((ws0, tag(","), ws0)),
					separated_pair(
						VariableName::parse,
						tuple((ws0, tag(":"), ws0)),
						Expression::parse,
					),
				),
				pair(ws0, tag("}")),
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
			Ok((input, map))
		}

		let (input, (name, values)) =
			separated_pair(StructName::parse, ws0, parse_values)(input)?;

		Ok((input, StructInstance { name, values }))
	}
}

impl Display for StructInstance {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// TODO
		write!(f, "{{{}}}", self.name)
	}
}
