//TODO: move definiton to other site and break circular dependency
use crate::system::{shapes::Family, typography::Typescale};
use color_utilities::utils::color::ARGB;
use yew::{classes, html::Classes};

pub mod elevation;

pub fn new_style(class_name: &str, css: &str) -> Classes {
	yew::html::Classes::from(
		css_in_rust::Style::create(class_name, css)
			.expect("")
			.to_string(),
	)
}

pub fn background(class_name: &str, color: ARGB) -> Classes {
	let a = color[0] as f64 / 255.0;

	background_with_alpha(class_name, color, a)
}

pub fn background_with_alpha(class_name: &str, color: ARGB, alpha: f64) -> Classes {
	let [_, r, g, b] = color;

	let style = format!(r#"background: rgba({},{},{},{});"#, r, g, b, alpha);

	new_style(class_name, &style)
}

pub fn color(class_name: &str, color: ARGB) -> Classes {
	let a = color[0] as f64 / 255.0;

	color_with_alpha(class_name, color, a)
}

pub fn color_with_alpha(class_name: &str, color: ARGB, alpha: f64) -> Classes {
	let [_, r, g, b] = color;

	let style = format!(r#"color: rgba({},{},{},{});"#, r, g, b, alpha);

	new_style(class_name, &style)
}

pub fn border(class_name: &str, kind: &Family, size: [f32; 4]) -> Classes {
	//TODO: make styles for cutted and circular borders
	let style = match kind {
		Family::Cuted => r#"border: 2px solid red;"#.to_string(),
		Family::Circular => r#"100rem;"#.to_string(),
		Family::Rounded => format!(
			r#"border-radius: {}px {}px {}px {}px;"#,
			size[0], size[1], size[2], size[3]
		),
	};

	new_style(class_name, &style)
}

pub fn fonts_style(class_name: &str, typescale: &Typescale) -> Classes {
	//TODO: make styles
	let style = format!(
		r#"
			blockquote, dd, div, dl, dt, figcaption, figure, hr, li, menu, ol, p, pre, ul, a, abbr, b, bdi, bdo, br, cite,code, data, dfn, em, i, kbd, mark,q, rp, rt, ruby, s, samp, small, span, strong, sub, sup, time, u, var, wbr: {{
				font-family: {};
				font-size: {}px;
		}}"#,
		typescale.font(),
		typescale.size()
	);

	let s = new_style(
		class_name,
		&format!(
			r#"
				font-family: {};
				font-size: {}px;
			"#,
			typescale.font(),
			typescale.size()
		),
	);

	//new_style(class_name, &style)
	classes!(style, s)
}
