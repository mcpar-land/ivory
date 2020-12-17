use crate::Parse;
use nom::{
	bytes::complete::tag,
	character::complete::{multispace0, multispace1},
	combinator::{map, opt},
	number::complete::recognize_float,
	sequence::{pair, preceded, separated_pair},
	IResult,
};
use serde::{Deserialize, Serialize};

use super::{
	expression::Expression,
	number::Number,
	util::{identifier, ws},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableAssignment {
	pub name: String,
	pub initial: Number,
	pub range: Option<VariableRange>,
}

impl VariableAssignment {
	pub fn new(name: &str, initial: f64, range: Option<VariableRange>) -> Self {
		Self {
			name: name.to_string(),
			initial: Number(initial),
			range,
		}
	}
}

impl Parse for VariableAssignment {
	fn parse(input: &str) -> IResult<&str, Self> {
		let (input, name) = map(identifier, |v| String::from(v))(input)?;
		let (input, _) = multispace1(input)?;
		let (input, initial) =
			map(recognize_float, |s: &str| Number(s.parse().unwrap()))(input)?;
		let (input, _) = multispace0(input)?;
		let (input, range) =
			opt(preceded(pair(tag("~"), multispace0), VariableRange::parse))(input)?;
		Ok((
			input,
			VariableAssignment {
				name,
				initial,
				range,
			},
		))
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableRange {
	// Ranges need to be able to evaluate without rolling.
	pub min: Option<Expression<()>>,
	pub max: Option<Expression<()>>,
}

impl Parse for VariableRange {
	fn parse(input: &str) -> IResult<&str, Self> {
		let (input, (min, max)) = separated_pair(
			opt(Expression::parse),
			ws(tag("::")),
			opt(Expression::parse),
		)(input)?;
		Ok((input, VariableRange { min, max }))
	}
}

#[cfg(test)]
mod test {
	use crate::syntax::util::test_utils::number_expression;

	use super::*;

	#[test]
	fn test_variable_range() {
		assert_eq!(
			VariableRange::parse("0.0::100.0").unwrap().1,
			VariableRange {
				min: Some(number_expression(0.0)),
				max: Some(number_expression(100.0))
			}
		);
		assert_eq!(
			VariableRange::parse("0::100").unwrap().1,
			VariableRange {
				min: Some(number_expression(0.0)),
				max: Some(number_expression(100.0))
			}
		);
	}

	#[test]
	fn test_assign_variable() {
		assert_eq!(
			VariableAssignment::parse("variable_name 20").unwrap().1,
			VariableAssignment {
				name: "variable_name".to_string(),
				initial: Number(20.0),
				range: None
			}
		);
		assert_eq!(
			VariableAssignment::parse("variable_name 20 ~ 0::100")
				.unwrap()
				.1,
			VariableAssignment {
				name: "variable_name".to_string(),
				initial: Number(20.0),
				range: Some(VariableRange {
					min: Some(number_expression(0.0)),
					max: Some(number_expression(100.0))
				})
			}
		);
	}
}
