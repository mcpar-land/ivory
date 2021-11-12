use quick_error::quick_error;

use crate::{expr::RolledOp, value::ValueKind};

pub type Result<T> = std::result::Result<T, RuntimeError>;

quick_error! {
	#[derive(Debug, Clone)]
	pub enum RuntimeError {
		Syntax(err: ivory_tokenizer::TokenizerError) {
			from()
			display(s) -> ("Syntax error: {}", err)
		}
		CannotRunOp(lhs: ValueKind, op: RolledOp, rhs: ValueKind) {
			display(s) -> ("Cannot perform operation {} {} {}", lhs, op, rhs)
		}
		WrongExpectedValue(expected: ValueKind, got: ValueKind) {
			display(s) -> ("Expected value type {}, got {}", expected, got)
		}
		IncompatibleDiceOps {
			display(s) -> ("Incompatible dice ops")
		}
		NegativeNumber {
			display(s) -> ("Supplied number cannot be negative")
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
		StructNotFound(struct_name: String) {
			display(s) -> ("Struct type {} not found", struct_name)
		}
		FieldNotOnStruct(struct_name: String, field_name: String) {
			display(s) -> ("Field {} not present on struct {}", field_name, struct_name)
		}
		NoModLoaderSpecified {
			display(s) -> ("This runtime doesn't have a specified module loader.")
		}
		NoStdFnForKind(fn_name: String, kind: ValueKind) {
			display(s) -> ("No standard function {} for kind \"{}\"", fn_name, kind)
		}
		BadStdFnCall(info: String) {
			display(s) -> ("{}", info)
		}
	}
}
