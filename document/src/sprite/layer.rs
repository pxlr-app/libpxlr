use crate::color::AlbedoMode;
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
	CanvasRGB(CanvasRGB),
	CanvasUV(CanvasUV),
	Group(LayerGroup),
	Sprite(Sprite),
}

impl LayerNode {
	pub fn id(&self) -> Uuid {
		match self {
			LayerNode::CanvasPalette(node) => node.id,
			LayerNode::CanvasRGB(node) => node.id,
			LayerNode::CanvasUV(node) => node.id,
			LayerNode::Group(node) => node.id,
			LayerNode::Sprite(node) => node.id,
		}
	}

	pub fn albedo_mode(&self) -> AlbedoMode {
		match self {
			LayerNode::CanvasPalette(_) => AlbedoMode::Palette,
			LayerNode::CanvasRGB(_) => AlbedoMode::RGB,
			LayerNode::CanvasUV(_) => AlbedoMode::UV,
			LayerNode::Group(node) => node.albedo_mode,
			LayerNode::Sprite(node) => node.albedo_mode,
		}
	}

	pub fn patch(&self, patch: &Patch) -> Option<LayerNode> {
		match self {
			LayerNode::CanvasPalette(node) => {
				node.patch(&patch).map(|node| LayerNode::CanvasPalette(*node))
			}
			LayerNode::CanvasRGB(node) => {
				node.patch(&patch).map(|node| LayerNode::CanvasRGB(*node))
			}
			LayerNode::CanvasUV(node) => node.patch(&patch).map(|node| LayerNode::CanvasUV(*node)),
			LayerNode::Group(node) => node.patch(&patch).map(|node| LayerNode::Group(*node)),
			LayerNode::Sprite(node) => node.patch(&patch).map(|node| LayerNode::Sprite(*node)),
		}
	}

	pub fn crop(
		&self,
		offset: Vec2<u32>,
		size: Extent2<u32>,
	) -> Result<(Patch, Patch), CropLayerError> {
		match self {
			LayerNode::CanvasPalette(node) => node.crop(offset, size),
			LayerNode::CanvasRGB(node) => node.crop(offset, size),
			LayerNode::CanvasUV(node) => node.crop(offset, size),
			LayerNode::Group(node) => node.crop(offset, size),
			LayerNode::Sprite(node) => node.crop(offset, size),
		}
	}

	pub fn resize(
		&self,
		size: Extent2<u32>,
		interpolation: Interpolation,
	) -> Result<(Patch, Patch), ResizeLayerError> {
		match self {
			LayerNode::CanvasPalette(node) => node.resize(size, interpolation),
			LayerNode::CanvasRGB(node) => node.resize(size, interpolation),
			LayerNode::CanvasUV(node) => node.resize(size, interpolation),
			LayerNode::Group(node) => node.resize(size, interpolation),
			LayerNode::Sprite(node) => node.resize(size, interpolation),
		}
	}
}

impl INode for LayerNode {
	fn is_visible(&self) -> bool {
		match self {
			LayerNode::CanvasPalette(node) => node.is_visible(),
			LayerNode::CanvasRGB(node) => node.is_visible(),
			LayerNode::CanvasUV(node) => node.is_visible(),
			LayerNode::Group(node) => node.is_visible(),
			LayerNode::Sprite(node) => node.is_visible(),
		}
	}
	fn is_locked(&self) -> bool {
		match self {
			LayerNode::CanvasPalette(node) => node.is_locked(),
			LayerNode::CanvasRGB(node) => node.is_locked(),
			LayerNode::CanvasUV(node) => node.is_locked(),
			LayerNode::Group(node) => node.is_locked(),
			LayerNode::Sprite(node) => node.is_locked(),
		}
	}
	fn is_folded(&self) -> bool {
		match self {
			LayerNode::CanvasPalette(node) => node.is_folded(),
			LayerNode::CanvasRGB(node) => node.is_folded(),
			LayerNode::CanvasUV(node) => node.is_folded(),
			LayerNode::Group(node) => node.is_folded(),
			LayerNode::Sprite(node) => node.is_folded(),
		}
	}
}

impl std::convert::TryFrom<Node> for LayerNode {
	type Error = &'static str;

	fn try_from(node: Node) -> Result<Self, Self::Error> {
		match node {
			Node::CanvasPalette(node) => Ok(LayerNode::CanvasPalette(node)),
			Node::CanvasRGB(node) => Ok(LayerNode::CanvasRGB(node)),
			Node::CanvasUV(node) => Ok(LayerNode::CanvasUV(node)),
			Node::Sprite(node) => Ok(LayerNode::Sprite(node)),
			_ => Err("Node is not a valid LayerNode."),
		}
	}
}
