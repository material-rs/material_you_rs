use yew::prelude::*;
use {crate::color::Color, crate::context, crate::css, crate::roles::Roles};

//Anatomy: 1 -> Container
#[derive(PartialEq, Properties)]
pub struct CardProps {
	pub children: Children,
	#[prop_or_default]
	pub elevated: bool,
	#[prop_or_default]
	pub filled: bool,
	#[prop_or_default]
	pub outlined: bool,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
	let context = use_context::<context::Context>().unwrap();

	let bg = css::background("div", &Color::of(Roles::Surface, context));

	let ctn = css::new_style(
		"div",
		r#"
			border-radius: 12px;
			padding-left: 16px;
			padding-right: 16px;
			margin: 8px;
			text-align: start;
			display: inline-block;
		"#,
	);
	html! { <div class={ vec![ctn, bg]} >
		{ for props.children.iter() }
	</div> }
}

//Anatomy: 2 -> Headline
#[derive(PartialEq, Properties)]
pub struct CardHeadlineProps {
	pub children: Children,
}

#[function_component(CardHeadline)]
pub fn headline(props: &CardHeadlineProps) -> Html {
	html! {<div>
		{ for props.children.iter() }
	</div>}
}

//Anatomy: 3 -> Subhead
#[derive(PartialEq, Properties)]
pub struct CardSubheadProps {
	pub children: Children,
}

#[function_component(CardSubhead)]
pub fn subhead(props: &CardSubheadProps) -> Html {
	html! {<div>
		{ for props.children.iter() }
	</div>}
}

//Anatomy: 4 -> SupportingText
#[derive(PartialEq, Properties)]
pub struct CardBodyProps {
	pub children: Children,
}

#[function_component(CardBody)]
pub fn body(props: &CardBodyProps) -> Html {
	html! {<div>
		{ for props.children.iter() }
	</div>}
}

//Anatomy: 5 -> Image
#[derive(PartialEq, Properties)]
pub struct CardMediaProps {
	pub children: Children,
}

#[function_component(CardMedia)]
pub fn image(props: &CardMediaProps) -> Html {
	html! {<div>
		{ for props.children.iter() }
	</div>}
}

//Anatomy: 6 -> Button
#[derive(PartialEq, Properties)]
pub struct CardActionsProps {
	pub children: Children,
}

#[function_component(CardActions)]
pub fn footer(props: &CardActionsProps) -> Html {
	html! {<div>
		{ for props.children.iter() }
	</div>}
}
