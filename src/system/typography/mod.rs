use crate::css;
use yew::prelude::{classes, function_component, html, use_context, Children, Classes, Properties};

mod provider;
pub use provider::*;
mod typescale;
pub use typescale::*;

/// usage
/// ```rust
/// use yew::{function_component, html, Html};
/// use material_you::typography::{Typography, TypescaleModifier, TypescaleRoles};
///
/// pub fn my_component() -> Html {
///     //let role = TypescaleRoles::Headline;
///
///     html! { <div>
///         <Typography
///             //role={role}
///             //modifier={TypescaleModifier::Large}
///         ><span>{"Hi I'm a custom typography"}</span>
///         </Typography>
///     </div> }
/// }
/// ```
#[derive(PartialEq, Properties)]
pub struct Props {
	pub children: Children,
	#[prop_or_default]
	pub role: TypescaleRoles,
	#[prop_or_default]
	pub modifier: TypescaleModifier,
	#[prop_or_default]
	pub styles: Vec<Classes>,
}

/// usage
/// ```rust
/// use yew::{function_component, html, Html};
/// use material_you::typography::{Typography};
///
/// pub fn my_component() -> Html {
///     html! { <div>
///         <Typography><span>{"Hi I'm a typography"}</span></Typography>
///     </div> }
/// }
/// ```
#[function_component(Typography)]
pub fn typography(props: &Props) -> Html {
	let core_typescale = use_context::<CoreTypescale>().unwrap();

	let typescale = core_typescale.typescale_of(&props.role, &props.modifier);

	let styles = css::fonts_style("div", typescale);

	html! { <div class={classes!(styles)} >
		{ for props.children.iter() }
	</div> }
}
