use document::color::{*};
use document::patch::{IPatchable, Renamable};
use document::sprite::*;
use math::interpolation::Interpolation;
use math::{Extent2, Vec2};
use std::sync::Arc;

#[test]
fn it_adds_child() {
	let g1 = LayerGroup::new(
		None,
		"group",
		ColorMode::Palette,
		vec![],
		Vec2::new(0., 0.),
		Extent2::new(4u32, 4u32),
	);
	let c1 = Arc::new(LayerNode::CanvasPalette(CanvasPalette::new(
		None,
		"canvas",
		Extent2::new(2u32, 2u32),
		vec![Palette::new(255), Palette::new(128), Palette::new(64), Palette::new(32)],
		Vec::new(),
	)));

	let (patch, _) = g1.add_layer(c1.clone()).unwrap();
	let g2 = g1.patch(&patch).unwrap();

	assert_eq!(g2.children.len(), 1);
	assert_eq!(Arc::strong_count(&c1), 3);
}

#[test]
fn it_removes_child() {
	let g1 = LayerGroup::new(
		None,
		"group",
		ColorMode::Palette,
		vec![Arc::new(LayerNode::CanvasPalette(CanvasPalette::new(
			None,
			"canvas",
			Extent2::new(2u32, 2u32),
			vec![Palette::new(255), Palette::new(128), Palette::new(64), Palette::new(32)],
			Vec::new(),
		)))],
		Vec2::new(0., 0.),
		Extent2::new(4u32, 4u32),
	);

	let c1 = if let LayerNode::CanvasPalette(node) = &**g1.children.get(0).unwrap() {
		node
	} else {
		panic!("Note a CanvasPalette");
	};

	let (patch, _) = g1.remove_layer(c1.id).unwrap();
	let g2 = g1.patch(&patch).unwrap();

	assert_eq!(g1.children.len(), 1);
	assert_eq!(g2.children.len(), 0);
}

#[test]
fn it_moves_child() {
	let rc1 = Arc::new(LayerNode::CanvasPalette(CanvasPalette::new(
		None,
		"canvas_a",
		Extent2::new(2u32, 2u32),
		vec![Palette::new(255), Palette::new(128), Palette::new(64), Palette::new(32)],
		Vec::new(),
	)));
	let rc2 = Arc::new(LayerNode::CanvasPalette(CanvasPalette::new(
		None,
		"canvas_b",
		Extent2::new(2u32, 2u32),
		vec![Palette::new(255), Palette::new(128), Palette::new(64), Palette::new(32)],
		Vec::new(),
	)));
	let g1 = LayerGroup::new(
		None,
		"group",
		ColorMode::Palette,
		vec![rc1.clone(), rc2.clone()],
		Vec2::new(0., 0.),
		Extent2::new(4u32, 4u32),
	);

	let c1 = if let LayerNode::CanvasPalette(node) = &**g1.children.get(0).unwrap() {
		node
	} else {
		panic!("Note a CanvasPalette");
	};

	let c2 = if let LayerNode::CanvasPalette(node) = &**g1.children.get(1).unwrap() {
		node
	} else {
		panic!("Note a CanvasPalette");
	};

	let (patch, _) = g1.move_layer(c2.id, 0).unwrap();
	let g2 = g1.patch(&patch).unwrap();

	assert_eq!(g2.children.len(), 2);
	assert_eq!(g2.children.get(0).unwrap().id(), c2.id);
	assert_eq!(g2.children.get(1).unwrap().id(), c1.id);
}

#[test]
fn it_patchs_child() {
	let rc1 = Arc::new(LayerNode::CanvasPalette(CanvasPalette::new(
		None,
		"canvas_a",
		Extent2::new(2u32, 2u32),
		vec![Palette::new(255), Palette::new(128), Palette::new(64), Palette::new(32)],
		Vec::new(),
	)));
	let rc2 = Arc::new(LayerNode::CanvasPalette(CanvasPalette::new(
		None,
		"canvas_b",
		Extent2::new(2u32, 2u32),
		vec![Palette::new(32), Palette::new(64), Palette::new(128), Palette::new(255)],
		Vec::new(),
	)));
	let g1 = LayerGroup::new(
		None,
		"group",
		ColorMode::Palette,
		vec![rc1.clone(), rc2.clone()],
		Vec2::new(0., 0.),
		Extent2::new(4u32, 4u32),
	);

	let (patch, _) = if let LayerNode::CanvasPalette(node) = &**g1.children.get(0).unwrap() {
		node.rename("canvas_aa").unwrap()
	} else {
		panic!("Note a CanvasPalette");
	};
	let g2 = g1.patch(&patch).unwrap();

	assert_eq!(Arc::strong_count(&rc1), 2);
	assert_eq!(Arc::strong_count(&rc2), 3);

	let c1 = if let LayerNode::CanvasPalette(node) = &**g2.children.get(0).unwrap() {
		node
	} else {
		panic!("Note a CanvasPalette");
	};

	let c2 = if let LayerNode::CanvasPalette(node) = &**g2.children.get(1).unwrap() {
		node
	} else {
		panic!("Note a CanvasPalette");
	};

	assert_eq!(*c1.name, "canvas_aa");
	assert_eq!(*c2.name, "canvas_b");

	let (patch, _) = g1
		.resize(Extent2::new(4, 1), Interpolation::Nearest)
		.unwrap();
	let g2 = g1.patch(&patch).unwrap();

	assert_eq!(*g2.size, Extent2::new(4, 1));

	let c1 = if let LayerNode::CanvasPalette(node) = &**g2.children.get(0).unwrap() {
		node
	} else {
		panic!("Note a CanvasPalette");
	};

	let c2 = if let LayerNode::CanvasPalette(node) = &**g2.children.get(1).unwrap() {
		node
	} else {
		panic!("Note a CanvasPalette");
	};

	assert_eq!(*c1.size, Extent2::new(4, 1));
	assert_eq!(*c2.size, Extent2::new(4, 1));
	assert_eq!(
		*c1.color,
		vec![Palette::new(255), Palette::new(255), Palette::new(64), Palette::new(64)]
	);
	assert_eq!(
		*c2.color,
		vec![Palette::new(32), Palette::new(32), Palette::new(128), Palette::new(128)]
	);
}
