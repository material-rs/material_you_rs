use yew::html::Classes;

pub fn new_style(class_name: &str, css: &str) -> Classes {
	yew::html::Classes::from(
		css_in_rust::Style::create(class_name, css)
			.expect("")
			.to_string(),
	)
}

/*pub fn color(class_name: &str, color: &str) -> Classes {
	let style = format!(r#"color: {};"#, color);

	new_style(class_name, &style)
}*/

pub fn background(class_name: &str, color: &str) -> Classes {
	let style = format!(r#"background: {};"#, color);

	new_style(class_name, &style)
}
