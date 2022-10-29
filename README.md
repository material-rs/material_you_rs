# This is a Rust's Design System based on Material 3 spec

## Status
working on, (not ready usable)

## Development

### Prerequisites 

- rust [install rust](https://www.rust-lang.org/tools/install)

- add wasm32 target
```bash
rustup target add wasm32-unknown-unknown
```

- install wasm tools, wasm-pack and wasm-bingend-cli, and cargo-watch (for live reload)
```bash
cargo install wasm-pack wasm-bindgen-cli cargo-watch
```

- some http server for example `python http.server`

### Run

- clone repository and web-site (for render examples at time that proof components)
both have to be in same root direcotry
```bash
git clone https://github.com/material-rs/material-you-rs.git
git clone https://github.com/material-rs/material-you-site.git
```

- build lib
```bash
cd /path/of/material-you-rs
wasm-pack build --target web
```

- build and serve site
```bash
cd /path/of/material-you-site
wasm-pack build --target web
python3 -m http.server
```

now you can see example site on your browser

### Development whatching file's changes

- for development and live reload
```bash
cd /path/of/material_you_site
cargo watch -w . -w ../material_you_rs -s "wasm-pack build --target web"
```

- and in another termninal serve site
```bash
cd /path/of/material_you_site
python3 -m http.server
```



## Goals
- provide material's design spec in a rust's api for use in rustwasm based environments
- provide a zero js design system for rust

>  ***currently we working on yew's api, but in the future we hope can provide an api for all rustwasm frameworks***

## Roadmap
- implement basic token mappings of material design
- implement theming provider
- implement components
- implement theme generator


## Usage 

- add dependency to your `Cargo.toml`
```bash
cargo add material_you
```

- import library and add the theme provider

```rust
use yew::prelude::{function_component, html};
use material_you::provider::MaterialProvider;

#[function_component(MyApp)]
fn my_app() -> Html {
	// MaterialProvider provides context info for material design
	// for theming, icons, typography
	html! { <MaterialProvider>
		<p>{"My App"}</p>
	</MaterialProvider> }
}
```

- now you can use components

```rust
use yew::prelude::{function_component, html};
use material_you::{
	color::ColorRole, 
	components::card::{
		Card, 
		CardKind, 
		FilledCard, 
	},
	css,
	provider::MaterialProvider
};

#[function_component(MyComponent)]
fn my_component() -> Html {
	let styles = {
		let custom_styles = css::new_style("div", r#"margin: 8px;"#);

		vec![custom_styles]
	};

	let custom_bg_role = ColorRole::Tertiary;

	html! { <div>
		<Card bg_role={custom_bg_role} styles={styles.clone()} >{"ElevatedCard is the default card"}</Card>

		<Card kind={CardKind::Outlined} styles={styles.clone()} >{"Outlined Card"}</Card>

		<FilledCard styles={styles.clone()} >{"Filled card"}</FilledCard>
	</div> }
}

#[function_component(MyApp)]
fn my_app() -> Html {
	html! { <MaterialProvider>
		<MyComponent />
	</MaterialProvider> }
}
```

# Contribution guide
working on... (we have not defined the development cycle)

# License
LGPL V3 or MIT
[![License: LGPL v3](https://img.shields.io/badge/License-LGPL_v3-blue.svg)](https://www.gnu.org/licenses/lgpl-3.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

