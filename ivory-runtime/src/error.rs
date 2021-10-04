use ivory_tokenizer::{expression::math::ExprOpMath, ErrorKind};
use quick_error::quick_error;

pub type Result<T> = std::result::Result<T, RuntimeError>;

quick_error! {
	#[derive(Debug)]
	pub enum RuntimeError {
		Syntax(err: String) {
			from()
			display(s) -> ("Syntax error: {}", err)
		}
		CannotRunOp(lhs: &'static str, op: ExprOpMath, rhs: &'static str) {
			display(s) -> ("Cannot perform operation {} {} {}", lhs, op, rhs)
		}
	}
}
