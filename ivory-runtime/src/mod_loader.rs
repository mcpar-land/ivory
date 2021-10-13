use crate::{Result, RuntimeError};
use ivory_tokenizer::{tokenize, Module};

pub trait ModLoader {
	type Error;
	fn load(&mut self, url: &str) -> std::result::Result<Module, Self::Error>;
}

/// Mod loader where the URL is the raw text of the file, not an actual URL
pub struct RawLoader;

impl ModLoader for RawLoader {
	type Error = RuntimeError;
	fn load(&mut self, url: &str) -> Result<Module> {
		Ok(tokenize(url)?)
	}
}
