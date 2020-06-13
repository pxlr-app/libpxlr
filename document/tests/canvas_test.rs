use collections::bitvec;
use document::color::*;
use document::patch::IPatchable;
use document::sprite::*;
use math::blend::*;
use math::interpolation::*;
use math::{Extent2, Vec2};

#[test]
fn from_buffer() {
	let c1 = CanvasGrey::new(
		None,
		"canvas",
		Extent2::new(2u32, 2u32),
		vec![Grey::new(255), Grey::new(128), Grey::new(64), Grey::new(32)],
		Vec::new(),
	);

	assert_eq!(
		*c1.color,
		vec![Grey::new(255), Grey::new(128), Grey::new(64), Grey::new(32)]
	);
}

#[test]
fn it_crops() {
	let c1 = CanvasGrey::new(
		None,
		"canvas",
		Extent2::new(2u32, 2u32),
		vec![Grey::new(255), Grey::new(128), Grey::new(64), Grey::new(32)],
		Vec::new(),
	);

	let (patch, _) = c1.crop(Vec2::new(1, 0), Extent2::new(1, 2)).unwrap();
	let c2 = c1.patch(&patch).unwrap();

	assert_eq!(*c2.color, vec![Grey::new(128), Grey::new(32)]);
}

#[test]
fn it_resizes() {
	let c1 = CanvasGrey::new(
		None,
		"canvas",
		Extent2::new(2u32, 2u32),
		vec![Grey::new(255), Grey::new(128), Grey::new(64), Grey::new(32)],
		Vec::new(),
	);

	let (patch, _) = c1
		.resize(Extent2::new(4, 4), Interpolation::Nearest)
		.unwrap();
	let c2 = c1.patch(&patch).unwrap();

	assert_eq!(
		*c2.color,
		vec![
			Grey::new(255),
			Grey::new(255),
			Grey::new(128),
			Grey::new(128),
			Grey::new(255),
			Grey::new(255),
			Grey::new(128),
			Grey::new(128),
			Grey::new(64),
			Grey::new(64),
			Grey::new(32),
			Grey::new(32),
			Grey::new(64),
			Grey::new(64),
			Grey::new(32),
			Grey::new(32)
		]
	);

	let (patch, _) = c1
		.resize(Extent2::new(4, 4), Interpolation::Bilinear)
		.unwrap();
	let c2 = c1.patch(&patch).unwrap();
	assert_eq!(
		*c2.color,
		vec![
			Grey::new(255),
			Grey::new(31),
			Grey::new(63),
			Grey::new(95),
			Grey::new(15),
			Grey::new(53),
			Grey::new(91),
			Grey::new(129),
			Grey::new(31),
			Grey::new(75),
			Grey::new(119),
			Grey::new(163),
			Grey::new(47),
			Grey::new(97),
			Grey::new(147),
			Grey::new(197)
		]
	);

	let (patch, _) = c1
		.resize(Extent2::new(2, 1), Interpolation::Nearest)
		.unwrap();
	let c2 = c1.patch(&patch).unwrap();

	assert_eq!(*c2.color, vec![Grey::new(255), Grey::new(64)]);
}

#[test]
fn it_apply_patch() {
	let c1 = CanvasGrey::new(
		None,
		"canvas",
		Extent2::new(2u32, 2u32),
		vec![Grey::new(196), Grey::new(128), Grey::new(64), Grey::new(32)],
		Vec::new(),
	);

	let (patch, _) = c1.apply_stencil(
		Vec2::new(0, 0),
		BlendMode::Normal,
		StencilGrey::from_buffer(
			Extent2::new(2, 2),
			&[
				Grey::new(255),
				Grey::new(255),
				Grey::new(255),
				Grey::new(255),
			],
		),
	);
	let c2 = c1.patch(&patch).unwrap();
	assert_eq!(
		*c2.color,
		vec![
			Grey::new(255),
			Grey::new(255),
			Grey::new(255),
			Grey::new(255)
		]
	);

	let (patch, _) = c1.apply_stencil(
		Vec2::new(0, 0),
		BlendMode::Normal,
		StencilGrey {
			size: Extent2::new(2, 2),
			mask: bitvec![1, 0, 0, 1],
			data: vec![Grey::new(255), Grey::new(255)],
		},
	);
	let c2 = c1.patch(&patch).unwrap();
	assert_eq!(
		*c2.color,
		vec![
			Grey::new(255),
			Grey::new(128),
			Grey::new(64),
			Grey::new(255)
		]
	);
}
