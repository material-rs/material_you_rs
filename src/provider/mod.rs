use crate::system::{
	color::{ColorContext, ColorProvider},
	icons::{IconKind, IconOpts, IconsProvider},
	typography::{CoreTypescale, TypographyProvider},
};
use yew::prelude::{function_component, html, use_state, Children, Properties};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TypographyContext {
	typescale: CoreTypescale,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct IconsContext {
	opts: IconOpts,
	family: IconKind,
}

#[derive(Clone, Debug, Default, PartialEq, Properties)]
pub struct MaterialContext {
	color: ColorContext,
	icons: IconsContext,
	typography: TypographyContext,
}

impl MaterialContext {
	pub fn color(&self) -> &ColorContext {
		&self.color
	}

	pub fn icon(&self) -> &IconsContext {
		&self.icons
	}

	pub fn typography(&self) -> &TypographyContext {
		&self.typography
	}
}

#[derive(PartialEq, Properties)]
pub struct Props {
	pub children: Children,
	#[prop_or_default]
	pub context: MaterialContext,
}

/// this module contains a implementation of theme provider component
/// which provides theming and material's color system access to whole app
///
/// Usage
/// ```rust
/// use material_you::provider::MaterialProvider;
/// use yew::prelude::{function_component, html};
///
/// #[function_component(MyApp)]
/// fn my_app() -> Html {
///     html! { <MaterialProvider>
///         <div>
///             <h1>{"Hello App"}</h1>
///             <p>{"`MaterialProvider` component should be at top of whole app"}</p>
///         </div>
///     </MaterialProvider> }
/// }
/// ```
#[function_component(MaterialProvider)]
pub fn material_provider(props: &Props) -> Html {
	let context = use_state(|| props.context.clone());

	let MaterialContext {
		color,
		icons,
		typography,
	} = &*context;

	html! { <ColorProvider context={ color.clone() } >
		<IconsProvider default_family={icons.family.clone()}  default_opts={icons.opts.clone()} >
			<TypographyProvider typescale={typography.typescale.clone()} >
				{ for props.children.iter() }
			</TypographyProvider>
		</IconsProvider>
	</ColorProvider> }
}
