use crate::hashmap;
use crate::tokens::system::MdSys;
use std::collections::HashMap;

/*TODO: impl token derivation
* enum Value {
	Token,
	Value,
}*/
pub fn dictionary() -> HashMap<MdSys, &'static str> {
	hashmap! {
		MdSys::ColorSurfaceTintLight => "#6750a4", // Surface tint
		MdSys::ColorOnErrorContainerLight => "#410e0b", // On error container
		MdSys::ColorOnErrorLight => "#ffffff", // On error
		MdSys::ColorErrorContainerLight => "#f9dedc", // Error container
		MdSys::ColorOnTertiaryContainerLight => "#31111d", // On tertiary container
		MdSys::ColorOnTertiaryLight => "#ffffff", // On tertiary
		MdSys::ColorTertiaryContainerLight => "#ffd8e4", // Tertiary container
		MdSys::ColorTertiaryLight => "#7d5260", // Tertiary
		MdSys::ColorShadowLight => "#000000", // Shadow
		MdSys::ColorErrorLight => "#b3261e", // Error
		MdSys::ColorOutlineLight => "#79747e", // Outline
		MdSys::ColorOnBackgroundLight => "#1c1b1f", // On background
		MdSys::ColorBackgroundLight => "#fffbfe", // Background
		MdSys::ColorInverseOnSurfaceLight => "#f4eff4", // Inverse on surface
		MdSys::ColorInverseSurfaceLight => "#313033", // Inverse surface
		MdSys::ColorOnSurfaceVariantLight => "#49454f", // On surface variant
		MdSys::ColorOnSurfaceLight => "#1c1b1f", // On surface
		MdSys::ColorSurfaceVariantLight => "#e7e0ec", // Surface Variant
		MdSys::ColorSurfaceLight => "#fffbfe", // Surface
		MdSys::ColorOnSecondaryContainerLight => "#1d192b", // On secondary container
		MdSys::ColorOnSecondaryLight => "#ffffff", // On secondary
		MdSys::ColorSecondaryContainerLight => "#e8def8", // Secondary container
		MdSys::ColorSecondaryLight => "#625b71", // Secondary
		MdSys::ColorInversePrimaryLight => "#d0bcff", // Inverse primary
		MdSys::ColorOnPrimaryContainerLight => "#21005d", // On primary container
		MdSys::ColorOnPrimaryLight => "#ffffff", // On primary
		MdSys::ColorPrimaryContainerLight => "#eaddff", // Primary container
		MdSys::ColorPrimaryLight => "#6750a4", // Primary
		MdSys::ColorSurfaceTintDark => "#6750a4", // Surface tint
		MdSys::ColorOnErrorContainerDark => "#f2b8b5", // On error container
		MdSys::ColorOnErrorDark => "#601410", // On error
		MdSys::ColorErrorContainerDark => "#8c1d18", // Error container
		MdSys::ColorOnTertiaryContainerDark => "#ffd8e4", // On tertiary container
		MdSys::ColorOnTertiaryDark => "#492532", // On tertiary
		MdSys::ColorTertiaryContainerDark => "#633b48", // Tertiary container
		MdSys::ColorTertiaryDark => "#efb8c8", // Tertiary
		MdSys::ColorShadowDark => "#000000", // Shadow
		MdSys::ColorErrorDark => "#f2b8b5", // Error
		MdSys::ColorOutlineDark => "#938f99", // Outline
		MdSys::ColorOnBackgroundDark => "#e6e1e5", // On background
		MdSys::ColorBackgroundDark => "#1c1b1f", // Background
		MdSys::ColorInverseOnSurfaceDark => "#313033", // Inverse on surface
		MdSys::ColorInverseSurfaceDark => "#e6e1e5", // Inverse surface
		MdSys::ColorOnSurfaceVariantDark => "#cac4d0", // On surface variant
		MdSys::ColorOnSurfaceDark => "#e6e1e5", // On surface
		MdSys::ColorSurfaceVariantDark => "#49454f", // Surface Variant
		MdSys::ColorSurfaceDark => "#1c1b1f", // Surface
		MdSys::ColorOnSecondaryContainerDark => "#e8def8", // On secondary container
		MdSys::ColorOnSecondaryDark => "#332d41", // On secondary
		MdSys::ColorSecondaryContainerDark => "#4a4458", // Secondary container
		MdSys::ColorSecondaryDark => "#ccc2dc", // Secondary
		MdSys::ColorInversePrimaryDark => "#6750a4", // Inverse primary
		MdSys::ColorOnPrimaryContainerDark => "#eaddff", // On primary container
		MdSys::ColorOnPrimaryDark => "#381e72", // On primary
		MdSys::ColorPrimaryContainerDark => "#4f378b", // Primary container
		MdSys::ColorPrimaryDark => "#d0bcff", // Primary
		MdSys::ColorSurfaceTint => "#6750a4", // Surface tint
		MdSys::ColorOnErrorContainer => "#410e0b", // On error container
		MdSys::ColorOnError => "#ffffff", // On error
		MdSys::ColorErrorContainer => "#f9dedc", // Error container
		MdSys::ColorOnTertiaryContainer => "#31111d", // On tertiary container
		MdSys::ColorOnTertiary => "#ffffff", // On tertiary
		MdSys::ColorTertiaryContainer => "#ffd8e4", // Tertiary container
		MdSys::ColorTertiary => "#7d5260", // Tertiary
		MdSys::ColorShadow => "#000000", // Shadow
		MdSys::ColorError => "#b3261e", // Error
		MdSys::ColorOutline => "#79747e", // Outline
		MdSys::ColorOnBackground => "#1c1b1f", // On background
		MdSys::ColorBackground => "#fffbfe", // Background
		MdSys::ColorInverseOnSurface => "#f4eff4", // Inverse on surface
		MdSys::ColorInverseSurface => "#313033", // Inverse surface
		MdSys::ColorOnSurfaceVariant => "#49454f", // On surface variant
		MdSys::ColorOnSurface => "#1c1b1f", // On surface
		MdSys::ColorSurfaceVariant => "#e7e0ec", // Surface Variant
		MdSys::ColorSurface => "#fffbfe", // Surface
		MdSys::ColorOnSecondaryContainer => "#1d192b", // On secondary container
		MdSys::ColorOnSecondary => "#ffffff", // On secondary
		MdSys::ColorSecondaryContainer => "#e8def8", // Secondary container
		MdSys::ColorSecondary => "#625b71", // Secondary
		MdSys::ColorInversePrimary => "#d0bcff", // Inverse primary
		MdSys::ColorOnPrimaryContainer => "#21005d", // On primary container
		MdSys::ColorOnPrimary => "#ffffff", // On primary
		MdSys::ColorPrimaryContainer => "#eaddff", // Primary container
		MdSys::ColorPrimary => "#6750a4", // Primary
		MdSys::TypescaleLabelSmallFamily => "Roboto",
		MdSys::TypescaleLabelSmallWeight => "500",
		MdSys::TypescaleLabelMediumFamily => "Roboto",
		MdSys::TypescaleLabelMediumWeight => "500",
		MdSys::TypescaleLabelLargeFamily => "Roboto",
		MdSys::TypescaleLabelLargeWeight => "500",
		MdSys::TypescaleBodySmallFamily => "Roboto",
		MdSys::TypescaleBodySmallWeight => "400",
		MdSys::TypescaleBodyMediumFamily => "Roboto",
		MdSys::TypescaleBodyMediumWeight => "400",
		MdSys::TypescaleBodyLargeFamily => "Roboto",
		MdSys::TypescaleBodyLargeWeight => "400",
		MdSys::TypescaleTitleSmallFamily => "Roboto",
		MdSys::TypescaleTitleSmallWeight => "500",
		MdSys::TypescaleTitleMediumFamily => "Roboto",
		MdSys::TypescaleTitleMediumWeight => "500",
		MdSys::TypescaleTitleLargeFamily => "Roboto",
		MdSys::TypescaleTitleLargeWeight => "400",
		MdSys::TypescaleHeadlineSmallFamily => "Roboto",
		MdSys::TypescaleHeadlineSmallWeight => "400",
		MdSys::TypescaleHeadlineMediumFamily => "Roboto",
		MdSys::TypescaleHeadlineMediumWeight => "400",
		MdSys::TypescaleHeadlineLargeFamily => "Roboto",
		MdSys::TypescaleHeadlineLargeWeight => "400",
		MdSys::TypescaleDisplaySmallFamily => "Roboto",
		MdSys::TypescaleDisplaySmallWeight => "400",
		MdSys::TypescaleDisplayMediumFamily => "Roboto",
		MdSys::TypescaleDisplayMediumWeight => "400",
		MdSys::TypescaleDisplayLargeFamily => "Roboto",
		MdSys::TypescaleDisplayLargeWeight => "400",
	}
}
