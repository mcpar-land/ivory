use ivory_tokenizer::ErrorKind;
use quick_error::quick_error;

pub type Result<T> = std::result::Result<T, RuntimeError>;

quick_error! {
	#[derive(Debug)]
	pub enum RuntimeError {
		Syntax(err: (String, ErrorKind)) {
			from()
			display(s) -> ("Syntax error: {}", err.0)
		}
	}
}
