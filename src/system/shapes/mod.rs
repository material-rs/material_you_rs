use crate::system::color::ColorRole;
use yew::prelude::*;

mod variants;
pub use variants::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
	pub children: Children,
	#[prop_or_default]
	pub scale: ShapeScale,
	#[prop_or_default]
	pub family: Family,
	#[prop_or_default]
	pub bg_role: ColorRole,
	#[prop_or_default]
	// custom styles passed from top level components
	pub styles: Vec<Classes>,
}

///
/// usage
///
/// ```rust
///     use yew::prelude::*;
///     use material_you::{
///         shapes::{Shape, ShapeScale, Family},
///         color::ColorRole,
///         css,
///     };
///
///     //let scale = ShapeScale::Medium;
///     //let family = Family::Rounded;
///     //let bg_role = ColorRole::Surface;
///
///     //let custom_css = css::new_style("div", r#"margin: 2px;"#);
///     //let styles = vec![custom_css];
///     
///     #[function_component(Html)]
///     pub fn my_compnent() -> Html {
///         html! { <Shape
///             //scale={scale}
///             //family={family}
///             //bg_role={bg_role}
///             //styles={styles}
///         >
///             <span>{"my shape content"}</span>
///         </Shape> }
///     }
/// ```
#[function_component(Shape)]
pub fn shape(props: &Props) -> Html {
	let (family, bg_role, styles) = (
		props.family.clone(),
		props.bg_role.clone(),
		props.styles.clone(),
	);

	match props.scale {
		ShapeScale::None => html! { <ShapeNone {bg_role} {family} {styles}>
			{ for props.children.iter() }
		</ShapeNone> },

		ShapeScale::ExtraSmall => html! { <ShapeExtraSmall {bg_role} {family} {styles}>
			{ for props.children.iter() }
		</ShapeExtraSmall> },

		ShapeScale::Small => html! { <ShapeSmall {bg_role} {family} {styles}>
			{ for props.children.iter() }
		</ShapeSmall> },

		ShapeScale::Medium => html! { <ShapeMedium {bg_role} {family} {styles}>
			{ for props.children.iter() }
		</ShapeMedium> },

		ShapeScale::Large => html! { <ShapeLarge {bg_role} {family} {styles}>
			{ for props.children.iter() }
		</ShapeLarge> },

		ShapeScale::ExtraLarge => html! { <ShapeExtraLarge {bg_role} {family} {styles}>
			{ for props.children.iter() }
		</ShapeExtraLarge> },

		ShapeScale::Full => html! { <ShapeFull {bg_role} {family} {styles}>
			{ for props.children.iter() }
		</ShapeFull> },
	}
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum ShapeScale {
	None,
	ExtraSmall,
	Small,
	#[default]
	Medium,
	Large,
	ExtraLarge,
	Full,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Family {
	#[default]
	Rounded,
	Cuted,
	Circular,
}
impl std::fmt::Display for Family {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}
