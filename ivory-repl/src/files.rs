use std::{fs::File, path::PathBuf, str::FromStr};

use crate::error::ReplError;
use ivory_runtime::runtime::Runtime;
use ivory_tokenizer::{tokenize, Module};
use rand::Rng;

pub struct FileLoader {
	pub current_file: Option<PathBuf>,
}

impl FileLoader {
	pub fn load<R: Rng>(
		&mut self,
		runtime: &mut Runtime<R>,
		url: &str,
	) -> Result<(), ReplError> {
		let p = PathBuf::from_str(url).expect("Error getting path from string");
		self.current_file = Some(p);
		let contents = std::fs::read_to_string(url)?;
		runtime.load(&contents)?;
		Ok(())
	}
	pub fn get_module(&self, url: &str) -> Result<Module, ReplError> {
		let contents = std::fs::read_to_string(url)?;
		Ok(tokenize(&contents)?)
	}

	pub fn file_display_name(&self) -> Option<String> {
		if let Some(path) = &self.current_file {
			path.file_name().map(|os| os.to_string_lossy().to_string())
		} else {
			None
		}
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
