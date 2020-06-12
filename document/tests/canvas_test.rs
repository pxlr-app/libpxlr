use collections::bitvec;
use document::color::*;
use document::patch::IPatchable;
use document::sprite::*;
use math::blend::*;
use math::interpolation::*;
use math::{Extent2, Vec2};

#[test]
fn from_buffer() {
	let c1 = CanvasPalette::new(
		None,
		"canvas",
		Extent2::new(2u32, 2u32),
		vec![Palette::new(255), Palette::new(128), Palette::new(64), Palette::new(32)],
		Vec::new(),
	);

	assert_eq!(
		*c1.color,
		vec![Palette::new(255), Palette::new(128), Palette::new(64), Palette::new(32)]
	);
}

#[test]
fn it_crops() {
	let c1 = CanvasPalette::new(
		None,
		"canvas",
		Extent2::new(2u32, 2u32),
		vec![Palette::new(255), Palette::new(128), Palette::new(64), Palette::new(32)],
		Vec::new(),
	);

	let (patch, _) = c1.crop(Vec2::new(1, 0), Extent2::new(1, 2)).unwrap();
	let c2 = c1.patch(&patch).unwrap();

	assert_eq!(*c2.color, vec![Palette::new(128), Palette::new(32)]);
}

#[test]
fn it_resizes() {
	let c1 = CanvasPalette::new(
		None,
		"canvas",
		Extent2::new(2u32, 2u32),
		vec![Palette::new(255), Palette::new(128), Palette::new(64), Palette::new(32)],
		Vec::new(),
	);

	let (patch, _) = c1
		.resize(Extent2::new(4, 4), Interpolation::Nearest)
		.unwrap();
	let c2 = c1.patch(&patch).unwrap();

	assert_eq!(
		*c2.color,
		vec![
			Palette::new(255),
			Palette::new(255),
			Palette::new(128),
			Palette::new(128),
			Palette::new(255),
			Palette::new(255),
			Palette::new(128),
			Palette::new(128),
			Palette::new(64),
			Palette::new(64),
			Palette::new(32),
			Palette::new(32),
			Palette::new(64),
			Palette::new(64),
			Palette::new(32),
			Palette::new(32)
		]
	);

	let (patch, _) = c1
		.resize(Extent2::new(4, 4), Interpolation::Bilinear)
		.unwrap();
	let c2 = c1.patch(&patch).unwrap();
	assert_eq!(
		*c2.color,
		vec![
			Palette::new(255),
			Palette::new(31),
			Palette::new(63),
			Palette::new(95),
			Palette::new(15),
			Palette::new(53),
			Palette::new(91),
			Palette::new(129),
			Palette::new(31),
			Palette::new(75),
			Palette::new(119),
			Palette::new(163),
			Palette::new(47),
			Palette::new(97),
			Palette::new(147),
			Palette::new(197)
		]
	);

	let (patch, _) = c1
		.resize(Extent2::new(2, 1), Interpolation::Nearest)
		.unwrap();
	let c2 = c1.patch(&patch).unwrap();

	assert_eq!(*c2.color, vec![Palette::new(255), Palette::new(64)]);
}

#[test]
fn it_apply_patch() {
	let c1 = CanvasPalette::new(
		None,
		"canvas",
		Extent2::new(2u32, 2u32),
		vec![Palette::new(196), Palette::new(128), Palette::new(64), Palette::new(32)],
		Vec::new(),
	);

	let (patch, _) = c1.apply_stencil(
		Vec2::new(0, 0),
		BlendMode::Normal,
		StencilPalette::from_buffer(
			Extent2::new(2, 2),
			&[Palette::new(255), Palette::new(255), Palette::new(255), Palette::new(255)],
		),
	);
	let c2 = c1.patch(&patch).unwrap();
	assert_eq!(
		*c2.color,
		vec![Palette::new(255), Palette::new(255), Palette::new(255), Palette::new(255)]
	);

	let (patch, _) = c1.apply_stencil(
		Vec2::new(0, 0),
		BlendMode::Normal,
		StencilPalette {
			size: Extent2::new(2, 2),
			mask: bitvec![1, 0, 0, 1],
			data: vec![Palette::new(255), Palette::new(255)],
		},
	);
	let c2 = c1.patch(&patch).unwrap();
	assert_eq!(
		*c2.color,
		vec![Palette::new(255), Palette::new(128), Palette::new(64), Palette::new(255)]
	);
}
