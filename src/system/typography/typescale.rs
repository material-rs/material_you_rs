use std::collections::HashMap;

#[derive(Default, Eq, PartialEq)]
pub enum TypescaleRoles {
	Body,
	#[default]
	Display,
	Headline,
	Label,
	Title,
}

#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
pub enum TypescaleModifier {
	Small,
	#[default]
	Medium,
	Large,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoreTypescale {
	display: HashMap<TypescaleModifier, Typescale>,
	body: HashMap<TypescaleModifier, Typescale>,
	headline: HashMap<TypescaleModifier, Typescale>,
	label: HashMap<TypescaleModifier, Typescale>,
	title: HashMap<TypescaleModifier, Typescale>,
}

impl Default for CoreTypescale {
	//TODO: core typescale font and weight should be dynamic
	// pub enum TypographyReferences {
	// 	BoldWeight,
	// 	BrandTypeface,
	// 	MediumWeight,
	// 	PlainTypeface,
	// 	RegularWeight,
	// }
	fn default() -> Self {
		let body = crate::hashmap! {
			TypescaleModifier::Small
				=> Typescale::new(
					String::from("Roboto Regular"),
					16,
					12,
					0.4,
					400
				),
			TypescaleModifier::Medium
				=> Typescale::new(
					String::from("Roboto Regular"),
					20,
					14,
					0.25,
					400
				),
			TypescaleModifier::Large
				=> Typescale::new(
					String::from("Roboto Medium"),
					24,
					16,
					0.5,
					400,
				),
		};

		let display = crate::hashmap! {
			TypescaleModifier::Small
				=> Typescale::new(
					String::from("Roboto Regular"),
					44,
					36,
					0.,
					400
				),
			TypescaleModifier::Medium
				=> Typescale::new(
					String::from("Roboto Regular"),
					52,
					45,
					0.,
					400
				),

			TypescaleModifier::Large
				=> Typescale::new(
					String::from("Roboto Regular"),
					64,
					57,
					0.,
					400
				),
		};

		let headline = crate::hashmap! {
			TypescaleModifier::Small
				=> Typescale::new(
					String::from("Roboto Regular"),
					32,
					24,
					0.,
					400
				),
			TypescaleModifier::Medium
				=> Typescale::new(
					String::from("Roboto Regular"),
					36,
					28,
					0.,
					400
				),
			TypescaleModifier::Large
				=> Typescale::new(
					String::from("Roboto Regular"),
					40,
					32,
					0.,
					400,
				),
		};

		let label = crate::hashmap! {
			TypescaleModifier::Small
				=> Typescale::new(
					String::from("Roboto Medium"),
					16,
					11,
					0.5,
					500
				),
			TypescaleModifier::Medium
				=> Typescale::new(
					String::from("Roboto Medium"),
					16,
					12,
					0.5,
					500
				),
			TypescaleModifier::Large
				=> Typescale::new(
					String::from("Roboto Medium"),
					20,
					14,
					0.1,
					500
				),
		};

		let title = crate::hashmap! {
			TypescaleModifier::Small
				=> Typescale::new(
					String::from("Roboto Medium"),
					20,
					14,
					0.1,
					500
				),
			TypescaleModifier::Medium
				=> Typescale::new(
					String::from("Roboto Medium"),
					24,
					16,
					0.15,
					500
				),
			TypescaleModifier::Large
				=> Typescale::new(
					String::from("Roboto Regular"),
					28,
					22,
					0.,
					400
				),
		};

		Self {
			display,
			body,
			headline,
			label,
			title,
		}
	}
}

impl CoreTypescale {
	fn body(&self) -> &HashMap<TypescaleModifier, Typescale> {
		&self.body
	}

	fn display(&self) -> &HashMap<TypescaleModifier, Typescale> {
		&self.display
	}

	fn headline(&self) -> &HashMap<TypescaleModifier, Typescale> {
		&self.headline
	}

	fn label(&self) -> &HashMap<TypescaleModifier, Typescale> {
		&self.label
	}

	fn title(&self) -> &HashMap<TypescaleModifier, Typescale> {
		&self.title
	}

	pub fn typescale_of(&self, role: &TypescaleRoles, modifier: &TypescaleModifier) -> &Typescale {
		match role {
			TypescaleRoles::Body => self.body().get(modifier).unwrap(),
			TypescaleRoles::Display => self.display().get(modifier).unwrap(),
			TypescaleRoles::Headline => self.headline().get(modifier).unwrap(),
			TypescaleRoles::Label => self.label().get(modifier).unwrap(),
			TypescaleRoles::Title => self.title().get(modifier).unwrap(),
		}
	}
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Typescale {
	font: String, //from ref
	line_height: u8,
	size: u8,
	tracking: f64,
	weight: u16, //from ref
}

impl Typescale {
	fn new(font: String, line_height: u8, size: u8, tracking: f64, weight: u16) -> Self {
		Self {
			font,
			line_height,
			size,
			tracking,
			weight,
		}
	}

	pub fn font(&self) -> &String {
		&self.font
	}

	pub fn line_height(&self) -> u8 {
		self.line_height
	}

	pub fn size(&self) -> u8 {
		self.size
	}

	pub fn tracking(&self) -> f64 {
		self.tracking
	}

	pub fn weight(&self) -> u16 {
		self.weight
	}
}
