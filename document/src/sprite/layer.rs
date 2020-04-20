use math::{Extent2, Vec2};
use uuid::Uuid;

use crate::patch::{Patchable, Patch};
use crate::node::Node;
use crate::sprite::{Canvas, Interpolation, LayerGroup, Sprite};

pub trait Layer: Node {
	fn crop(&self, offset: Vec2<u32>, size: Extent2<u32>) -> (Patch, Patch);
	fn resize(&self, size: Extent2<u32>, interpolation: Interpolation) -> (Patch, Patch);
}

pub enum LayerNode {
	Canvas(Canvas),
	Group(LayerGroup),
	Sprite(Sprite),
}

impl LayerNode {
	pub fn id(&self) -> Uuid {
		match self {
			LayerNode::Canvas(node) => node.id,
			LayerNode::Group(node) => node.id,
			LayerNode::Sprite(node) => node.id,
		}
	}

	pub fn patch(&self, patch: &Patch) -> Option<LayerNode> {
		match self {
			LayerNode::Canvas(node) => node.patch(&patch).map(|node| LayerNode::Canvas(*node)),
			LayerNode::Group(node) => node.patch(&patch).map(|node| LayerNode::Group(*node)),
			LayerNode::Sprite(node) => node.patch(&patch).map(|node| LayerNode::Sprite(*node)),
		}
	}

	pub fn crop(&self, offset: Vec2<u32>, size: Extent2<u32>) -> (Patch, Patch) {
		match self {
			LayerNode::Canvas(node) => node.crop(offset, size),
			LayerNode::Group(node) => node.crop(offset, size),
			LayerNode::Sprite(node) => node.crop(offset, size),
		}
	}

	pub fn resize(&self, size: Extent2<u32>, interpolation: Interpolation) -> (Patch, Patch) {
		match self {
			LayerNode::Canvas(node) => node.resize(size, interpolation),
			LayerNode::Group(node) => node.resize(size, interpolation),
			LayerNode::Sprite(node) => node.resize(size, interpolation),
		}
	}
}