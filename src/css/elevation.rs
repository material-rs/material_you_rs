use crate::system::color::{Color, ColorContext, ColorRole};
use color_utilities::{hct::Hct, palettes::TonalPalette};
use yew::Classes;

pub fn tone(class_name: &str, context: &mut ColorContext, role: &ColorRole, level: u8) -> Classes {
	let opacity = match level {
		1 => 5,
		2 => 8,
		3 => 11,
		4 => 12,
		5 => 14,
		_ => 0,
	};

	let hct = Hct::from_argb(Color::of(role, context));

	let (h, c, t) = (hct.hue(), hct.chroma(), hct.tone());

	let color = if context.is_light_theme() {
		TonalPalette::from_hue_and_chroma(h, c).tone(t as u8 - opacity)
	} else {
		TonalPalette::from_hue_and_chroma(h, c).tone(t as u8 + opacity)
	};

	super::background(class_name, color)
}

pub fn shadow(class_name: &str, context: &mut ColorContext, level: u8) -> Classes {
	let [_, r, g, b] = Color::of(&ColorRole::Shadow, context);

	let [h1, v1, b1, s1, h2, v2, b2, s2, h3, v3, b3, s3] = match level {
		1 => [0, 1, 1, 0, 0, 2, 1, -1, 0, 1, 3, 0],
		2 => [0, 3, 4, 0, 0, 3, 3, -2, 0, 1, 8, 0],
		3 => [0, 6, 10, 0, 0, 1, 18, 0, 0, 3, 5, -1],
		4 => [0, 8, 10, 1, 0, 3, 14, 2, 0, 5, 5, -3],
		5 => [0, 12, 17, 2, 0, 5, 22, 4, 0, 7, 8, -4],
		_ => [0; 12],
	};

	let [a1, a2, a3] = [0.14, 0.12, 0.20];

	let rule = if level == 0 {
		r#"box-shadow: none;"#.to_owned()
	} else {
		format!(
			r#"
			box-shadow: {}px {}px {}px {}px rgba({}, {}, {}, {}),
				{}px {}px {}px {}px rgba({}, {}, {}, {}),
				{}px {}px {}px {}px rgba({}, {}, {}, {});
            "#,
			h1, v1, b1, s1, r, g, b, a1, h2, v2, b2, s2, r, g, b, a2, h3, v3, b3, s3, r, g, b, a3
		)
	};

	super::new_style(class_name, &rule)
}

//TODO:
pub fn scrim(class_name: &str, _: &ColorContext, _: u8) -> Classes {
	super::new_style(
		class_name,
		r#"
			color: green;
		"#,
	)
}
