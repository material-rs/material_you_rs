use crate::{
	color::{Color, ColorRole},
	context,
	context::Theme,
	css,
};
use color_utilities::utils::color::ARGB;
use std::fmt::Display;
use yew::prelude::{classes, function_component, html, use_context, Properties};

pub mod provider;
pub use provider::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum IconKind {
	#[default]
	Outlined,
	Rounded,
	Sharp,
}

impl Display for IconKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct IconOpts {
	/// 0.0 - 1.0
	fill: f32,
	/// 100 - 700
	weight: u16,
	/// -25 - 200
	grade: i16,
	/// 20 - 48
	optical_size: u8,
}

impl IconOpts {
	pub fn new(fill: f32, weight: u16, grade: i16, optical_size: u8) -> Self {
		Self {
			fill,
			weight,
			grade,
			optical_size,
		}
	}
}

impl Default for IconOpts {
	fn default() -> Self {
		Self {
			fill: 0.,
			weight: 400,
			grade: 0,
			optical_size: 24,
		}
	}
}

/// ```rust
/// use material_you::{color::ColorRole, icons::{IconsProvider, IconOpts, IconKind, Icon}};
/// use yew::{function_component, html};
///
/// #[function_component(MyApp)]
/// fn my_app() -> Html {
///     //let opts = IconOpts::new(1.0, 600, 100, 48);
///     //let family = IconKind::Rounded;
///     //let color = ColorRole::Tertiary;
///
///     html! { <IconsProvider
///         //opts={opts}
///         //family={family}
///         //inactive={true}
///         //color={color}
///     >
///         <Icon icon={String::from("home")} />
///     </IconsProvider> }
/// }
/// ```
#[derive(PartialEq, Properties)]
pub struct Props {
	pub icon: String,
	#[prop_or_default]
	pub family: Option<IconKind>,
	#[prop_or_default]
	pub opts: Option<IconOpts>,
	#[prop_or_default]
	pub inactive: bool,
	#[prop_or_default]
	pub color: Option<ColorRole>,
}

/// usage
/// ```
/// use yew::prelude::*;
/// use material_you::icons::*;
///
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     html!{
///         <Icon icon={String::from("home")} />
///     }
/// }
/// ```
#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
	let context = use_context::<context::Context>().unwrap();

	let family = if let Some(family) = props.family.clone() {
		family
	} else {
		use_context::<IconKind>().unwrap()
	};

	let opts = if let Some(opts) = props.opts.clone() {
		opts
	} else {
		use_context::<IconOpts>().unwrap()
	};

	let color = color(props.inactive, &context, &props.color);

	let color = css::color("span", color);

	let icon_family = format!("material-symbols-{}", family.to_string().to_lowercase());

	let icon_style = {
		let IconOpts {
			fill,
			weight,
			grade,
			optical_size,
		} = opts;

		let style = format!(
			r#".{} {{
                font-variation-settings:
                    'FILL' {},
                    'wght' {},
                    'GRAD' {},
                    'opsz' {};

                font-size: {}px;
            }}
            "#,
			&icon_family, fill, weight, grade, optical_size, optical_size
		);

		css::new_style("div", &style)
	};

	html! { <div class={icon_style}>
		<span class={classes!(icon_family, color)} >{&props.icon}</span>
	</div>}
}

// returns ['ARGB'] for icon
fn color(inactive: bool, context: &context::Context, color: &Option<ColorRole>) -> ARGB {
	let theme = context.theme();

	let [a, r, g, b] = match theme {
		Theme::Dark => {
			let a = if inactive { 67 } else { 138 };

			let [_, r, g, b] = if let Some(color) = color {
				Color::of(color, context)
			} else {
				[0, 255, 255, 255]
			};

			[a, r, g, b]
		}
		Theme::Light => {
			let a = if inactive { 77 } else { 255 };

			let [_, r, g, b] = if let Some(color) = color {
				Color::of(color, context)
			} else {
				[0, 0, 0, 0]
			};

			[a, r, g, b]
		}
	};

	[a, r, g, b]
}
