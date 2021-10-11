use ivory_tokenizer::{expression::Op, ErrorKind};
use quick_error::quick_error;

use crate::value::ValueKind;

pub type Result<T> = std::result::Result<T, RuntimeError>;

quick_error! {
	#[derive(Debug, Clone)]
	pub enum RuntimeError {
		Syntax(err: String) {
			from()
			display(s) -> ("Syntax error: {}", err)
		}
		CannotRunOp(lhs: ValueKind, op: Op, rhs: ValueKind) {
			display(s) -> ("Cannot perform operation {} {} {}", lhs, op, rhs)
		}
		WrongExpectedValue(expected: ValueKind, got: ValueKind) {
			display(s) -> ("Expected value type {}, got {}", expected, got)
		}
		IncompatibleDiceOps {
			display(s) -> ("Incompatible dice ops")
		}
		NegativeDiceNumber {
			display(s) -> ("Numbers in dice rolls cannot be negative")
		}
	}
}
