#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum ElevationKind {
	#[default]
	Tone,
	Shadow,
	Scrim,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Elevation {
	kind: ElevationKind,
	level: u8,
}

impl Elevation {
	pub fn tone(level: u8) -> Self {
		Self {
			kind: ElevationKind::Tone,
			level,
		}
	}

	pub fn shadow(level: u8) -> Self {
		Self {
			kind: ElevationKind::Shadow,
			level,
		}
	}

	pub fn scrim(level: u8) -> Self {
		Self {
			kind: ElevationKind::Scrim,
			level,
		}
	}

	pub fn level(&self) -> u8 {
		self.level
	}

	pub fn kind(&self) -> &ElevationKind {
		&self.kind
	}
}
