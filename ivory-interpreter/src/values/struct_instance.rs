use std::collections::HashMap;

use crate::{
	expression::Expression, istruct::StructName, variable::VariableName,
};

#[derive(Clone, Debug)]
pub struct StructInstance {
	pub name: StructName,
	pub values: HashMap<VariableName, Expression>,
}
