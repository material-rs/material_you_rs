pub mod components;
pub mod context;
pub mod css;
pub(crate) mod system;
pub mod theme;

pub use system::color;
pub use system::shapes;

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
