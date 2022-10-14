// This module contains the material's system color implementation
use crate::{context::Context, mapper::Mapper, roles::Roles};

#[allow(dead_code)]
pub struct Color;

impl Color {
	pub fn of(role: Roles, context: Context) -> String {
		let t = Mapper::map(role, context);
		Mapper::token_value(t)
	}
}
