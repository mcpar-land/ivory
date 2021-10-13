use std::fs::File;

use crate::error::ReplError;
use ivory_runtime::runtime::Runtime;
use ivory_tokenizer::Module;
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
