/*use material_you::{
	context::{Context, Theme},
	mapper::Mapper,
	roles::Roles,
	tokens::{system::MdSys, token::Token},
};

#[test]
fn basic_mapping() {
	let ligth_ctx = Context::new(Theme::Light, [255, 0, 0, 255]);
	let res = Mapper::map(Roles::Surface, ligth_ctx);

	let expected = Token::Sys(MdSys::ColorSurfaceLight);

	assert_eq!(res, expected);

	let dark_ctx = Context::new(Theme::Dark, [255, 0, 0, 255]);
	let res = Mapper::map(Roles::Surface, dark_ctx);

	let expected = Token::Sys(MdSys::ColorSurfaceDark);

	assert_eq!(res, expected);
}*/
