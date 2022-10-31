use crate::{
	css,
	shapes::Family,
	system::{
		color::{Color, ColorContext, ColorRole},
		elevation::{Elevation, ElevationKind},
	},
};
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
	pub children: Children,
	#[prop_or_default]
	pub family: Family,
	#[prop_or(ColorRole::Surface)]
	pub background: ColorRole,
	#[prop_or(Elevation::tone(0))]
	pub elevation: Elevation,
	#[prop_or_default]
	pub styles: Vec<Classes>,
}

#[function_component(ShapeFull)]
pub fn shape_full(props: &Props) -> Html {
	let mut context = use_context::<ColorContext>().unwrap();

	let (family, background, elevation, styles) = (
		props.family.clone(),
		props.background.clone(),
		props.elevation.clone(),
		props.styles.clone(),
	);

	let border = css::border("div", &family, [100.0; 4]);

	let bg = css::background("div", Color::of(&background, &context));

	let elevation = match elevation.kind() {
		ElevationKind::Shadow => css::elevation::shadow("div", &mut context, elevation.level()),
		ElevationKind::Tone => {
			css::elevation::tone("div", &mut context, &background, elevation.level())
		}
		ElevationKind::Scrim => css::elevation::scrim("div", &context, elevation.level()),
	};

	let mut class: Vec<_> = vec![bg, border, elevation];
	class.extend(styles);

	html! {<div {class} >
		{ for props.children.iter() }
	</div>}
}
