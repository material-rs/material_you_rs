mod color;
pub mod components;
pub mod context;
mod css;
//mod dictionary;
pub mod mapper;
pub mod roles;
pub mod theme;
pub mod tokens;

pub mod prelude {
	pub use crate::components::card::Card;
	pub use crate::components::*;
	pub use crate::theme::ThemeProvider;
}

#[macro_export]
macro_rules! hashmap {
	( $( $k:expr => $v:expr),* $(,)* ) => {
		{
			let mut temp_hashmap = HashMap::new();
			$(
				temp_hashmap.insert($k, $v);
			)*
			temp_hashmap
		}
	};
}

#[macro_export]
macro_rules! display {
	( $ident:ident ) => {
		impl std::fmt::Display for $ident {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				write!(f, "{:?}", self)
			}
		}
	};
}
