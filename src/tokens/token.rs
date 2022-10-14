use super::{component::MdComp, reference::MdRef, system::MdSys};
use crate::context::Context;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
	Sys(MdSys),
	Ref(MdRef),
	Comp(MdComp),
}

impl Token {
	pub fn derive(self, ctx: Context) -> Self {
		match self {
			Token::Sys(t) => {
				let pattern = format!("{}{}", t, ctx.theme());
				println!("PATTERN:  {}", pattern);
				Token::Sys(MdSys::from_str(&pattern).unwrap())
			}
			_ => todo!(),
		}
	}
}

//TODO: NOTE: this should have their own mapper
//some token can point to another tokens
//but value() should return the final value
//for token
/*#[derive(Debug, PartialEq)]
pub struct Token {
	token: TokenKind,
}*/

//"md_sys_color_surface" => "#fffbfe", // Surface
/*impl From<String> for Token {
	fn from(raw: String) -> Self {
		let mut v: Vec<&str> = raw.split("_").collect();

		let kind = v.remove(0);

		let pattern = v.join("_").to_case(Case::UpperCamel);

		let t: TokenKind;

		println!("kind: {}", kind);

		if kind == "sys" {
			t = TokenKind::Sys(MdSys::from_str(&pattern).unwrap());
		} else if kind == "ref" {
			t = TokenKind::Ref(MdRef::from_str(&pattern).unwrap());
		} else {
			t = TokenKind::Comp(MdComp::from_str(&pattern).unwrap());
		}

		Self { token: t }
	}
}

impl<'a> From<&'a str> for Token {
	fn from(raw: &'a str) -> Self {
		Self::from(raw.to_owned())
	}
}

impl<T> From<&T> for Token
where
	T: std::fmt::Display,
{
	fn from(token: &T) -> Self {
		Self::from(token.to_string().to_case(Case::Snake))
	}
}*/
