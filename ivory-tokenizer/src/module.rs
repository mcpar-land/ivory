use std::fmt::Display;

use nom::{
	combinator::{map, opt},
	multi::many1,
	sequence::{pair, preceded, terminated},
};

use crate::{commands::Command, comment::SingleComment, util::ws0, Parse};

pub mod iuse;

#[derive(Clone, Debug)]
pub struct Module(pub Vec<Command>);

impl Parse for Module {
	fn parse(input: &str) -> nom::IResult<&str, Self> {
		map(preceded(ws0, many1(terminated(Command::parse, ws0))), |v| {
			Self(v)
		})(input)
	}
}

impl Display for Module {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			self
				.0
				.iter()
				.fold(String::new(), |s, val| { format!("{}\n{}", s, val) })
		)
	}
}

#[cfg(test)]
#[test]
fn parse_module() {
	crate::util::test_multiple::<Module>(&[
		r#"
		use * from "wahifujdfdsaf";
		use a, b, c from "9egtuh";

		struct Foo {
			bar: int,
			baz: string
		}

		x = 10 + 100;
		y = a -> bcdefg;
		bazinga = [1, 2, 3, 4];
		"#,
		r#"
		
		
		struct Bar {
			a: decimal
		}
		
		
		"#,
		r#"
		# comment
		x # comment
		= # comment
		10 # comment
		+ # comment
		12 # comment
		# comment
		- # comment
		69 # comment
		; # comment
		# comment
		y = [ # some comment
			10, # comment
			11, # another comment
			12  # an entire comment
		]; #comment
		# comment
		"#,
	]);

	crate::util::test_multiple_should_fail::<Module>(&[r#"
		struct Foo {
			bar: int
		} x = 10;
		"#])
}
