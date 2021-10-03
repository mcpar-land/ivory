pub mod math;

use nom::{
	branch::alt,
	character::complete::{char, multispace0},
	combinator::map,
	multi::{many0, separated_list0},
	sequence::{delimited, pair, preceded, separated_pair},
};

use crate::{accessor::Accessor, values::Value, Parse};

use self::math::ExprOpMath;

#[derive(Clone, Debug)]
pub struct Expression {
	pub first: ExpressionComponent,
	pub pairs: Vec<ExpressionPair>,
}

impl Expression {
	pub fn has_dice_roll(&self) -> bool {
		todo!();
	}

	/// If there's a single accessor anywhere in here
	pub fn has_accessors(&self) -> bool {
		todo!();
	}
}

impl Parse for Expression {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, first) = ExpressionComponent::parse(input)?;
		let (input, pairs) =
			many0(preceded(multispace0, ExpressionPair::parse))(input)?;

		Ok((input, Self { first, pairs }))
	}
}

#[derive(Clone, Debug)]
pub enum ExpressionComponent {
	Value(Value),
	Accessor(Accessor),
	Paren(Box<Expression>),
}

impl ExpressionComponent {
	pub fn is_accessor(&self) -> bool {
		match self {
			Self::Accessor(_) => true,
			_ => false,
		}
	}
}

impl Parse for ExpressionComponent {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			map(Value::parse, |r| Self::Value(r)),
			map(Accessor::parse, |r| Self::Accessor(r)),
			map(delimited(char('('), Expression::parse, char(')')), |r| {
				Self::Paren(Box::new(r))
			}),
		))(input)
	}
}

#[derive(Clone, Debug)]
pub struct ExpressionPair {
	pub op: ExprOpMath,
	pub component: ExpressionComponent,
}

impl Parse for ExpressionPair {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			separated_pair(
				ExprOpMath::parse,
				multispace0,
				ExpressionComponent::parse,
			),
			|(op, component)| ExpressionPair { op, component },
		)(input)
	}
}

#[cfg(test)]
#[test]
fn parse_expression() {
	use crate::util::test_multiple;

	test_multiple::<Expression>(&[
		"11 + 3",
		"11+3",
		"5 * 4 + 13 - 2",
		"5\n *4+ 13    -2 ",
		"(11 + 3) / (3 + 11)",
		"38 /^ 3",
		"18 + bogos[34] / binted[8 * 8]",
		"((((((((((((69))))))))))))",
	]);
}
