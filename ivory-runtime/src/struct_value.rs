use std::{collections::HashMap, fmt::Display};

use ivory_tokenizer::{
	istruct::StructName, values::struct_instance::StructInstance,
};
use rand::Rng;

use crate::{
	runtime::{Runtime, RuntimeContext},
	value::Value,
	Result, RuntimeError,
};

#[derive(Clone, Debug)]
pub struct StructValue {
	pub kind: StructName,
	pub values: HashMap<String, Value>,
}

impl StructValue {
	pub fn build<R: Rng>(
		runtime: &Runtime<R>,
		ctx: &RuntimeContext,
		instance: &StructInstance,
	) -> Result<Self> {
		if let Some(def) = runtime.values.structs.get(&instance.name.0) {
			let mut values = HashMap::new();
			for (name, expr) in instance.values.iter() {
				// Return an error if the struct instance has a field in it that isn't
				// present in the struct definition
				if !def.values.iter().any(|(def_name, _)| def_name.0 == name.0) {
					return Err(RuntimeError::FieldNotOnStruct(
						def.name.0.clone(),
						name.0.clone(),
					));
				}
				values.insert(
					name.0.clone(),
					runtime.math_to_value(runtime.pick_ternary(ctx, expr)?)?,
				);
			}
			Ok(Self {
				kind: instance.name.clone(),
				values,
			})
		} else {
			Err(RuntimeError::StructNotFound(instance.name.0.clone()))
		}
	}
}

impl Display for StructValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// TODO
		write!(f, "<struct value>")
	}
}
