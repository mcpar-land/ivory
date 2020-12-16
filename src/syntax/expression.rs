use crate::{syntax, Parse};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use syntax::{dice::Dice, function::FunctionCall, number::Number};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expression<R> {
	first: ExpressionItem<R>,
	sequence: Vec<(ExpressionOperator, ExpressionItem<R>)>,
	roll_type: std::marker::PhantomData<R>,
}

impl<R: Debug + Clone> Parse for Expression<R> {
	fn parse(input: &str) -> crate::Result<(&str, Self)> {
		todo!()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpressionItem<R> {
	Number(Number),
	Dice(Dice),
	Parens(Box<Expression<R>>),
	FunctionCall(FunctionCall),
}

impl<R: Debug + Clone> Parse for ExpressionItem<R> {
	fn parse(input: &str) -> crate::Result<(&str, Self)> {
		todo!()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpressionOperator {
	Add,
	Sub,
	Mul,
	Div,
	Floor,
	Ceil,
	Rnd,
}

impl Parse for ExpressionOperator {
	fn parse(input: &str) -> crate::Result<(&str, Self)> {
		todo!()
	}
}
