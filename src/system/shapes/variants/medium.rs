use crate::{
	css::*,
	shapes::Family,
	system::color::{Color, ColorContext, ColorRole},
};
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
	pub children: Children,
	#[prop_or_default]
	pub family: Family,
	#[prop_or_default]
	pub bg_role: ColorRole,
	#[prop_or_default]
	pub styles: Vec<Classes>,
}

#[function_component(ShapeMedium)]
pub fn shape_medium(props: &Props) -> Html {
	let context = use_context::<ColorContext>().unwrap();

	let border = border("div", &props.family, [12.0; 4]);

	let bg = background("div", Color::of(&props.bg_role, &context));

	let mut class: Vec<_> = vec![bg, border];
	class.extend(props.styles.clone());

	html! {<div {class} >
		{ for props.children.iter() }
	</div>}
}
