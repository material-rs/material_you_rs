use crate::hashmap;
use crate::tokens::reference::MdRef;
use std::collections::HashMap;

//TODO: const values of dictionary should be assign automatically,
//		based on a set of global inputs, like base color...
pub fn dictionary() -> HashMap<MdRef, &'static str> {
	hashmap! {
		MdRef::PaletteError0 => "#000000", // Error 0
		MdRef::PaletteError10 => "#410e0b", // Error 10
		MdRef::PaletteError20 => "#601410", // Error 20
		MdRef::PaletteError30 => "#8c1d18", // Error 30
		MdRef::PaletteError40 => "#b3261e", // Error 40
		MdRef::PaletteError50 => "#dc362e", // Error 50
		MdRef::PaletteError60 => "#e46962", // Error 60
		MdRef::PaletteError70 => "#ec928e", // Error 70
		MdRef::PaletteError80 => "#f2b8b5", // Error 80
		MdRef::PaletteError90 => "#f9dedc", // Error 90
		MdRef::PaletteError95 => "#fceeee", // Error 95
		MdRef::PaletteError99 => "#fffbf9", // Error 99
		MdRef::PaletteError100 => "#ffffff", // Error 100
		MdRef::PaletteTertiary0 => "#000000", // Tertiary 0
		MdRef::PaletteTertiary10 => "#31111d", // Tertiary 10
		MdRef::PaletteTertiary20 => "#492532", // Tertiary 20
		MdRef::PaletteTertiary30 => "#633b48", // Tertiary 30
		MdRef::PaletteTertiary40 => "#7d5260", // Tertiary 40
		MdRef::PaletteTertiary50 => "#986977", // Tertiary 50
		MdRef::PaletteTertiary60 => "#b58392", // Tertiary 60
		MdRef::PaletteTertiary70 => "#d29dac", // Tertiary 70
		MdRef::PaletteTertiary80 => "#efb8c8", // Tertiary 80
		MdRef::PaletteTertiary90 => "#ffd8e4", // Tertiary 90
		MdRef::PaletteTertiary95 => "#ffecf1", // Tertiary 95
		MdRef::PaletteTertiary99 => "#fffbfa", // Tertiary 99
		MdRef::PaletteTertiary100 => "#ffffff", // Tertiary 100
		MdRef::PaletteSecondary0 => "#000000", // Secondary 0
		MdRef::PaletteSecondary10 => "#1d192b", // Secondary 10
		MdRef::PaletteSecondary20 => "#332d41", // Secondary 20
		MdRef::PaletteSecondary30 => "#4a4458", // Secondary 30
		MdRef::PaletteSecondary40 => "#625b71", // Secondary 40
		MdRef::PaletteSecondary50 => "#7a7289", // Secondary 50
		MdRef::PaletteSecondary60 => "#958da5", // Secondary 60
		MdRef::PaletteSecondary70 => "#b0a7c0", // Secondary 70
		MdRef::PaletteSecondary80 => "#ccc2dc", // Secondary 80
		MdRef::PaletteSecondary90 => "#e8def8", // Secondary 90
		MdRef::PaletteSecondary95 => "#f6edff", // Secondary 95
		MdRef::PaletteSecondary99 => "#fffbfe", // Secondary 99
		MdRef::PaletteSecondary100 => "#ffffff", // Secondary 100
		MdRef::PalettePrimary0 => "#000000", // Primary 0
		MdRef::PalettePrimary10 => "#21005d", // Primary 10
		MdRef::PalettePrimary20 => "#381e72", // Primary 20
		MdRef::PalettePrimary30 => "#4f378b", // Primary 30
		MdRef::PalettePrimary40 => "#6750a4", // Primary 40
		MdRef::PalettePrimary50 => "#7f67be", // Primary 50
		MdRef::PalettePrimary60 => "#9a82db", // Primary 60
		MdRef::PalettePrimary70 => "#b69df8", // Primary 70
		MdRef::PalettePrimary80 => "#d0bcff", // Primary 80
		MdRef::PalettePrimary90 => "#eaddff", // Primary 90
		MdRef::PalettePrimary95 => "#f6edff", // Primary 95
		MdRef::PalettePrimary99 => "#fffbfe", // Primary 99
		MdRef::PalettePrimary100 => "#ffffff", // Primary 100
		MdRef::PaletteNeutralVariant0 => "#000000", // Neutral Variant 0
		MdRef::PaletteNeutralVariant10 => "#1d1a22", // Neutral Variant 10
		MdRef::PaletteNeutralVariant20 => "#322f37", // Neutral Variant 20
		MdRef::PaletteNeutralVariant30 => "#49454f", // Neutral Variant 30
		MdRef::PaletteNeutralVariant40 => "#605d66", // Neutral Variant 40
		MdRef::PaletteNeutralVariant50 => "#79747e", // Neutral Variant 50
		MdRef::PaletteNeutralVariant60 => "#938f99", // Neutral Variant 60
		MdRef::PaletteNeutralVariant70 => "#aea9b4", // Neutral Variant 70
		MdRef::PaletteNeutralVariant80 => "#cac4d0", // Neutral Variant 80
		MdRef::PaletteNeutralVariant90 => "#e7e0ec", // Neutral Variant 90
		MdRef::PaletteNeutralVariant95 => "#f5eefa", // Neutral Variant 95
		MdRef::PaletteNeutralVariant99 => "#fffbfe", // Neutral Variant 99
		MdRef::PaletteNeutralVariant100 => "#ffffff", // Neutral Variant 100
		MdRef::PaletteNeutral0 => "#000000", // Neutral 0
		MdRef::PaletteNeutral10 => "#1c1b1f", // Neutral 10
		MdRef::PaletteNeutral20 => "#313033", // Neutral 20
		MdRef::PaletteNeutral30 => "#484649", // Neutral 30
		MdRef::PaletteNeutral40 => "#605d62", // Neutral 40
		MdRef::PaletteNeutral50 => "#787579", // Neutral 50
		MdRef::PaletteNeutral60 => "#939094", // Neutral 60
		MdRef::PaletteNeutral70 => "#aeaaae", // Neutral 70
		MdRef::PaletteNeutral80 => "#c9c5ca", // Neutral 80
		MdRef::PaletteNeutral90 => "#e6e1e5", // Neutral 90
		MdRef::PaletteNeutral95 => "#f4eff4", // Neutral 95
		MdRef::PaletteNeutral99 => "#fffbfe", // Neutral 99
		MdRef::PaletteNeutral100 => "#ffffff", // Neutral 100
		MdRef::PaletteBlack => "#000000", // Black
		MdRef::PaletteWhite => "#ffffff", // White
		MdRef::TypefacePlain => "Roboto", // Plain typeface
		MdRef::TypefaceBrand => "Roboto", // Brand typeface
		MdRef::TypefaceWeightBold => "700", // Bold weight
		MdRef::TypefaceWeightMedium => "500", // Medium weight
		MdRef::TypefaceWeightRegular => "400", // Regular weight
	}
}
