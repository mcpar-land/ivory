use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{alpha1, alphanumeric1, multispace0, one_of},
	combinator::recognize,
	multi::many0,
	sequence::{pair, tuple},
	IResult,
};

use crate::Parse;

pub fn snake_case(input: &str) -> IResult<&str, &str> {
	recognize(pair(
		one_of("_abcdefghijklmnopqrstuvwxyz"),
		many0(one_of("_abcdefghijklmnopqrstuvwxyz1234567890")),
	))(input)
}

pub fn upper_camel_case(input: &str) -> IResult<&str, &str> {
	recognize(pair(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), many0(alpha1)))(input)
}

pub fn variable_name(input: &str) -> IResult<&str, &str> {
	recognize(pair(
		alt((alpha1, tag("_"))),
		many0(alt((alphanumeric1, tag("_")))),
	))(input)
}

pub fn test_multiple<'a, T: Parse>(inputs: &[&'a str]) {
	for input in inputs {
		match T::parse(input) {
			Ok(val) => println!("{:?}", val),
			Err(err) => panic!("Error parsing \"{}\" -> {:?}", input, err),
		}
	}
}

pub fn test_multiple_should_fail<'a, T: Parse>(inputs: &[&'a str]) {
	for input in inputs {
		if let Ok(_) = T::parse(input) {
			panic!("Expected error parsing \"{}\", got Ok", input);
		}
	}
}

#[cfg(test)]
#[test]
fn test_snake_case() {
	assert!(snake_case("this_is_a_snake_case_word").is_ok());
	assert!(snake_case("ThisIsNotSnakeCase").is_err());
	assert!(snake_case("_this_is_snake_case").is_ok());
}

#[test]
fn test_variable_name() {
	let variables = [
		"this_is_a_variable_name",
		"ThisIsAVariableName",
		"thisisavariablename123",
	];
	let bad_variables = ["69badname", " ooooo"];
	for v in &variables {
		assert!(variable_name(v).is_ok());
	}
	for v in &bad_variables {
		assert!(variable_name(v).is_err());
	}
}
