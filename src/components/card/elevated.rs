use crate::{
	color::{Color, ColorContext, ColorRole},
	css,
	shapes::Family,
	system::{
		elevation::{Elevation, ElevationKind},
		shapes::ShapeMedium,
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
	#[prop_or(Elevation::shadow(1))]
	pub elevation: Elevation,
	#[prop_or_default]
	pub styles: Vec<Classes>,
}

#[function_component(ElevatedCard)]
pub fn elevated_card(props: &Props) -> Html {
	let mut context = use_context::<ColorContext>().unwrap();

	let (family, background, elevation, styles) = (
		props.family.clone(),
		props.background.clone(),
		props.elevation.clone(),
		props.styles.clone(),
	);

	let style = super::card_style();

	let bg = css::background("div", Color::of(&background, &context));

	let elevation_style = match elevation.kind() {
		ElevationKind::Shadow => css::elevation::shadow("div", &mut context, elevation.level()),
		ElevationKind::Scrim => css::elevation::scrim("div", &context, elevation.level()),
		ElevationKind::Tone => {
			css::elevation::tone("div", &mut context, &background, elevation.level())
		}
	};

	let styles = [styles, vec![style, bg, elevation_style]].concat();

	html! { <ShapeMedium {family} {background} {elevation} {styles} >
		{ for props.children.iter() }
	</ShapeMedium>}
}
