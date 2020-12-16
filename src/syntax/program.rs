use crate::{
	syntax::{function::Function, variable::VariableAssignment},
	Parse, Result,
};

#[derive(Debug, Clone)]
pub struct Program(Vec<ProgramItem>);

impl Program {
	pub fn items(&self) -> &Vec<ProgramItem> {
		&self.0
	}
}

impl Parse for Program {
	fn parse(input: &str) -> Result<(&str, Self)> {
		todo!()
	}
}

#[derive(Debug, Clone)]
pub enum ProgramItem {
	VariableAssignment(VariableAssignment),
	Function(Function),
}
