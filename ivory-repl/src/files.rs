use std::{collections::HashMap, path::PathBuf, str::FromStr};

use ivory_runtime::{error::ModLoaderError, mod_loader::ModLoader};
use relative_path::RelativePath;

pub struct FileLoader {
	pub current_file: Option<PathBuf>,
	pub loaded_files: HashMap<String, LocalSource>,
}

impl FileLoader {
	pub fn new() -> Self {
		Self {
			current_file: None,
			loaded_files: HashMap::new(),
		}
	}
}

impl ModLoader for FileLoader {
	fn load(
		&mut self,
		url: &str,
		parent_path: &str,
	) -> Result<String, ModLoaderError> {
		if url.starts_with("http://") || url.starts_with("https://") {
			match self.loaded_files.get(url) {
				Some(LocalSource::Web { cache }) => Ok(cache.clone()),
				None => {
					println!("Fetching {}", url);
					reqwest::blocking::get(url)
						.and_then(|res| res.text())
						.map_err(|e| {
							ModLoaderError::ErrorLoadingModule(url.to_string(), e.to_string())
						})
				}
				Some(LocalSource::File { .. }) => unreachable!(),
			}
		} else {
			let rel_p = RelativePath::new(url);
			let mut p = PathBuf::from_str(parent_path).map_err(|e| {
				ModLoaderError::ErrorLoadingModule(url.to_string(), e.to_string())
			})?;
			if p.is_file() {
				p.pop();
			}
			let final_path = rel_p.to_logical_path(p);

			self.current_file = Some(final_path.clone());

			Ok(std::fs::read_to_string(&final_path).map_err(|e| {
				ModLoaderError::ErrorLoadingModule(
					final_path.as_os_str().to_str().unwrap().to_string(),
					e.to_string(),
				)
			})?)
		}
	}

	fn zinger(&self) -> Option<String> {
		if let Some(path) = &self.current_file {
			path.file_name().map(|os| os.to_string_lossy().to_string())
		} else {
			None
		}
	}
}

pub enum LocalSource {
	File { path: String },
	Web { cache: String },
}

impl LocalSource {
	fn load(url: &str) -> Result<Self, ModLoaderError> {
		todo!();
	}
}
