use ivory_tokenizer;

#[cfg(test)]
#[test]
fn testing_files() {
	use ivory_tokenizer::{module::Module, Parse};

	let file_names = &["test01"];
	for name in file_names {
		let fname = format!("./tests/{}.iv", name);
		let data = std::fs::read_to_string(&fname).expect("Unable to read file");

		match Module::parse(&data) {
			Ok(module) => println!("{:#?}", module),
			Err(err) => panic!("Error loading module file {} -> {}", fname, err),
		}
	}
}
