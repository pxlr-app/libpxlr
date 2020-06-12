use crate::color::ColorMode;
use crate::patch::{CropLayerError, IPatchable, Patch, ResizeLayerError};
use crate::sprite::*;
use crate::{INode, Node};
use math::interpolation::*;
use math::{Extent2, Vec2};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait ILayer: INode {
	fn crop(&self, offset: Vec2<u32>, size: Extent2<u32>)
		-> Result<(Patch, Patch), CropLayerError>;
	fn resize(
		&self,
		size: Extent2<u32>,
		interpolation: Interpolation,
	) -> Result<(Patch, Patch), ResizeLayerError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LayerNode {
	CanvasPalette(CanvasPalette),
	CanvasRGBA(CanvasRGBA),
	CanvasUV(CanvasUV),
	Group(LayerGroup),
}

impl LayerNode {
	pub fn id(&self) -> Uuid {
		match self {
			LayerNode::CanvasPalette(node) => node.id,
			LayerNode::CanvasRGBA(node) => node.id,
			LayerNode::CanvasUV(node) => node.id,
			LayerNode::Group(node) => node.id,
		}
	}

	pub fn color_mode(&self) -> ColorMode {
		match self {
			LayerNode::CanvasPalette(_) => ColorMode::Palette,
			LayerNode::CanvasRGBA(_) => ColorMode::RGBA,
			LayerNode::CanvasUV(_) => ColorMode::UV,
			LayerNode::Group(node) => node.color_mode,
		}
	}

	pub fn patch(&self, patch: &Patch) -> Option<LayerNode> {
		match self {
			LayerNode::CanvasPalette(node) => node
				.patch(&patch)
				.map(|node| LayerNode::CanvasPalette(*node)),
			LayerNode::CanvasRGBA(node) => {
				node.patch(&patch).map(|node| LayerNode::CanvasRGBA(*node))
			}
			LayerNode::CanvasUV(node) => node.patch(&patch).map(|node| LayerNode::CanvasUV(*node)),
			LayerNode::Group(node) => node.patch(&patch).map(|node| LayerNode::Group(*node)),
		}
	}

	pub fn crop(
		&self,
		offset: Vec2<u32>,
		size: Extent2<u32>,
	) -> Result<(Patch, Patch), CropLayerError> {
		match self {
			LayerNode::CanvasPalette(node) => node.crop(offset, size),
			LayerNode::CanvasRGBA(node) => node.crop(offset, size),
			LayerNode::CanvasUV(node) => node.crop(offset, size),
			LayerNode::Group(node) => node.crop(offset, size),
		}
	}

	pub fn resize(
		&self,
		size: Extent2<u32>,
		interpolation: Interpolation,
	) -> Result<(Patch, Patch), ResizeLayerError> {
		match self {
			LayerNode::CanvasPalette(node) => node.resize(size, interpolation),
			LayerNode::CanvasRGBA(node) => node.resize(size, interpolation),
			LayerNode::CanvasUV(node) => node.resize(size, interpolation),
			LayerNode::Group(node) => node.resize(size, interpolation),
		}
	}
}

impl INode for LayerNode {
	fn is_visible(&self) -> bool {
		match self {
			LayerNode::CanvasPalette(node) => node.is_visible(),
			LayerNode::CanvasRGBA(node) => node.is_visible(),
			LayerNode::CanvasUV(node) => node.is_visible(),
			LayerNode::Group(node) => node.is_visible(),
		}
	}
	fn is_locked(&self) -> bool {
		match self {
			LayerNode::CanvasPalette(node) => node.is_locked(),
			LayerNode::CanvasRGBA(node) => node.is_locked(),
			LayerNode::CanvasUV(node) => node.is_locked(),
			LayerNode::Group(node) => node.is_locked(),
		}
	}
	fn is_folded(&self) -> bool {
		match self {
			LayerNode::CanvasPalette(node) => node.is_folded(),
			LayerNode::CanvasRGBA(node) => node.is_folded(),
			LayerNode::CanvasUV(node) => node.is_folded(),
			LayerNode::Group(node) => node.is_folded(),
		}
	}
}

impl std::convert::TryFrom<Node> for LayerNode {
	type Error = &'static str;

	fn try_from(node: Node) -> Result<Self, Self::Error> {
		match node {
			Node::CanvasPalette(node) => Ok(LayerNode::CanvasPalette(node)),
			Node::CanvasRGBA(node) => Ok(LayerNode::CanvasRGBA(node)),
			Node::CanvasUV(node) => Ok(LayerNode::CanvasUV(node)),
			Node::LayerGroup(node) => Ok(LayerNode::Group(node)),
			_ => Err("Node is not a valid LayerNode."),
		}
	}
}
