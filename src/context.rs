use color_utilities::{palettes::CorePalette, scheme::Scheme, utils::color::ARGB};
use yew::Properties;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Theme {
	#[default]
	Light,
	Dark,
}

impl Theme {
	pub fn change(&self) -> Theme {
		match self {
			Theme::Dark => Theme::Light,
			Theme::Light => Theme::Dark,
		}
	}
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Context {
	color: ARGB,
	palette: CorePalette,
	scheme: Scheme,
	theme: Theme,
}

impl Default for Context {
	fn default() -> Self {
		Self {
			color: [255, 0, 0, 255],
			palette: CorePalette::default(),
			scheme: Scheme::default(),
			theme: Theme::default(),
		}
	}
}

impl Context {
	pub fn new(theme: Theme, color: ARGB) -> Self {
		let scheme = match theme {
			Theme::Light => Scheme::light(color),
			Theme::Dark => Scheme::dark(color),
		};

		Self {
			color,
			palette: CorePalette::of(color),
			scheme,
			theme,
		}
	}
	pub fn theme(&self) -> &Theme {
		&self.theme
	}
	pub fn change_theme(&self) -> Context {
		Context::new(self.theme.change(), self.color)
	}

	pub fn scheme(&self) -> Scheme {
		match self.theme {
			Theme::Light => Scheme::light(self.color),
			Theme::Dark => Scheme::dark(self.color),
		}
	}
}
