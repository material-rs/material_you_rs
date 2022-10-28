use crate::context;
use yew::prelude::{
	function_component, html, use_state, Callback, Children, ContextProvider, Properties,
};

/// you can use default values for theme provider
/// or also you can pass your custom values for
/// theme, and base color
///
/// ```rust
/// use material_you::{context::{Context, Theme}, theme::ThemeProvider};
/// use yew::{function_component, html};
///
/// #[function_component(MyApp)]
/// fn my_app() -> Html {
///     let my_custom_context = {
///         let my_custom_color = [255, 125, 125, 125];
///         let theme = Theme::Dark;
///
///         Context::new(theme, my_custom_color)
///     };
///
///     html! { <ThemeProvider context={my_custom_context}>
///         <p>{"My app with custom context provider"}</p>
///     </ThemeProvider>}
/// }
/// ```
#[derive(PartialEq, Properties)]
pub struct Props {
	pub children: Children,
	#[prop_or_default]
	pub context: context::Context,
}

/// toggle theme, dark/light
pub type ThemeDispatcher = Callback<()>;

/// this module contains a implementation of theme provider component
/// which provides theming and material's color system access to whole app
///
/// Usage
/// ```rust
/// use material_you::theme::ThemeProvider;
/// use yew::prelude::{function_component, html};
///
/// #[function_component(MyApp)]
/// fn my_app() -> Html {
///     html! { <ThemeProvider>
///         <div>
///             <h1>{"Hello App"}</h1>
///             <p>{"`ThemeProvider` component should be at top of whole app"}</p>
///         </div>
///     </ThemeProvider> }
/// }
/// ```
#[function_component(ThemeProvider)]
pub fn theme_provider(props: &Props) -> Html {
	let context = use_state(|| props.context.clone());

	let onclick = {
		let context = context.clone();
		Callback::from(move |_: _| context.set(context.change_theme()))
	};

	let context = &*context;

	html! { <ContextProvider<context::Context> context={ context.clone() } >
		<ContextProvider<ThemeDispatcher> context={ onclick } >
				{ for props.children.iter() }
		</ContextProvider<ThemeDispatcher> >
	</ContextProvider<context::Context> >}
}
