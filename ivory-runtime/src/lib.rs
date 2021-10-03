use std::collections::HashMap;

pub mod error;
pub mod variable;

use ivory_tokenizer::tokenize;

pub use crate::error::{Result, RuntimeError};

pub struct Runtime {
	pub structs: HashMap<String, ()>,
	pub variables: HashMap<String, ()>,
}

impl Runtime {
	pub fn load(input: &str) -> Result<Self> {
		let module = tokenize(input)?;

		todo!();
	}
}
