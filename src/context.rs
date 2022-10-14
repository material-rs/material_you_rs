use yew::Properties;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Theme {
	#[default]
	Light,
	Dark,
}

crate::display!(Theme);

#[derive(Clone, Debug, Default, Eq, PartialEq, Properties)]
pub struct Context {
	theme: Theme,
}

impl Context {
	pub fn new(theme: Theme) -> Self {
		Self { theme }
	}
	pub fn theme(&self) -> &Theme {
		&self.theme
	}
}
