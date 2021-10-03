use std::collections::HashMap;

use nom::{
	bytes::complete::tag,
	character::complete::multispace0,
	multi::separated_list0,
	sequence::{delimited, pair, separated_pair, tuple},
};

use crate::{
	expression::Expression, istruct::StructName, variable::VariableName, Parse,
};

#[derive(Clone, Debug)]
pub struct StructInstance {
	pub name: StructName,
	pub values: HashMap<VariableName, Expression>,
}

impl Parse for StructInstance {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		fn parse_values(
			input: &str,
		) -> nom::IResult<&str, HashMap<VariableName, Expression>> {
			let (input, pairs): (&str, Vec<(VariableName, Expression)>) = delimited(
				pair(tag("{"), multispace0),
				separated_list0(
					tuple((multispace0, tag(","), multispace0)),
					separated_pair(
						VariableName::parse,
						tuple((multispace0, tag(":"), multispace0)),
						Expression::parse,
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
			Ok((input, map))
		}

		let (input, (name, values)) =
			separated_pair(StructName::parse, multispace0, parse_values)(input)?;

		Ok((input, StructInstance { name, values }))
	}
}
