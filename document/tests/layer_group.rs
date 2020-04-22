use document::patch::{Patchable, Renamable};
use document::sprite::*;
use math::color::{ColorMode, I};
use math::interpolation::Interpolation;
use math::{Extent2, Vec2};
use std::rc::Rc;

#[test]
fn it_adds_child() {
	let g1 = LayerGroup::new(
		None,
		"group",
		ColorMode::I,
		vec![],
		Vec2::new(0., 0.),
		Extent2::new(4u32, 4u32),
	);
	let c1 = Rc::new(LayerNode::CanvasI(CanvasI::new(
		None,
		"canvas",
		Extent2::new(2u32, 2u32),
		vec![I::new(255), I::new(128), I::new(64), I::new(32)],
	)));

	let (patch, _) = g1.add_layer(c1.clone()).unwrap();
	let g2 = g1.patch(&patch).unwrap();

	assert_eq!(g2.children.len(), 1);
	assert_eq!(Rc::strong_count(&c1), 3);
}

#[test]
fn it_removes_child() {
	let g1 = LayerGroup::new(
		None,
		"group",
		ColorMode::I,
		vec![Rc::new(LayerNode::CanvasI(CanvasI::new(
			None,
			"canvas",
			Extent2::new(2u32, 2u32),
			vec![I::new(255), I::new(128), I::new(64), I::new(32)],
		)))],
		Vec2::new(0., 0.),
		Extent2::new(4u32, 4u32),
	);

	let c1 = if let LayerNode::CanvasI(node) = &**g1.children.get(0).unwrap() {
		node
	} else {
		panic!("Note a CanvasI");
	};

	let (patch, _) = g1.remove_layer(c1.id).unwrap();
	let g2 = g1.patch(&patch).unwrap();

	assert_eq!(g1.children.len(), 1);
	assert_eq!(g2.children.len(), 0);
}

#[test]
fn it_moves_child() {
	let rc1 = Rc::new(LayerNode::CanvasI(CanvasI::new(
		None,
		"canvas_a",
		Extent2::new(2u32, 2u32),
		vec![I::new(255), I::new(128), I::new(64), I::new(32)],
	)));
	let rc2 = Rc::new(LayerNode::CanvasI(CanvasI::new(
		None,
		"canvas_b",
		Extent2::new(2u32, 2u32),
		vec![I::new(255), I::new(128), I::new(64), I::new(32)],
	)));
	let g1 = LayerGroup::new(
		None,
		"group",
		ColorMode::I,
		vec![rc1.clone(), rc2.clone()],
		Vec2::new(0., 0.),
		Extent2::new(4u32, 4u32),
	);

	let c1 = if let LayerNode::CanvasI(node) = &**g1.children.get(0).unwrap() {
		node
	} else {
		panic!("Note a CanvasI");
	};

	let c2 = if let LayerNode::CanvasI(node) = &**g1.children.get(1).unwrap() {
		node
	} else {
		panic!("Note a CanvasI");
	};

	let (patch, _) = g1.move_layer(c2.id, 0).unwrap();
	let g2 = g1.patch(&patch).unwrap();

	assert_eq!(g2.children.len(), 2);
	assert_eq!(g2.children.get(0).unwrap().id(), c2.id);
	assert_eq!(g2.children.get(1).unwrap().id(), c1.id);
}

#[test]
fn it_patchs_child() {
	let rc1 = Rc::new(LayerNode::CanvasI(CanvasI::new(
		None,
		"canvas_a",
		Extent2::new(2u32, 2u32),
		vec![I::new(255), I::new(128), I::new(64), I::new(32)],
	)));
	let rc2 = Rc::new(LayerNode::CanvasI(CanvasI::new(
		None,
		"canvas_b",
		Extent2::new(2u32, 2u32),
		vec![I::new(32), I::new(64), I::new(128), I::new(255)],
	)));
	let g1 = LayerGroup::new(
		None,
		"group",
		ColorMode::I,
		vec![rc1.clone(), rc2.clone()],
		Vec2::new(0., 0.),
		Extent2::new(4u32, 4u32),
	);

	let (patch, _) = if let LayerNode::CanvasI(node) = &**g1.children.get(0).unwrap() {
		node.rename("canvas_aa").unwrap()
	} else {
		panic!("Note a CanvasI");
	};
	let g2 = g1.patch(&patch).unwrap();

	assert_eq!(Rc::strong_count(&rc1), 2);
	assert_eq!(Rc::strong_count(&rc2), 3);

	let c1 = if let LayerNode::CanvasI(node) = &**g2.children.get(0).unwrap() {
		node
	} else {
		panic!("Note a CanvasI");
	};

	let c2 = if let LayerNode::CanvasI(node) = &**g2.children.get(1).unwrap() {
		node
	} else {
		panic!("Note a CanvasI");
	};

	assert_eq!(*c1.name, "canvas_aa");
	assert_eq!(*c2.name, "canvas_b");

	let (patch, _) = g1.resize(Extent2::new(4, 1), Interpolation::Nearest);
	let g2 = g1.patch(&patch).unwrap();

	assert_eq!(*g2.size, Extent2::new(4, 1));

	let c1 = if let LayerNode::CanvasI(node) = &**g2.children.get(0).unwrap() {
		node
	} else {
		panic!("Note a CanvasI");
	};

	let c2 = if let LayerNode::CanvasI(node) = &**g2.children.get(1).unwrap() {
		node
	} else {
		panic!("Note a CanvasI");
	};

	assert_eq!(*c1.size, Extent2::new(4, 1));
	assert_eq!(*c2.size, Extent2::new(4, 1));
	assert_eq!(
		*c1.data,
		vec![I::new(255), I::new(255), I::new(64), I::new(64)]
	);
	assert_eq!(
		*c2.data,
		vec![I::new(32), I::new(32), I::new(128), I::new(128)]
	);
}
