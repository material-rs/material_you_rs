use super::typescale::CoreTypescale;
use yew::prelude::{function_component, html, use_state, Children, ContextProvider, Properties};

//TODO: write a way in ['CoreTypescale'] to set custom params
#[derive(PartialEq, Properties)]
pub struct Props {
	pub children: Children,
	#[prop_or_default]
	pub typescale: Option<CoreTypescale>,
}

/// ```rust
/// use material_you::typography::{TypographyProvider, Typography};
/// use yew::{function_component, html};
///
/// #[function_component(MyApp)]
/// fn my_app() -> Html {
///     html! { <TypographyProvider>
///         <Typography>{"Hello! you're in material rs now!"}</Typography>
///     </TypographyProvider> }
/// }
/// ```
#[function_component(TypographyProvider)]
pub fn typography_provider(props: &Props) -> Html {
	let typescale = use_state(|| props.typescale.clone());

	let typescale = if let Some(typescale) = &*typescale {
		typescale.clone()
	} else {
		CoreTypescale::default()
	};

	html! { <ContextProvider<CoreTypescale> context={ typescale } >
		{ for props.children.iter() }
	</ContextProvider<CoreTypescale>> }
}
