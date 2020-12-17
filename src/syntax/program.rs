use nom::IResult;

use crate::{
	syntax::{function::Function, variable::VariableAssignment},
	Parse, Result,
};

use super::util::ws;

#[derive(Debug, Clone, PartialEq)]
pub struct Program(pub Vec<ProgramItem>);

impl Program {
	pub fn new(input: &str) -> Result<Program> {
		use nom::{combinator::map, multi::many1};
		Ok(map(many1(ws(ProgramItem::parse)), |v| Program(v))(input)?.1)
	}

	pub fn items(&self) -> &Vec<ProgramItem> {
		&self.0
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgramItem {
	VariableAssignment(VariableAssignment),
	Function(Function),
}

impl Parse for ProgramItem {
	fn parse(input: &str) -> IResult<&str, Self> {
		use nom::{branch::alt, bytes::complete::tag, combinator::map};

		let (input, v) = alt((
			map(VariableAssignment::parse, |v| {
				ProgramItem::VariableAssignment(v)
			}),
			map(Function::parse, |v| ProgramItem::Function(v)),
		))(input)?;
		let (input, _) = tag(";")(input)?;
		Ok((input, v))
	}
}

#[cfg(test)]
mod test {
	use crate::syntax::{
		expression::{Expression, ExpressionItem, ExpressionOperator},
		function::FunctionCall,
		util::test_utils::number_expression,
		variable::VariableRange,
	};

	use super::*;

	#[test]
	fn test_program() {
		use crate::syntax::expression::ExpressionOperatorOp::*;

		let raw = r#"
			var1 10;
			var2 10 ~ -60::60;
			var3 0; var4 101.1;
			
			fn1: var1 * var2;
			fn2 arg1 arg2: arg1 + arg2 + fn1;
		"#;
		let program = Program::new(raw).unwrap();
		println!("{:#?}", program);
		assert_eq!(
			program,
			Program(vec![
				ProgramItem::VariableAssignment(VariableAssignment::new(
					"var1", 10.0, None
				)),
				ProgramItem::VariableAssignment(VariableAssignment::new(
					"var2",
					10.0,
					Some(VariableRange {
						min: Some(number_expression(-60.0)),
						max: Some(number_expression(60.0))
					})
				)),
				ProgramItem::VariableAssignment(VariableAssignment::new(
					"var3", 0.0, None
				)),
				ProgramItem::VariableAssignment(VariableAssignment::new(
					"var4", 101.1, None
				)),
				ProgramItem::Function(Function {
					name: "fn1".to_string(),
					arguments: vec![],
					expression: Expression::new(
						ExpressionItem::FunctionCall(FunctionCall::new("var1", vec![])),
						vec![(
							ExpressionOperator::new(None, Mul),
							ExpressionItem::FunctionCall(FunctionCall::new("var2", vec![]))
						)]
					)
				}),
				ProgramItem::Function(Function {
					name: "fn2".to_string(),
					arguments: vec!["arg1".to_string(), "arg2".to_string()],
					expression: Expression::new(
						ExpressionItem::FunctionCall(FunctionCall::new("arg1", vec![])),
						vec![
							(
								ExpressionOperator::new(None, Add),
								ExpressionItem::FunctionCall(FunctionCall::new("arg2", vec![]))
							),
							(
								ExpressionOperator::new(None, Add),
								ExpressionItem::FunctionCall(FunctionCall::new("fn1", vec![]))
							)
						]
					)
				}),
			])
		);
	}

	#[test]
	fn parse_example_file() {
		let file = std::fs::read_to_string("./example/pathfinder.ivory").unwrap();

		Program::new(file.as_str()).unwrap();
	}
}
