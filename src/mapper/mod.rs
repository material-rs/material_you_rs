/**
*   this module contains the logic to map roles to tokens, in order to get token's value
* */
use crate::{context::Context, roles::Roles, tokens::token::*};

mod reference;
mod system;

pub struct Mapper;

/**
*   component injects context from provider to mapper
*   role is passed to mapper from component by material's spec
* */
impl Mapper {
	// obatins first high level token from roles
	// and then derives into specific token based on context
	pub fn map(role: Roles, ctx: Context) -> Token {
		role.token().derive(ctx)
	}

	pub fn token_value(token: Token) -> String {
		//TODO: impl token derivation (recursive mapper)
		match token {
			Token::Sys(t) => system::dictionary()[&t].to_string(),
			Token::Ref(t) => reference::dictionary()[&t].to_string(),
			Token::Comp(_) => todo!(),
		}
	}
}
