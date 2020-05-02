use crate::color::ColorMode;
use crate::parser;
use crate::patch::{CropLayerError, Patch, Patchable, ResizeLayerError};
use crate::sprite::*;
use math::interpolation::*;
use math::{Extent2, Vec2};
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::io;
use uuid::Uuid;

pub trait Layer {
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

impl parser::v0::PartitionTableParse for LayerNode {
	type Output = LayerNode;

	fn parse<'b, S>(
		index: &parser::v0::PartitionIndex,
		row: &parser::v0::PartitionTableRow,
		storage: &mut S,
		bytes: &'b [u8],
	) -> IResult<&'b [u8], Self::Output>
	where
		S: io::Read + io::Seek,
	{
		match row.chunk_type {
			parser::v0::ChunkType::CanvasI => CanvasI::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, LayerNode::CanvasI(node))),
			parser::v0::ChunkType::CanvasIXYZ => CanvasIXYZ::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, LayerNode::CanvasIXYZ(node))),
			parser::v0::ChunkType::CanvasUV => CanvasUV::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, LayerNode::CanvasUV(node))),
			parser::v0::ChunkType::CanvasRGB => CanvasRGB::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, LayerNode::CanvasRGB(node))),
			parser::v0::ChunkType::CanvasRGBA => CanvasRGBA::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, LayerNode::CanvasRGBA(node))),
			parser::v0::ChunkType::CanvasRGBAXYZ => {
				CanvasRGBAXYZ::parse(index, row, storage, bytes)
					.map(|(bytes, node)| (bytes, LayerNode::CanvasRGBAXYZ(node)))
			}
			parser::v0::ChunkType::Sprite => Sprite::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, LayerNode::Sprite(node))),
			parser::v0::ChunkType::LayerGroup => LayerGroup::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, LayerNode::Group(node))),
			_ => unimplemented!(),
		}
	}

	fn write<S>(&self, index: &mut parser::v0::PartitionIndex, storage: &mut S) -> io::Result<usize>
	where
		S: io::Write + io::Seek,
	{
		match self {
			LayerNode::CanvasI(node) => node.write(index, storage),
			LayerNode::CanvasIXYZ(node) => node.write(index, storage),
			LayerNode::CanvasUV(node) => node.write(index, storage),
			LayerNode::CanvasRGB(node) => node.write(index, storage),
			LayerNode::CanvasRGBA(node) => node.write(index, storage),
			LayerNode::CanvasRGBAXYZ(node) => node.write(index, storage),
			LayerNode::Sprite(node) => node.write(index, storage),
			LayerNode::Group(node) => node.write(index, storage),
			_ => unimplemented!(),
		}
	}
}
