use color_utilities::{palettes::CorePalette, scheme::Scheme, utils::color::ARGB};
use yew::prelude::{
	function_component, html, use_state, Callback, Children, ContextProvider, Properties,
};

/// don't use this!, isntead use ['MaterialProvider']
/// which provide full context, for theming, icons, and typography
///
/// use it only if you are not going to use typographies and icons.
///
/// ```rust
/// use material_you::color::{ColorContext, Theme, ColorProvider};
/// use yew::{function_component, html};
///
/// #[function_component(MyApp)]
/// fn my_app() -> Html {
///     let my_custom_context = {
///         let my_custom_color = [255, 125, 125, 125];
///         let theme = Theme::Dark;
///
///         ColorContext::new(theme, my_custom_color)
///     };
///
///     html! { <ColorProvider context={my_custom_context}>
///         <p>{"My app with custom context provider"}</p>
///     </ColorProvider>}
/// }
/// ```
#[derive(PartialEq, Properties)]
pub struct Props {
	pub children: Children,
	#[prop_or_default]
	pub context: ColorContext,
}

/// toggle theme, dark/light
pub type ThemeDispatcher = Callback<()>;

/// don't use this!, isntead use ['MaterialProvider']
/// which provide full context, for theming, icons, and typography
///
/// use it only if you are not going to use typographies and icons.
///
/// Usage
/// ```rust
/// use material_you::color::ColorProvider;
/// use yew::prelude::{function_component, html};
///
/// #[function_component(MyApp)]
/// fn my_app() -> Html {
///     html! { <ColorProvider>
///         <div>
///             <h1>{"Hello App"}</h1>
///             <p>{"`ColorProvider` component should be at top of whole app"}</p>
///         </div>
///     </ColorProvider> }
/// }
/// ```
#[function_component(ColorProvider)]
pub fn theme_provider(props: &Props) -> Html {
	let context = use_state(|| props.context.clone());

	let onclick = {
		let context = context.clone();
		Callback::from(move |_: _| context.set(context.change_theme()))
	};

	let context = &*context;

	html! { <ContextProvider<ColorContext> context={ context.clone() } >
		<ContextProvider<ThemeDispatcher> context={ onclick } >
				{ for props.children.iter() }
		</ContextProvider<ThemeDispatcher> >
	</ContextProvider<ColorContext> >}
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Theme {
	#[default]
	Light,
	Dark,
}

impl Theme {
	pub fn change(&self) -> Theme {
		match self {
			Theme::Dark => Theme::Light,
			Theme::Light => Theme::Dark,
		}
	}
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ColorContext {
	color: ARGB,
	palette: CorePalette,
	scheme: Scheme,
	theme: Theme,
}

impl Default for ColorContext {
	fn default() -> Self {
		Self {
			color: [255, 0, 0, 255],
			palette: CorePalette::default(),
			scheme: Scheme::default(),
			theme: Theme::default(),
		}
	}
}

impl ColorContext {
	pub fn new(theme: Theme, color: ARGB) -> Self {
		let scheme = match theme {
			Theme::Light => Scheme::light(color),
			Theme::Dark => Scheme::dark(color),
		};

		Self {
			color,
			palette: CorePalette::of(color),
			scheme,
			theme,
		}
	}
	pub fn theme(&self) -> &Theme {
		&self.theme
	}
	pub fn change_theme(&self) -> ColorContext {
		ColorContext::new(self.theme.change(), self.color)
	}

	pub fn scheme(&self) -> Scheme {
		match self.theme {
			Theme::Light => Scheme::light(self.color),
			Theme::Dark => Scheme::dark(self.color),
		}
	}

	pub fn is_light_theme(&self) -> bool {
		matches!(self.theme, Theme::Light)
	}

	pub fn palette(&mut self) -> &mut CorePalette {
		&mut self.palette
	}
}
