use std::collections::HashMap;

use crate::{error::ModLoaderError, runtime::RuntimeValues, Result};
use ivory_tokenizer::{
	module::iuse::{Froms, Use},
	tokenize,
	variable::Variable,
	Module,
};

pub trait ModLoader {
	fn load(
		&mut self,
		url: &str,
		parent_path: &str,
	) -> std::result::Result<String, ModLoaderError>;

	fn zinger(&self) -> Option<String> {
		None
	}
}

impl ModLoader for () {
	fn load(
		&mut self,
		_: &str,
		_: &str,
	) -> std::result::Result<String, ModLoaderError> {
		Err(ModLoaderError::NoModLoaderSpecified)
	}
}

pub struct LoadedModule {
	pub values: RuntimeValues,
	/// maps aliases to real variable names within the module
	pub froms: ModuleImports,
}

impl LoadedModule {
	pub fn new(
		loader: &mut Box<dyn ModLoader>,
		src: &Use,
		parent: &str,
	) -> Result<Self> {
		Ok(Self {
			values: RuntimeValues::new(
				tokenize::<Module>(&loader.load(src.path.0.as_str(), parent)?)?,
				src.path.0.as_str(),
				loader,
			)?,
			froms: match &src.froms {
				Froms::Asterix => ModuleImports::Asterix,
				Froms::Variables(froms) => ModuleImports::Aliases(
					froms
						.iter()
						.map(|a| {
							(
								a.alias.clone().unwrap_or_else(|| a.source.clone()).0,
								a.source.0.clone(),
							)
						})
						.collect(),
				),
			},
		})
	}
	pub fn get_variable(&self, name: &str) -> Option<&Variable> {
		match &self.froms {
			ModuleImports::Asterix => self.values.get_variable(name),
			ModuleImports::Aliases(aliases) => aliases
				.get(name)
				.map(|n| self.values.get_variable(n))
				.flatten(),
		}
	}
}

pub enum ModuleImports {
	Asterix,
	Aliases(HashMap<String, String>),
}

#[cfg(test)]
mod test {

	use crate::runtime::Runtime;

	use super::*;

	const MODS: &'static [&'static str] = &[
		r#"a_foo = 10; a_bar = 20;"#,
		r#"b_foo = "ten"; b_bar = "twenty";"#,
		r#"c_foo = 5 + 5; c_bar = some -> some + 20;"#,
	];

	struct DummyLoader;

	impl ModLoader for DummyLoader {
		fn load(
			&mut self,
			url: &str,
			_: &str,
		) -> std::result::Result<String, ModLoaderError> {
			Ok(match url {
				"a" => MODS[0].to_string(),
				"b" => MODS[1].to_string(),
				"c" => MODS[2].to_string(),
				_ => unreachable!(),
			})
		}
	}

	fn dummy_runtime(v: &str, run: &str) -> Result<String> {
		let mut runtime = Runtime::new(rand::thread_rng(), DummyLoader);
		runtime.load(v, "").unwrap();
		runtime.run(run).map(|v| format!("{}", v))
	}

	fn dummy_runtime_ok(v: &str, run: &str) {
		println!("{}", dummy_runtime(v, run).unwrap());
	}

	fn dummy_runtime_err(v: &str, run: &str) {
		assert!(dummy_runtime(v, run).is_err());
	}

	#[test]
	fn load_modules() {
		let m = r#"
		use * from "a";
		use * from "b";
		use * from "c";

		power = a_foo + a_bar;
		"#;
		dummy_runtime_ok(m, "power");
		dummy_runtime_ok(m, "power + a_foo");
		dummy_runtime_ok(m, "3 + c_bar(10) + b_foo");
		let m2 = r#"
			use a_foo from "a";
			good = a_foo + 10;
			bad = a_bar + 10;
		"#;
		dummy_runtime_ok(m2, "good");
		dummy_runtime_err(m2, "bad");
	}
	#[test]
	fn load_alias() {
		let m = r#"
		use a_foo as a_foo_aliased from "a";
		
		good = a_foo_aliased + 10;
		bad = a_foo + 10;
		"#;
		dummy_runtime_ok(m, "good");
		dummy_runtime_err(m, "bad");
	}

	#[test]
	fn override_load_modules() {
		let m = r#"
		use c_foo from "c";

		c_foo = 1000;
		"#;
		dummy_runtime_ok(m, "c_foo");
		let m2 = r#"
		use c_foo as c_alias from "c";

		c_alias = 999;
		c_foo = 1000 + c_alias;
		"#;
		dummy_runtime_ok(m2, "c_foo");
	}
}
