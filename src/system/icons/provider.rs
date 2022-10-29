use super::{IconKind, IconOpts};
use wasm_bindgen::JsCast;
use web_sys::{HtmlLinkElement, Node};
use yew::prelude::{function_component, html, use_state, Children, ContextProvider, Properties};

/// don't use this!, isntead use ['MaterialProvider']
/// which provide full context, for theming, icons, and typography
///
/// use it only if you are not going to use typographies and color.
///
/// ```rust
/// use material_you::icons::{IconsProvider, IconOpts, IconKind, Icon};
/// use yew::{function_component, html};
///
/// #[function_component(MyApp)]
/// fn my_app() -> Html {
///     let default_opts = IconOpts::new(1.0, 500, 50, 36);
///
///     let default_family = IconKind::Sharp;
///
///     html! { <IconsProvider default_opts={default_opts} default_family={default_family} >
///         <Icon icon={String::from("home")} />
///     </IconsProvider> }
/// }
/// ```
#[derive(PartialEq, Properties)]
pub struct Props {
	pub children: Children,
	#[prop_or_default]
	pub default_family: IconKind,
	#[prop_or_default]
	pub default_opts: IconOpts,
}

/// don't use this!, isntead use ['MaterialProvider']
/// which provide full context, for theming, icons, and typography
///
/// use it only if you are not going to use typographies and color.
///
/// ```rust
/// use material_you::icons::{IconsProvider, Icon};
/// use yew::{function_component, html};
///
/// #[function_component(MyApp)]
/// fn my_app() -> Html {
///     html! { <IconsProvider>
///         <Icon icon={String::from("home")} />
///     </IconsProvider> }
/// }
/// ```
#[function_component(IconsProvider)]
pub fn icons_provider(props: &Props) -> Html {
	icons_url();

	let (family, opts) = (
		use_state(|| props.default_family.clone()),
		use_state(|| props.default_opts.clone()),
	);

	let (family, opts) = (&*family, &*opts);

	html! { <ContextProvider<IconKind> context={ family.clone() } >
		<ContextProvider<IconOpts> context={ opts.clone() } >
			{ for props.children.iter() }
		</ContextProvider<IconOpts> >
	</ContextProvider<IconKind> >}
}

fn icons_url() {
	let doc = web_sys::window().unwrap().document().unwrap();
	let head = doc.head().unwrap();

	let families = ["Outlined", "Rounded", "Sharp"];
	let api_url = "https://fonts.googleapis.com";

	for f in families {
		let href = format!(
			"{}/css2?family=Material+Symbols+{}:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200",
			api_url, f
		);

		let link_elem = doc
			.create_element("link")
			.unwrap()
			.dyn_into::<HtmlLinkElement>()
			.unwrap();

		link_elem.set_rel("stylesheet");
		link_elem.set_href(&href);

		head.append_child(&Node::from(link_elem)).unwrap();
	}
}
