use crate::{error::RuntimeError, Result};
use ivory_tokenizer::Module;

pub trait ModLoader {
	fn load(&mut self, url: &str) -> Result<Module>;
}

impl ModLoader for () {
	fn load(&mut self, _: &str) -> Result<Module> {
		Err(RuntimeError::NoModLoaderSpecified)
	}
}
