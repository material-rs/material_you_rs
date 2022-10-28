use crate::{color::ColorRole, css, shapes::Family, system::shapes::ShapeMedium};
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

#[function_component(OutlinedCard)]
pub fn outlined_card(props: &Props) -> Html {
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

	let styles = [styles, vec![style]].concat();

	html! { <ShapeMedium {family} {bg_role} {styles} >
		{ for props.children.iter() }
	</ShapeMedium>}
}
