# This is a Rust's Design System base on Material 3 spec

## Status
working on,

- ***currently we working on yew's api, but in the future we hope can provide an api for all rustwasm frameworks***

## Goals
- provide material's design spec in a rust's api for use in wasm environment
- provide a zero js design system for rust


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
use yew::prelude::*;
use yew::prelude::ThemeProvider;

#[funcion_component(App)]
fn app() -> Htlm{
	html! { <ThemeProvider> //provides access to material color system and theming to whole app
		<App/>
	</ThemeProvider> }
}
```

- now you can use components
```rust
use yew::prelude::*;
use yew::prelude::Card;

#[funcion_component(SomeComponent)]
fn app() -> Htlm{ 
	html! { <>
		<Card>{"Some card content"}</Card>
	</> }
}
```

## Development

### Prerequisites 

- rust [install rust](https://www.rust-lang.org/tools/install)

- add wasm32 target
```bash
rustup target add wasm32-unknown-unknown
```

- install wasm tools, wasm-pack and wasm-bingend-cli, and cargo-watch (for live reload)
```bash
cargo install wasm-pack wasm-bindgen-cli
```

- some http server for example `python http.server`

### Run

clone repository and web-site (for render examples at time that proof components)
both have to be in same root direcotry
```bash
git clone https://github.com/material-rs/material-you-rs.git
git clone https://github.com/material-rs/material-you-site.git
```

build lib
```bash
cd /path/of/material-you-rs
wasm-pack build --target web
```

build and serve site
```bash
cd /path/of/material-you-site
wasm-pack build --target web
python3 -m http.server
```

now you can see example site on your browser

# Development

for development and live reload
```bash
cd /path/of/material-you-site
cargo watch -w . -w ../material-you-site -s "wasm-pack build --target web"
```

and in another termninal serve site
```bash
cd /path/of/material-you-site
python3 -m http.server
```

# Contribution guide
working on... (we have not defined the development cycle)


# License
LGPL V3 or MIT

<!--
[//]: <>  definitions
[//]: <>  
[//]: <>  Key: Accent | Neutral | Error
[//]: <>  		< Accent[ primary key color | secondary key color | tertiary key color ] >
[//]: <>  	< Neutral[ neutral key color | neutral variant key color ] >
[//]: <>  palette: <Key, <ton1, ton2, ...., ton13> >
[//]: <>  
[//]: <>  tone -> assigned to role
[//]: <>  
[//]: <>  tonal palette ->  [color,0..100] color with % of ligth (palette have 13 tones)
[//]: <>  e.g. primary40 -> primary key color with 40% of ligth
[//]: <>  
[//]: <>  color scheme -> a set/group of tones assigned to specific roles that get mapped to components
[//]: <>  
[//]: <>  scheme roles ->
[//]: <>  accent keys: pairing, emphasis, visual expresion
[//]: <>  neutral keys: surfaces and backgrouns... and high emphasis text and icons
[//]: <>  
[//]: <>  -----------------------------------------------------------------------------
[//]: <>   Pseudocode of usage (design)
[//]: <>  {
[//]: <>  
[//]: <>  	color(md.sys.color.surface) = Color::of(Role::Surface);
[//]: <>  	css = \"
[//]: <>  		background-color: md.sys.color.surface //(color:surface)
[//]: <>  	\";
[//]: <>  	<template css>...</template>
[//]: <>  }
[//]: <>  ----------------------------------------------------------------------------
[//]: <>  
[//]: <>  
[//]: <>  
[//]: <>  NOTE: all this is based on one color scheme.. where is passed?
[//]: <>  let color = Color::of(Role::Primary);
[//]: <>  fn Color::of(role: Role) -> Hex {
[//]: <>  	Token::of(Role).hex()
[//]: <>  }
[//]: <>  
[//]: <>  fn Token::of(role: Role) -> Token {
[//]: <>  	// encapsulates current baseline theme
[//]: <>  	if dark return dark token
[//]: <>  	if light return ligth token
[//]: <>  }
[//]: <>  fn hex(self: Token) -> Hex {
[//]: <>  }
[//]: <>  
[//]: <>  let color = Color::of(Role::Surface);
[//]: <>  
[//]: <>  
[//]: <>  for surfaces level 1..5
[//]: <>  let color = Color::of(Role::Surface).level(3);
[//]: <>  
[//]: <>  fn Hex::level(self: Hex, level: Level) -> Hex  {
[//]: <>  	//add opacity to current color
[//]: <>  }
[//]: <>  
-->
