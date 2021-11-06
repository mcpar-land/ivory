use ivory_expression::{Expression, ExpressionComponent, Pair};
use ivory_runtime::{expr::RolledOp, value::Value};

pub fn contains_rolls(expr: &Expression<RolledOp, Value>) -> bool {
	if contains_rolls_cmp(&expr.first) {
		return true;
	} else {
		for Pair(_, cmp) in &expr.pairs {
			if contains_rolls_cmp(cmp) {
				return true;
			}
		}
	}
	return false;
}

fn contains_rolls_cmp(cmp: &ExpressionComponent<RolledOp, Value>) -> bool {
	match cmp {
		ExpressionComponent::Token(token) => match token {
			Value::Roll(_) => true,
			_ => false,
		},
		ExpressionComponent::Paren(expr) => contains_rolls(&expr),
	}
}
