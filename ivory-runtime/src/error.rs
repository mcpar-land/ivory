use ivory_tokenizer::{expression::Op, ErrorKind};
use quick_error::quick_error;

use crate::value::ValueKind;

pub type Result<T> = std::result::Result<T, RuntimeError>;

quick_error! {
	#[derive(Debug, Clone)]
	pub enum RuntimeError {
		Syntax(err: ivory_tokenizer::TokenizerError) {
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
		VariableNotFound(var: String) {
			display(s) -> ("Variable not found: {}", var)
		}
		NoPropertyOnKind(kind: ValueKind, prop: String) {
			display(s) -> ("Can't get prop {} of kind {}", prop, kind)
		}
		CannotIndexKind(kind: ValueKind) {
			display(s) -> ("kind {} cannot be indexed", kind)
		}
		CannotCallKind(kind: ValueKind) {
			display(s) -> ("Can't perform a function call on kind {}", kind)
		}
		PropNotFound(prop: String) {
			display(s) -> ("Cannot find prop {}", prop)
		}
		IndexOutOfBounds(i: usize, max: usize) {
			display(s) -> ("Index {} out of bounds (0 to {})", i, max - 1)
		}
	}
}
