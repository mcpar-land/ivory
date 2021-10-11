use crate::{value::Value, Result};
use ivory_expression::Expression;
use ivory_tokenizer::{
	accessor::Accessor,
	expression::{ExpressionToken, Op},
	tokenize,
	variable::Variable,
};
use std::collections::BTreeMap;

pub struct Runtime {
	pub structs: BTreeMap<String, ()>,
	pub variables: BTreeMap<String, Variable>,
}

impl Runtime {
	pub fn load(input: &str) -> Result<Self> {
		let module = tokenize(input)?;

		let mut structs = BTreeMap::new();
		let mut variables = BTreeMap::new();

		for command in module.0.into_iter() {
			match command {
				ivory_tokenizer::commands::Command::Variable(variable) => {
					variables.insert(variable.name.0.clone(), variable);
				}
				ivory_tokenizer::commands::Command::StructDefinition => todo!(),
			}
		}

		Ok(Self { structs, variables })
	}

	pub fn access(
		&self,
		ctx: &RuntimeContext,
		Accessor(var, components): &Accessor,
	) -> Result<Value> {
		let param_value = ctx.params.get(&var.0);
		todo!();
	}

	pub fn execute(
		&self,
		ctx: &RuntimeContext,
		expr: &Expression<Op, ExpressionToken>,
	) -> Result<Value> {
		todo!();
	}
}

/// For handling context inside of functions
pub struct RuntimeContext {
	pub params: BTreeMap<String, Value>,
}
