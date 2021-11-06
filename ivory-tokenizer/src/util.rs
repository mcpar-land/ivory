use std::fmt::Display;

use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{
		alpha1, alphanumeric1, multispace0, multispace1, one_of, space0,
	},
	combinator::recognize,
	multi::{many0, many1},
	sequence::{pair, separated_pair, tuple},
	IResult,
};

use crate::{comment::SingleComment, Parse};

pub fn ws0(input: &str) -> nom::IResult<&str, &str> {
	alt((
		recognize(many1(tuple((
			multispace0,
			SingleComment::parse,
			multispace0,
		)))),
		multispace0,
	))(input)
}

pub fn ws1(input: &str) -> nom::IResult<&str, &str> {
	alt((
		recognize(many1(tuple((
			multispace0,
			SingleComment::parse,
			multispace0,
		)))),
		multispace1,
	))(input)
}

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
	recognize(pair(alpha1, many0(alt((alphanumeric1, tag("_"))))))(input)
}

pub fn test_multiple<'a, T: Parse>(inputs: &[&'a str]) {
	for input in inputs {
		match T::parse(input) {
			Ok(val) => {
				if val.0.len() != 0 {
					panic!(
						"Error parsing \"{}\" \n unfinished input: \n \"{}\"",
						input, val.0
					);
				} else {
					println!("{}", val.1);
				}
			}
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

pub fn comma_separated_display<T: Display>(vec: &Vec<T>) -> String {
	vec.iter().enumerate().fold(String::new(), |s, (i, v)| {
		format!("{}{}{}", s, v, if i == vec.len() - 1 { "" } else { ", " })
	})
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

#[test]
fn test_comment_whitespace0() {
	let mut test = separated_pair(tag("foo"), ws0, tag("bar"));
	assert_eq!(test("foobar").unwrap().1, ("foo", "bar"));
	assert_eq!(test("foo bar").unwrap().1, ("foo", "bar"));
	assert_eq!(
		test("foo # this is a comment\nbar").unwrap().1,
		("foo", "bar")
	);
	assert_eq!(
		test("foo \n # this is a comment\n\n\n# this is a comment too \n bar")
			.unwrap()
			.1,
		("foo", "bar")
	);
	assert!(test("foo # this is a comment bar").is_err());
}

#[test]
fn test_comment_whitespace1() {
	let mut test = separated_pair(tag("foo"), ws1, tag("bar"));
	assert_eq!(test("foo bar").unwrap().1, ("foo", "bar"));
	assert_eq!(
		test("foo #this is a comment\nbar").unwrap().1,
		("foo", "bar")
	);
	assert_eq!(
		test("foo#this is a comment\nbar").unwrap().1,
		("foo", "bar")
	);
	assert_eq!(
		test("foo#this is a comment\n bar").unwrap().1,
		("foo", "bar")
	);
	assert_eq!(
		test("foo #this is a comment\n bar").unwrap().1,
		("foo", "bar")
	);
	assert_eq!(
		test("foo # this is a comment\n#also a comment\nbar")
			.unwrap()
			.1,
		("foo", "bar")
	);
	assert!(test("foobar").is_err());
}
