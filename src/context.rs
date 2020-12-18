use crate::{
	data_layer::DataLayer,
	expression_result::ExpressionResult,
	syntax::{
		expression::Expression,
		function::Function,
		program::{Program, ProgramItem},
		variable::{VariableAssignment, VariableRange},
	},
	IvoryError, Parse, Result,
};
use std::collections::HashMap;

pub struct IvoryContext<'a> {
	pub program: &'a Program,
	pub variables: HashMap<String, (&'a VariableAssignment, f64)>,
	pub functions: HashMap<String, &'a Function>,
	pub data_layer: DataLayer,
}

impl<'a> IvoryContext<'a> {
	pub fn new(program: &'a Program) -> Self {
		let mut ctx = IvoryContext {
			program,
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

	pub fn eval(&self, name: &str) -> Result<ExpressionResult> {
		todo!();
	}

	pub fn eval_expr<R: Parse>(
		&self,
		expr: &Expression<R>,
	) -> Result<ExpressionResult> {
		todo!();
	}

	pub fn get_variable(
		&self,
		name: &str,
	) -> Result<(&VariableAssignment, &f64)> {
		self
			.variables
			.get(name)
			.map(|(a, b)| (*a, b))
			.or(self.data_layer.variables.get(name).map(|(a, b)| (a, b)))
			.ok_or(IvoryError::NameNotFound(name.to_string()))
	}
	pub fn set_variable(&mut self, name: &str, new: f64) -> Result<()> {
		let (range, mut val) = {
			let (dev, val) = self
				.variables
				.get_mut(name)
				.map(|(a, b)| (a.clone(), b))
				.or(
					self
						.data_layer
						.variables
						.get_mut(name)
						.map(|(a, b)| (a.clone(), b)),
				)
				.ok_or(IvoryError::NameNotFound(name.to_string()))?;
			*val = new;
			(dev.range.clone(), *val)
		};
		if let Some(VariableRange { min, max }) = range {
			if let Some(min) = min {
				val = val.max(self.eval_expr(&min)?.total());
			}
			if let Some(max) = max {
				val = val.min(self.eval_expr(&max)?.total());
			}
		}
		self
			.variables
			.get_mut(name)
			.ok_or(IvoryError::NameNotFound(name.to_string()))?
			.1 = val;
		Ok(())
	}
	pub fn get_function(&self, name: &str) -> Result<&Function> {
		self
			.functions
			.get(name)
			.map(|f| *f)
			.or(self.data_layer.functions.get(name))
			.ok_or(IvoryError::NameNotFound(name.to_string()))
	}
	pub fn insert_variable(&mut self, value: VariableAssignment) {
		self.data_layer.insert_variable(value);
	}
	pub fn insert_function(&mut self, value: Function) {
		self.data_layer.insert_function(value);
	}
	pub fn get_inserted_variable_mut(
		&mut self,
		name: &str,
	) -> Result<&mut (VariableAssignment, f64)> {
		self.data_layer.get_variable_mut(name)
	}
	pub fn get_inserted_function_mut(
		&mut self,
		name: &str,
	) -> Result<&mut Function> {
		self.data_layer.get_function_mut(name)
	}
}
