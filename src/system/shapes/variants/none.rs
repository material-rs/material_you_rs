use crate::shapes::Family;
use crate::system::color::{Color, ColorRole};
use crate::{context, css::*};
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

#[function_component(ShapeNone)]
pub fn shape_none(props: &Props) -> Html {
	let context = use_context::<context::Context>().unwrap();

	let border = border("div", &props.family, [0.0; 4]);

	let bg = background("div", Color::of(&props.bg_role, &context));

	let mut class: Vec<_> = vec![bg, border];
	class.extend(props.styles.clone());

	html! {<div {class} >
		{ for props.children.iter() }
	</div>}
}
