//TODO: add specific card type styles
use crate::system::{color::ColorRole, shapes::Family};
use yew::prelude::*;

pub mod elevated;
pub use elevated::*;
pub mod filled;
pub use filled::*;
pub mod outlined;
pub use outlined::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum CardKind {
	#[default]
	Elevated,
	Filled,
	Outlined,
}

//Anatomy: 1 -> Container
#[derive(PartialEq, Properties)]
pub struct CardProps {
	pub children: Children,
	#[prop_or_default]
	pub kind: CardKind,
	#[prop_or_default]
	pub bg_role: ColorRole,
	#[prop_or_default]
	pub family: Family,
	#[prop_or_default]
	pub styles: Vec<Classes>,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
	let (family, bg_role, styles) = (
		props.family.clone(),
		props.bg_role.clone(),
		props.styles.clone(),
	);

	match props.kind {
		CardKind::Elevated => html! { <ElevatedCard {family} {bg_role} {styles} >
			{ for props.children.iter() }
		</ElevatedCard> },
		CardKind::Filled => html! { <FilledCard {family} {bg_role} {styles} >
			{ for props.children.iter() }
		</FilledCard> },
		CardKind::Outlined => html! { <OutlinedCard {family} {bg_role} {styles} >
			{ for props.children.iter() }
		</OutlinedCard> },
	}
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
