//TODO: move definiton to other site and break circular dependency
use crate::system::shapes::Family;
use color_utilities::utils::color::ARGB;
use yew::html::Classes;

pub fn new_style(class_name: &str, css: &str) -> Classes {
	yew::html::Classes::from(
		css_in_rust::Style::create(class_name, css)
			.expect("")
			.to_string(),
	)
}

pub fn background(class_name: &str, color: ARGB) -> Classes {
	let [a, r, g, b] = color;

	let a = a as f64 / 255.0;

	let style = format!(r#"background: rgba({},{},{},{});"#, r, g, b, a);

	new_style(class_name, &style)
}

pub fn color(class_name: &str, color: ARGB) -> Classes {
	let [a, r, g, b] = color;

	let a = a as f64 / 255.0;

	let style = format!(r#"color: rgba({},{},{},{});"#, r, g, b, a);

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
