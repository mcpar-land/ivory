use crate::{
	syntax::{function::Function, variable::VariableAssignment},
	IvoryError, Result,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLayer {
	pub variables: HashMap<String, (VariableAssignment, f64)>,
	pub functions: HashMap<String, Function>,
}

impl DataLayer {
	pub fn new() -> Self {
		DataLayer {
			variables: HashMap::new(),
			functions: HashMap::new(),
		}
	}

	pub fn deserialize(input: &[u8]) -> Result<Self> {
		Ok(bincode::deserialize(input)?)
	}

	pub fn serialize(&self) -> Result<Vec<u8>> {
		Ok(bincode::serialize(&self)?)
	}

	pub fn get_variable(&self, name: &str) -> Result<&(VariableAssignment, f64)> {
		self
			.variables
			.get(name)
			.ok_or(IvoryError::NameNotFound(name.to_string()))
	}
	pub fn set_variable(&mut self, name: &str, value: f64) -> Result<()> {
		todo!();
	}
	pub fn get_variable_mut(
		&mut self,
		name: &str,
	) -> Result<&mut (VariableAssignment, f64)> {
		self
			.variables
			.get_mut(name)
			.ok_or(IvoryError::NameNotFound(name.to_string()))
	}
	pub fn insert_variable(&mut self, assignment: VariableAssignment) {
		todo!();
	}
	pub fn get_function(&self, name: &str) -> Result<&Function> {
		todo!();
	}
	pub fn get_function_mut(&mut self, name: &str) -> Result<&mut Function> {
		todo!();
	}
	pub fn insert_function(&mut self, function: Function) {
		todo!();
	}
}
