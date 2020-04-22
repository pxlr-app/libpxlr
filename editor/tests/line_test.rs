use document::sprite::color::I;
use document::sprite::{Interpolation, StencilI};
use editor::tool::Line;
use math::Vec2;

#[test]
fn it_paints() {
	let line = Line::<StencilI> {
		from: Vec2::new(0, 0),
		to: Vec2::new(0, 9),
		width: 1,
		color: I::new(255),
		interpolation: Interpolation::Nearest,
	};
	let stencil = line.get_stencil();

	assert_eq!(
		format!("{:?}", stencil),
		"StencilI { ⡇\n          ⡇\n          ⠃ }"
	);

	let line = Line::<StencilI> {
		from: Vec2::new(9, 0),
		to: Vec2::new(0, 9),
		width: 1,
		color: I::new(255),
		interpolation: Interpolation::Nearest,
	};
	let stencil = line.get_stencil();

	assert_eq!(
		format!("{:?}", stencil),
		"StencilI { ⠀⠀⠀⡠⠊\n          ⠀⡠⠊⠀⠀\n          ⠊⠀⠀⠀⠀ }"
	);
}
