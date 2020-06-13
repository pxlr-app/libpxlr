use document::color::Grey;
use document::sprite::StencilGrey;
use editor::tool::Line;
use math::interpolation::Interpolation;
use math::Vec2;

#[test]
fn it_paints() {
	let line = Line::<StencilGrey> {
		from: Vec2::new(0, 0),
		to: Vec2::new(0, 9),
		width: 1,
		color: Grey::new(255),
		interpolation: Interpolation::Nearest,
	};
	let stencil = line.get_stencil();

	assert_eq!(
		format!("{:?}", stencil),
		"StencilGrey { ⡇\n          ⡇\n          ⠃ }"
	);

	let line = Line::<StencilGrey> {
		from: Vec2::new(9, 0),
		to: Vec2::new(0, 9),
		width: 1,
		color: Grey::new(255),
		interpolation: Interpolation::Nearest,
	};
	let stencil = line.get_stencil();

	assert_eq!(
		format!("{:?}", stencil),
		"StencilGrey { ⠀⠀⠀⡠⠊\n          ⠀⡠⠊⠀⠀\n          ⠊⠀⠀⠀⠀ }"
	);
}
