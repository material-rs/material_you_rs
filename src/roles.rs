/*
*   this module contains roles that's needed to map tokens
* */
use crate::tokens::{system::MdSys, token::*};

impl Roles {
	// obtains high level token
	pub fn token(self) -> Token {
		match self {
			Roles::Primary => Token::Sys(MdSys::ColorPrimary),

			Roles::PrimaryContainer => Token::Sys(MdSys::ColorPrimaryContainer),

			Roles::Secondary => Token::Sys(MdSys::ColorSecondary),

			Roles::SecondaryContainer => Token::Sys(MdSys::ColorSecondaryContainer),

			Roles::Tertiary => Token::Sys(MdSys::ColorTertiary),

			Roles::TertiaryContainer => Token::Sys(MdSys::ColorTertiaryContainer),

			Roles::Surface => Token::Sys(MdSys::ColorSurface),

			Roles::SurfaceVariant => Token::Sys(MdSys::ColorSurfaceVariant),

			Roles::Background => Token::Sys(MdSys::ColorBackground),

			Roles::Error => Token::Sys(MdSys::ColorError),

			Roles::ErrorContainer => Token::Sys(MdSys::ColorErrorContainer),

			Roles::OnPrimary => Token::Sys(MdSys::ColorOnPrimary),

			Roles::OnPrimaryContainer => Token::Sys(MdSys::ColorOnPrimaryContainer),

			Roles::OnSecondary => Token::Sys(MdSys::ColorOnSecondary),

			Roles::OnSecondaryContainer => Token::Sys(MdSys::ColorOnSecondaryContainer),

			Roles::OnTertiary => Token::Sys(MdSys::ColorOnTertiary),

			Roles::OnTertiaryContainer => Token::Sys(MdSys::ColorOnTertiaryContainer),

			Roles::OnSurface => Token::Sys(MdSys::ColorOnSurface),

			Roles::OnSurfaceVariant => Token::Sys(MdSys::ColorOnSurfaceVariant),

			Roles::OnError => Token::Sys(MdSys::ColorOnError),

			Roles::OnErrorContainer => Token::Sys(MdSys::ColorOnErrorContainer),

			Roles::OnBackground => Token::Sys(MdSys::ColorOnBackground),

			Roles::Outline => Token::Sys(MdSys::ColorOutline),

			//NOTE: missing (error on materials docs?)
			//Roles::OutlineVariant => Token::Sys(MdSys::ColorOutlineVariant),
			Roles::Shadow => Token::Sys(MdSys::ColorShadow),

			//NOTE: ColorSurfaceTint instead of ColorSurfaceTintColor (error on materials docs?)
			Roles::SurfaceTint => Token::Sys(MdSys::ColorSurfaceTint),

			Roles::InverseSurface => Token::Sys(MdSys::ColorInverseSurface),

			Roles::InverseOnSurface => Token::Sys(MdSys::ColorInverseOnSurface),

			Roles::InversePrimary => Token::Sys(MdSys::ColorInversePrimary),

			//NOTE: missing (error on materials docs?)
			//Roles::Scrim => Token::Sys(MdSys::ColorScrim),
			_ => todo!(),
		}
	}
}

crate::display!(Roles);

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Roles {
	Background,
	Black,
	BoldWeight,
	BrandTypeface,
	Error,
	Error0,
	Error10,
	Error100,
	Error20,
	Error30,
	Error40,
	Error50,
	Error60,
	Error70,
	Error80,
	Error90,
	Error95,
	Error99,
	ErrorContainer,
	InverseOnSurface,
	InversePrimary,
	InverseSurface,
	MediumWeight,
	Neutral0,
	Neutral10,
	Neutral100,
	Neutral20,
	Neutral30,
	Neutral40,
	Neutral50,
	Neutral60,
	Neutral70,
	Neutral80,
	Neutral90,
	Neutral95,
	Neutral99,
	NeutralVariant0,
	NeutralVariant10,
	NeutralVariant100,
	NeutralVariant20,
	NeutralVariant30,
	NeutralVariant40,
	NeutralVariant50,
	NeutralVariant60,
	NeutralVariant70,
	NeutralVariant80,
	NeutralVariant90,
	NeutralVariant95,
	NeutralVariant99,
	OnBackground,
	OnError,
	OnErrorContainer,
	OnPrimary,
	OnPrimaryContainer,
	OnSecondary,
	OnSecondaryContainer,
	OnSurface,
	OnSurfaceVariant,
	OnTertiary,
	OnTertiaryContainer,
	Outline,
	PlainTypeface,
	Primary,
	Primary0,
	Primary10,
	Primary100,
	Primary20,
	Primary30,
	Primary40,
	Primary50,
	Primary60,
	Primary70,
	Primary80,
	Primary90,
	Primary95,
	Primary99,
	PrimaryContainer,
	RegularWeight,
	Secondary,
	Secondary0,
	Secondary10,
	Secondary100,
	Secondary20,
	Secondary30,
	Secondary40,
	Secondary50,
	Secondary60,
	Secondary70,
	Secondary80,
	Secondary90,
	Secondary95,
	Secondary99,
	SecondaryContainer,
	Shadow,
	Surface,
	SurfaceTint,
	SurfaceVariant,
	Tertiary,
	Tertiary0,
	Tertiary10,
	Tertiary100,
	Tertiary20,
	Tertiary30,
	Tertiary40,
	Tertiary50,
	Tertiary60,
	Tertiary70,
	Tertiary80,
	Tertiary90,
	Tertiary95,
	Tertiary99,
	TertiaryContainer,
	White,
}
