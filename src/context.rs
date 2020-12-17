use crate::{
	data_layer::DataLayer,
	expression_result::ExpressionResult,
	syntax::{
		function::Function,
		program::{Program, ProgramItem},
		variable::VariableAssignment,
	},
	Result,
};
use std::collections::HashMap;

pub struct IvoryContext<'a> {
	pub variables: HashMap<String, (&'a VariableAssignment, f64)>,
	pub functions: HashMap<String, &'a Function>,
	pub data_layer: DataLayer,
}

impl<'a> IvoryContext<'a> {
	pub fn new(program: &'a Program) -> Self {
		let mut ctx = IvoryContext {
			variables: HashMap::new(),
			functions: HashMap::new(),
			data_layer: DataLayer::new(),
		};
		for item in program.items() {
			match item {
				ProgramItem::VariableAssignment(v) => {
					ctx.variables.insert(v.name.clone(), (&v, v.initial.0));
				}
				ProgramItem::Function(f) => {
					ctx.functions.insert(f.name.clone(), &f);
				}
			}
		}
		ctx
	}

	pub fn roll(&self, name: &str) -> Result<ExpressionResult> {
		todo!();
	}

	pub fn get_variable(
		&self,
		name: &str,
	) -> Result<(&'a VariableAssignment, &f64)> {
		todo!();
	}
	pub fn set_variable(&mut self, name: &str) -> crate::Result<()> {
		todo!();
	}
	pub fn get_function(&self, name: &str) -> Option<&'a Function> {
		todo!();
	}
	pub fn insert_variable(&mut self, value: VariableAssignment) {
		todo!();
	}
	pub fn insert_function(&mut self, value: Function) {
		todo!();
	}
	pub fn get_inserted_variable_mut(
		&mut self,
		name: &str,
	) -> Option<&mut (VariableAssignment, f64)> {
		todo!();
	}
}
