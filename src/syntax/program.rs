use nom::IResult;

use crate::{
	syntax::{function::Function, variable::VariableAssignment},
	Parse, Result,
};

#[derive(Debug, Clone)]
pub struct Program(Vec<ProgramItem>);

impl Program {
	pub fn new(input: &str) -> Result<Program> {
		todo!();
	}

	pub fn items(&self) -> &Vec<ProgramItem> {
		&self.0
	}
}

#[derive(Debug, Clone)]
pub enum ProgramItem {
	VariableAssignment(VariableAssignment),
	Function(Function),
}
