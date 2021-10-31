use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;

pub trait Token<Re, Err, Ctx = ()> {
	fn convert(self, ctx: &Ctx) -> Result<Re, Err>;
}
pub struct Climber<Op, To: Token<Re, Err, Ctx> + Clone, Re, Err, Ctx = ()> {
	pub rules: fn(&Op, &Ctx) -> (usize, Assoc),
	/// Function to handle the result of an operator between two tokens.
	///
	/// Arguments are:
	/// - Left-hand side token
	/// - Operator
	/// - Right-hand side token
	pub handler: fn(To, Op, To, &Ctx) -> Result<To, Err>,
	p_rule_value: PhantomData<Op>,
	p_token: PhantomData<To>,
	p_result: PhantomData<Re>,
	p_ctx: PhantomData<Ctx>,
}

impl<Op: Clone, To: Token<Re, Err, Ctx> + Clone, Re, Err, Ctx>
	Climber<Op, To, Re, Err, Ctx>
{
	pub fn new(
		rules: fn(&Op, &Ctx) -> (usize, Assoc),
		handler: fn(To, Op, To, &Ctx) -> Result<To, Err>,
	) -> Self {
		Self {
			rules,
			handler,
			p_rule_value: PhantomData,
			p_token: PhantomData,
			p_result: PhantomData,
			p_ctx: PhantomData,
		}
	}
	pub fn process(
		&self,
		expr: &Expression<Op, To>,
		ctx: &Ctx,
	) -> Result<Re, Err> {
		let mut primary = expr.first_token.clone().convert(ctx)?;
		let lhs = expr.first_token.clone();
		let mut tokens = expr.pairs.iter().peekable();
		self
			.process_rec(
				lhs, //
				0,
				&mut primary,
				&mut tokens,
				ctx,
			)?
			.convert(ctx)
	}

	fn process_rec(
		&self,
		mut lhs: To,
		min_prec: usize,
		primary: &mut Re,
		tokens: &mut std::iter::Peekable<std::slice::Iter<(Op, To)>>,
		ctx: &Ctx,
	) -> Result<To, Err> {
		while let Some((rule, _)) = tokens.peek() {
			let (prec, _) = (self.rules)(rule, ctx);
			if prec >= min_prec {
				let (_, rhs_ref) = tokens.next().unwrap();
				let mut rhs = rhs_ref.clone();

				while let Some((peek_rule, _)) = tokens.peek() {
					let (peek_prec, peek_assoc) = (self.rules)(peek_rule, ctx);
					if peek_prec > prec || peek_assoc == Assoc::Right && peek_prec == prec
					{
						rhs = self.process_rec(rhs, peek_prec, primary, tokens, ctx)?;
					} else {
						break;
					}
				}
				lhs = (self.handler)(lhs, rule.clone(), rhs, ctx)?;
			} else {
				break;
			}
		}
		Ok(lhs)
	}
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Assoc {
	Left,
	Right,
}

/// A faithful, always-valid representation of an expression.
///
/// It's impossible to throw an error due to the order of `token, operator, token` not being respected.
#[derive(Debug, Clone)]
pub struct Expression<Op: Clone, To: Clone> {
	pub first_token: To,
	pub pairs: Vec<(Op, To)>,
}

impl<Op: Clone, To: Clone> Expression<Op, To> {
	/// ```ignore
	/// // 5 * 6 + 3 / 2 ^ 4
	/// let expression = Expression::new(
	/// 	5.0f64,
	/// 	vec![
	/// 		(Op::Mul, 6.0),
	/// 		(Op::Add, 3.0),
	/// 		(Op::Div, 2.0),
	/// 		(Op::Exp, 4.0)
	/// 	]
	/// );
	/// ```
	pub fn new(first_token: To, pairs: Vec<(Op, To)>) -> Self {
		Self { first_token, pairs }
	}
}

impl<Op: Copy + fmt::Display, To: Clone + fmt::Display> fmt::Display
	for Expression<Op, To>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut s = format!("{}", self.first_token);
		for (o, t) in &self.pairs {
			s = format!("{} {} {}", s, o, t);
		}
		write!(f, "{}", s)
	}
}

impl<Op: Copy + PartialEq, To: Clone + PartialEq> PartialEq
	for Expression<Op, To>
{
	fn eq(&self, other: &Self) -> bool {
		self.first_token == other.first_token && self.pairs == other.pairs
	}
}

impl<Op: Copy + Eq, To: Clone + Eq> Eq for Expression<Op, To> {}

// #[cfg(test)]
// mod test {
// 	use super::*;

// 	fn c(
// 		expression: &Expression<MathOperator, MathToken>,
// 		ctx: &f32,
// 	) -> Result<f32, &'static str> {
// 		use MathOperator::*;
// 		let climber = Climber::new(
// 			|op, _| match op {
// 				Add | Sub => (0, Assoc::Left),
// 				Mul | Div => (1, Assoc::Left),
// 			},
// 			|lhs: MathToken, op: MathOperator, rhs: MathToken, ctx: &f32| {
// 				let lhs: f32 = lhs.convert(ctx)?;
// 				let rhs: f32 = rhs.convert(ctx)?;
// 				Ok(match op {
// 					Add => MathToken::Num(lhs + rhs),
// 					Sub => MathToken::Num(lhs - rhs),
// 					Mul => MathToken::Num(lhs * rhs),
// 					Div => MathToken::Num(lhs / rhs),
// 				})
// 			},
// 		);
// 		climber.process(&expression, ctx)
// 	}

// 	#[derive(Hash, Eq, PartialEq, Copy, Clone)]
// 	pub enum MathOperator {
// 		Add,
// 		Sub,
// 		Mul,
// 		Div,
// 	}

// 	#[derive(Clone)]
// 	pub enum MathToken {
// 		Paren(Box<Expression<MathOperator, MathToken>>),
// 		Num(f32),
// 		X,
// 	}

// 	impl Token<f32, &'static str, f32> for MathToken {
// 		fn convert(self, ctx: &f32) -> Result<f32, &'static str> {
// 			Ok(match self {
// 				MathToken::Paren(expr) => c(expr.as_ref(), ctx)?,
// 				MathToken::Num(n) => n,
// 				MathToken::X => *ctx,
// 			})
// 		}
// 	}

// 	#[test]
// 	fn process() {
// 		let res = c(
// 			&Expression::new(
// 				MathToken::Num(7.0),
// 				vec![(MathOperator::Add, MathToken::X)],
// 			),
// 			&8.0,
// 		)
// 		.unwrap();

// 		assert_eq!(res, 15.0);
// 	}
// 	#[test]
// 	fn proces_complex() {
// 		use MathOperator::*;
// 		use MathToken::*;
// 		let res = c(
// 			&Expression::new(
// 				Num(10.0),
// 				vec![(Add, Num(5.0)), (Mul, Num(3.0)), (Add, Num(1.0))],
// 			),
// 			&8.0,
// 		)
// 		.unwrap();
// 		println!("{}", res);
// 	}
// }
