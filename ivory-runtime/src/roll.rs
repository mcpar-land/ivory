use crate::{
	error::RuntimeError,
	runtime::{Runtime, RuntimeContext},
	Result,
};
use rand::Rng;

pub struct Roll {
	count: u32,
	sides: u32,
	rerolls: Vec<i32>,
}

pub struct SingleRoll {
	pub val: u32,
	pub rerolls: Vec<u32>,
	pub explodes: Vec<u32>,
	pub kept: Option<bool>,
	pub success: Option<bool>,
}

impl Roll {}
