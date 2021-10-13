use quick_error::quick_error;

quick_error! {
	#[derive(Debug, Clone)]
	pub enum ReplError {
		Runtime(err: ivory_runtime::RuntimeError) {
			from()
		}
		Io(err: String) {
			from(err: std::io::Error) -> (err.to_string())
		}
	}
}