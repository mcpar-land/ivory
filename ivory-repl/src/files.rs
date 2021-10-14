use std::fs::File;

use crate::error::ReplError;
use ivory_runtime::runtime::Runtime;
use ivory_tokenizer::{tokenize, Module};
use rand::Rng;

pub struct FileLoader {}

impl FileLoader {
	pub fn load<R: Rng>(
		&mut self,
		runtime: &mut Runtime<R>,
		url: &str,
	) -> Result<(), ReplError> {
		let contents = std::fs::read_to_string(url)?;
		runtime.load(&contents)?;
		Ok(())
	}
	pub fn get_module(&self, url: &str) -> Result<Module, ReplError> {
		let contents = std::fs::read_to_string(url)?;
		Ok(tokenize(&contents)?)
	}
}

pub struct LoadedFile<S: FileSource> {
	body: String,
	source: S,
}

pub enum LocalSource {
	File { file: File },
	Web { url: String },
}

pub trait FileSource {
	fn load(url: &str) -> Self;
}
