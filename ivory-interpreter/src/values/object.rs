use std::collections::HashMap;

use crate::variable::VariableName;

use super::Value;

#[derive(Clone, Debug)]
pub struct ObjectValue(HashMap<VariableName, Value>);
