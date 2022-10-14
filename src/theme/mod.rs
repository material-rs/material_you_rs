use crate::context;
use yew::prelude::*;
/*
* this module contains a implementation of theme provider component
* which provides theming and material's color system access to whole app
*
*   Usage
*   ```rust
*   <ThemeProvider> //provides access to material color system and theming to whole app
*       <App/>
*   </ThemeProvider>
*   ```
* */

//props define context of theme provider
//only children is strictly required
#[derive(PartialEq, Properties)]
pub struct Props {
	pub children: Children,
	#[prop_or_default]
	pub context: context::Context,
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &Props) -> Html {
	html! { <ContextProvider<context::Context> context={ props.context.clone() } >
		{ for props.children.iter() }
	</ContextProvider<context::Context> >}
}
