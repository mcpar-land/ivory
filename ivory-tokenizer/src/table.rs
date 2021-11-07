use std::{collections::HashMap, fmt::Display};

use ivory_expression::Expression;
use nom::{
	bytes::complete::tag,
	character::complete::{line_ending, space0, space1},
	combinator::verify,
	multi::separated_list1,
	sequence::{delimited, pair, preceded, tuple},
};

use crate::{
	accessor::Accessor,
	expression::{ExpressionToken, Op},
	values::{array::ArrayValue, object::ObjectValue, Value},
	variable::{Variable, VariableName},
	Parse,
};

#[derive(Clone, Debug)]
pub struct Table {
	pub name: VariableName,
	pub keys: Vec<VariableName>,
	pub values: Vec<Vec<Expression<Op, ExpressionToken>>>,
}

impl Table {
	pub fn into_variable(self) -> Variable {
		let mut exprs = Vec::new();
		for row in self.values {
			let mut obj = HashMap::new();
			for (val, key) in row.into_iter().zip(self.keys.iter()) {
				obj.insert(key.clone(), val);
			}
			exprs.push(Expression::<Op, _>::new(ExpressionToken::new(
				Value::Object(ObjectValue(obj)),
			)));
		}
		Variable {
			name: self.name,
			value: Expression::new(ExpressionToken::new(Value::Array(ArrayValue(
				exprs,
			)))),
		}
	}
}

impl Parse for Table {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, name) =
			preceded(pair(tag("table"), space1), VariableName::parse)(input)?;
		let (input, _) = tuple((space0, line_ending, space0))(input)?;

		fn row<'a, T, F: 'a>(
			internal: F,
		) -> impl FnMut(&'a str) -> nom::IResult<&'a str, Vec<T>>
		where
			F: FnMut(&'a str) -> nom::IResult<&'a str, T>,
		{
			delimited(
				pair(tag("|"), space0),
				separated_list1(tuple((space0, tag("|"), space0)), internal),
				pair(space0, tag("|")),
			)
		}

		let (input, keys) = row(VariableName::parse)(input)?;

		let (input, _) = tuple((space0, line_ending, space0))(input)?;

		let (input, values) = separated_list1(
			tuple((space0, line_ending, space0)),
			verify(
				row(Expression::<Op, ExpressionToken>::parse),
				|row: &Vec<Expression<Op, ExpressionToken>>| row.len() == keys.len(),
			),
		)(input)?;

		Ok((input, Table { name, keys, values }))
	}
}

impl Display for Table {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use pad::{Alignment, PadStr};
		let mut lens = Vec::new();
		for i in 0..self.keys.len() {
			let len = std::iter::once(self.keys[i].0.clone())
				.chain(self.values.iter().map(|v| v[i].to_string()))
				.fold(0, |len, v| v.len().max(len));
			lens.push(len);
		}
		write!(f, "table {}\n", self.name)?;
		write!(f, "|")?;
		for i in 0..lens.len() {
			write!(
				f,
				" {} |",
				self.keys[i]
					.to_string()
					.pad_to_width_with_alignment(lens[i], Alignment::Middle)
			)?;
		}
		for row in &self.values {
			write!(f, "\n|")?;
			for i in 0..lens.len() {
				write!(f, " {} |", row[i].to_string().pad_to_width(lens[i]))?;
			}
		}
		write!(f, "")
	}
}

#[cfg(test)]
mod test {
	use nom::Finish;

	use super::*;

	#[test]
	fn display_table() {
		let table = Table::parse(
			r#"table foosball_players
 | name | age | gold | power |
  | "foo guy" | 12 | 100 | "fire blasting" |
 | "bar master" | 14 | 0 | "death" |
| "bazlord of the infinite" | 10000 | 999 | "infinite bazlord jutsu" |"#,
		)
		.finish()
		.unwrap()
		.1;
		println!("{}", table);
	}
}
