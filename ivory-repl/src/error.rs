use quick_error::quick_error;

quick_error! {
	#[derive(Debug, Clone)]
	pub enum ReplError {
		Runtime(err: ivory_runtime::RuntimeError) {
			from()
			display("{}", err)
		}
		Tokenizer(err: ivory_tokenizer::TokenizerError) {
			from()
			display("Syntax error: {}", err)
		}
		Io(err: String) {
			from(err: std::io::Error) -> (err.to_string())
			display("I/O error: {}", err)
		}
	}
}
