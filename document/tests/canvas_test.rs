use collections::bitvec;
use document::patch::Patchable;
use document::sprite::{CanvasI, Layer, StencilI};
use math::blend::*;
use math::color::I;
use math::interpolation::*;
use math::{Extent2, Vec2};

#[test]
fn from_buffer() {
	let c1 = CanvasI::new(
		None,
		"canvas",
		Extent2::new(2u32, 2u32),
		vec![I::new(255), I::new(128), I::new(64), I::new(32)],
	);

	assert_eq!(
		*c1.data,
		vec![I::new(255), I::new(128), I::new(64), I::new(32)]
	);
}

#[test]
fn it_crops() {
	let c1 = CanvasI::new(
		None,
		"canvas",
		Extent2::new(2u32, 2u32),
		vec![I::new(255), I::new(128), I::new(64), I::new(32)],
	);

	let (patch, _) = c1.crop(Vec2::new(1, 0), Extent2::new(1, 2)).unwrap();
	let c2 = c1.patch(&patch).unwrap();

	assert_eq!(*c2.data, vec![I::new(128), I::new(32)]);
}

#[test]
fn it_resizes() {
	let c1 = CanvasI::new(
		None,
		"canvas",
		Extent2::new(2u32, 2u32),
		vec![I::new(255), I::new(128), I::new(64), I::new(32)],
	);

	let (patch, _) = c1
		.resize(Extent2::new(4, 4), Interpolation::Nearest)
		.unwrap();
	let c2 = c1.patch(&patch).unwrap();

	assert_eq!(
		*c2.data,
		vec![
			I::new(255),
			I::new(255),
			I::new(128),
			I::new(128),
			I::new(255),
			I::new(255),
			I::new(128),
			I::new(128),
			I::new(64),
			I::new(64),
			I::new(32),
			I::new(32),
			I::new(64),
			I::new(64),
			I::new(32),
			I::new(32)
		]
	);

	let (patch, _) = c1
		.resize(Extent2::new(4, 4), Interpolation::Bilinear)
		.unwrap();
	let c2 = c1.patch(&patch).unwrap();
	assert_eq!(
		*c2.data,
		vec![
			I::new(255),
			I::new(31),
			I::new(63),
			I::new(95),
			I::new(15),
			I::new(53),
			I::new(91),
			I::new(129),
			I::new(31),
			I::new(75),
			I::new(119),
			I::new(163),
			I::new(47),
			I::new(97),
			I::new(147),
			I::new(197)
		]
	);

	let (patch, _) = c1
		.resize(Extent2::new(2, 1), Interpolation::Nearest)
		.unwrap();
	let c2 = c1.patch(&patch).unwrap();

	assert_eq!(*c2.data, vec![I::new(255), I::new(64)]);
}

#[test]
fn it_apply_patch() {
	let c1 = CanvasI::new(
		None,
		"canvas",
		Extent2::new(2u32, 2u32),
		vec![I::new(196), I::new(128), I::new(64), I::new(32)],
	);

	let (patch, _) = c1.apply_stencil(
		Vec2::new(0, 0),
		BlendMode::Normal,
		StencilI::from_buffer(
			Extent2::new(2, 2),
			&[I::new(255), I::new(255), I::new(255), I::new(255)],
		),
	);
	let c2 = c1.patch(&patch).unwrap();
	assert_eq!(
		*c2.data,
		vec![I::new(255), I::new(255), I::new(255), I::new(255)]
	);

	let (patch, _) = c1.apply_stencil(
		Vec2::new(0, 0),
		BlendMode::Normal,
		StencilI {
			size: Extent2::new(2, 2),
			mask: bitvec![1, 0, 0, 1],
			data: vec![I::new(255), I::new(255)],
		},
	);
	let c2 = c1.patch(&patch).unwrap();
	assert_eq!(
		*c2.data,
		vec![I::new(255), I::new(128), I::new(64), I::new(255)]
	);
}
