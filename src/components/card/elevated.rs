use crate::{
	color::{Color, ColorContext, ColorRole},
	css,
	shapes::Family,
	system::shapes::ShapeMedium,
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

#[function_component(ElevatedCard)]
pub fn elevated_card(props: &Props) -> Html {
	let context = use_context::<ColorContext>().unwrap();

	let (family, bg_role, styles) = (
		props.family.clone(),
		props.bg_role.clone(),
		props.styles.clone(),
	);

	let style = css::new_style(
		"div",
		r#"
			text-align: start;
			display: inline-block;
			padding-left: 16px;
			padding-right: 16px;
			margin-left: 8px;
			margin-right: 8px;
		"#,
	);

	//let bg_child = css::background("div", ColorRole::Primary);

	let [a, r, g, b] = Color::of(&ColorRole::Shadow, &context);

	let a = a as f32 / 255.0;

	let shadow = css::new_style(
		"div",
		&format!(
			r#"
        box-shadow: 1px 1px rgba({},{},{},{});
    "#,
			r, g, b, a
		),
	);

	let styles = [styles, vec![style, shadow]].concat();

	html! { <ShapeMedium {family} {bg_role} {styles} >
		{ for props.children.iter() }
	</ShapeMedium>}
}
