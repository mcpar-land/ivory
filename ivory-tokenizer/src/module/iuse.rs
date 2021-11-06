use std::fmt::Display;

use nom::{
	branch::alt,
	bytes::complete::tag,
	combinator::{map, opt, value},
	multi::separated_list1,
	sequence::{pair, preceded, terminated, tuple},
};

use crate::{
	util::{comma_separated_display, ws0, ws1},
	values::string::StringValue,
	variable::VariableName,
	Parse,
};

#[derive(Clone, Debug)]
pub struct Use {
	pub froms: Froms,
	pub path: StringValue,
}

impl Parse for Use {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		let (input, _) = terminated(tag("use"), ws1)(input)?;
		let (input, froms) = terminated(Froms::parse, ws1)(input)?;
		let (input, _) = terminated(tag("from"), ws1)(input)?;
		let (input, path) = StringValue::parse(input)?;

		Ok((input, Self { froms, path }))
	}
}

#[derive(Clone, Debug)]
pub struct As<S: Parse> {
	pub source: S,
	pub alias: Option<VariableName>,
}

impl<S: Parse> Parse for As<S> {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(
			pair(
				S::parse,
				opt(preceded(tuple((ws1, tag("as"), ws1)), VariableName::parse)),
			),
			|(source, alias)| Self { source, alias },
		)(input)
	}
}

#[derive(Clone, Debug)]
pub enum Froms {
	Asterix,
	Variables(Vec<As<VariableName>>),
}

impl Parse for Froms {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		alt((
			value(Self::Asterix, tag("*")),
			map(
				separated_list1(tuple((ws0, tag(","), ws0)), As::<VariableName>::parse),
				|variables| Self::Variables(variables),
			),
		))(input)
	}
}

impl Display for Use {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "use {} from {}", self.froms, self.path)
	}
}

impl<S: Parse> Display for As<S> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(a) = &self.alias {
			write!(f, "{} as {}", self.source, a)
		} else {
			write!(f, "{}", self.source)
		}
	}
}

impl Display for Froms {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Froms::Asterix => write!(f, "*"),
			Froms::Variables(vars) => {
				write!(f, "{}", comma_separated_display(&vars))
			}
		}
	}
}

#[cfg(test)]
#[test]
fn parse_use() {
	use crate::util::test_multiple;

	test_multiple::<Use>(&[
		"use * from \"./foo.ivory\"",
		"use a, b, c from \"amognus\"",
		"use foo as an_alias_name, bar as another_alias_name, \
		baz as wig_wog_wooooly from \"hggggggggggggggshihegrerg\"",
	]);
}
