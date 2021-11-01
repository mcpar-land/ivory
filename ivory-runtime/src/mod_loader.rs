use crate::{error::RuntimeError, Result};
use ivory_tokenizer::{tokenize, Module};

pub trait ModLoader {
	type Error;
	fn load(&mut self, url: &str) -> std::result::Result<Module, Self::Error>;
}

impl ModLoader for () {
	type Error = RuntimeError;

	fn load(&mut self, _: &str) -> std::result::Result<Module, Self::Error> {
		Err(RuntimeError::NoModLoaderSpecified)
	}
}
