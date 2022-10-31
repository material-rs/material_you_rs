pub mod components;
pub mod css;
pub mod provider;
pub(crate) mod system;

pub use system::color;
pub use system::elevation;
pub use system::icons;
pub use system::shapes;
pub use system::typography;

pub mod prelude {
	pub use crate::components::card::Card;
	pub use crate::components::*;
	pub use crate::provider;
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
