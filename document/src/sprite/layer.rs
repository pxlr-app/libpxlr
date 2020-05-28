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
	CanvasI(CanvasI),
	CanvasIXYZ(CanvasIXYZ),
	CanvasUV(CanvasUV),
	CanvasRGB(CanvasRGB),
	CanvasRGBA(CanvasRGBA),
	CanvasRGBAXYZ(CanvasRGBAXYZ),
	Group(LayerGroup),
	Sprite(Sprite),
}

impl LayerNode {
	pub fn id(&self) -> Uuid {
		match self {
			LayerNode::CanvasI(node) => node.id,
			LayerNode::CanvasIXYZ(node) => node.id,
			LayerNode::CanvasUV(node) => node.id,
			LayerNode::CanvasRGB(node) => node.id,
			LayerNode::CanvasRGBA(node) => node.id,
			LayerNode::CanvasRGBAXYZ(node) => node.id,
			LayerNode::Group(node) => node.id,
			LayerNode::Sprite(node) => node.id,
		}
	}

	pub fn color_mode(&self) -> ColorMode {
		match self {
			LayerNode::CanvasI(_) => ColorMode::I,
			LayerNode::CanvasIXYZ(_) => ColorMode::IXYZ,
			LayerNode::CanvasUV(_) => ColorMode::UV,
			LayerNode::CanvasRGB(_) => ColorMode::RGB,
			LayerNode::CanvasRGBA(_) => ColorMode::RGBA,
			LayerNode::CanvasRGBAXYZ(_) => ColorMode::RGBAXYZ,
			LayerNode::Group(node) => node.color_mode,
			LayerNode::Sprite(node) => node.color_mode,
		}
	}

	pub fn patch(&self, patch: &Patch) -> Option<LayerNode> {
		match self {
			LayerNode::CanvasI(node) => node.patch(&patch).map(|node| LayerNode::CanvasI(*node)),
			LayerNode::CanvasIXYZ(node) => {
				node.patch(&patch).map(|node| LayerNode::CanvasIXYZ(*node))
			}
			LayerNode::CanvasUV(node) => node.patch(&patch).map(|node| LayerNode::CanvasUV(*node)),
			LayerNode::CanvasRGB(node) => {
				node.patch(&patch).map(|node| LayerNode::CanvasRGB(*node))
			}
			LayerNode::CanvasRGBA(node) => {
				node.patch(&patch).map(|node| LayerNode::CanvasRGBA(*node))
			}
			LayerNode::CanvasRGBAXYZ(node) => node
				.patch(&patch)
				.map(|node| LayerNode::CanvasRGBAXYZ(*node)),
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
			LayerNode::CanvasI(node) => node.crop(offset, size),
			LayerNode::CanvasIXYZ(node) => node.crop(offset, size),
			LayerNode::CanvasUV(node) => node.crop(offset, size),
			LayerNode::CanvasRGB(node) => node.crop(offset, size),
			LayerNode::CanvasRGBA(node) => node.crop(offset, size),
			LayerNode::CanvasRGBAXYZ(node) => node.crop(offset, size),
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
			LayerNode::CanvasIXYZ(node) => node.resize(size, interpolation),
			LayerNode::CanvasI(node) => node.resize(size, interpolation),
			LayerNode::CanvasUV(node) => node.resize(size, interpolation),
			LayerNode::CanvasRGB(node) => node.resize(size, interpolation),
			LayerNode::CanvasRGBA(node) => node.resize(size, interpolation),
			LayerNode::CanvasRGBAXYZ(node) => node.resize(size, interpolation),
			LayerNode::Group(node) => node.resize(size, interpolation),
			LayerNode::Sprite(node) => node.resize(size, interpolation),
		}
	}
}

impl INode for LayerNode {
	fn is_visible(&self) -> bool {
		match self {
			LayerNode::CanvasI(node) => node.is_visible(),
			LayerNode::CanvasIXYZ(node) => node.is_visible(),
			LayerNode::CanvasUV(node) => node.is_visible(),
			LayerNode::CanvasRGB(node) => node.is_visible(),
			LayerNode::CanvasRGBA(node) => node.is_visible(),
			LayerNode::CanvasRGBAXYZ(node) => node.is_visible(),
			LayerNode::Group(node) => node.is_visible(),
			LayerNode::Sprite(node) => node.is_visible(),
		}
	}
	fn is_locked(&self) -> bool {
		match self {
			LayerNode::CanvasI(node) => node.is_locked(),
			LayerNode::CanvasIXYZ(node) => node.is_locked(),
			LayerNode::CanvasUV(node) => node.is_locked(),
			LayerNode::CanvasRGB(node) => node.is_locked(),
			LayerNode::CanvasRGBA(node) => node.is_locked(),
			LayerNode::CanvasRGBAXYZ(node) => node.is_locked(),
			LayerNode::Group(node) => node.is_locked(),
			LayerNode::Sprite(node) => node.is_locked(),
		}
	}
	fn is_folded(&self) -> bool {
		match self {
			LayerNode::CanvasI(node) => node.is_folded(),
			LayerNode::CanvasIXYZ(node) => node.is_folded(),
			LayerNode::CanvasUV(node) => node.is_folded(),
			LayerNode::CanvasRGB(node) => node.is_folded(),
			LayerNode::CanvasRGBA(node) => node.is_folded(),
			LayerNode::CanvasRGBAXYZ(node) => node.is_folded(),
			LayerNode::Group(node) => node.is_folded(),
			LayerNode::Sprite(node) => node.is_folded(),
		}
	}
}

impl std::convert::TryFrom<Node> for LayerNode {
	type Error = &'static str;

	fn try_from(node: Node) -> Result<Self, Self::Error> {
		match node {
			Node::CanvasI(node) => Ok(LayerNode::CanvasI(node)),
			Node::CanvasIXYZ(node) => Ok(LayerNode::CanvasIXYZ(node)),
			Node::CanvasUV(node) => Ok(LayerNode::CanvasUV(node)),
			Node::CanvasRGB(node) => Ok(LayerNode::CanvasRGB(node)),
			Node::CanvasRGBA(node) => Ok(LayerNode::CanvasRGBA(node)),
			Node::CanvasRGBAXYZ(node) => Ok(LayerNode::CanvasRGBAXYZ(node)),
			Node::Sprite(node) => Ok(LayerNode::Sprite(node)),
			_ => Err("Node is not a valid LayerNode."),
		}
	}
}
