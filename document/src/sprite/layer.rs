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

	fn parse<'a, 'b>(
		file: &mut parser::v0::Database<'a>,
		row: &parser::v0::PartitionTableRow,
		bytes: &'b [u8],
	) -> IResult<&'b [u8], Self::Output> {
		match row.chunk_type {
			parser::v0::ChunkType::CanvasI => CanvasI::parse(file, row, bytes)
				.map(|(bytes, node)| (bytes, LayerNode::CanvasI(node))),
			parser::v0::ChunkType::CanvasIXYZ => CanvasIXYZ::parse(file, row, bytes)
				.map(|(bytes, node)| (bytes, LayerNode::CanvasIXYZ(node))),
			parser::v0::ChunkType::CanvasUV => CanvasUV::parse(file, row, bytes)
				.map(|(bytes, node)| (bytes, LayerNode::CanvasUV(node))),
			parser::v0::ChunkType::CanvasRGB => CanvasRGB::parse(file, row, bytes)
				.map(|(bytes, node)| (bytes, LayerNode::CanvasRGB(node))),
			parser::v0::ChunkType::CanvasRGBA => CanvasRGBA::parse(file, row, bytes)
				.map(|(bytes, node)| (bytes, LayerNode::CanvasRGBA(node))),
			parser::v0::ChunkType::CanvasRGBAXYZ => CanvasRGBAXYZ::parse(file, row, bytes)
				.map(|(bytes, node)| (bytes, LayerNode::CanvasRGBAXYZ(node))),
			parser::v0::ChunkType::Sprite => Sprite::parse(file, row, bytes)
				.map(|(bytes, node)| (bytes, LayerNode::Sprite(node))),
			parser::v0::ChunkType::LayerGroup => LayerGroup::parse(file, row, bytes)
				.map(|(bytes, node)| (bytes, LayerNode::Group(node))),
			_ => unimplemented!(),
		}
	}

	fn write<'a, W: io::Write + io::Seek>(
		&self,
		file: &mut parser::v0::Database<'a>,
		writer: &mut W,
	) -> io::Result<usize> {
		match self {
			LayerNode::CanvasI(node) => node.write(file, writer),
			LayerNode::CanvasIXYZ(node) => node.write(file, writer),
			LayerNode::CanvasUV(node) => node.write(file, writer),
			LayerNode::CanvasRGB(node) => node.write(file, writer),
			LayerNode::CanvasRGBA(node) => node.write(file, writer),
			LayerNode::CanvasRGBAXYZ(node) => node.write(file, writer),
			LayerNode::Sprite(node) => node.write(file, writer),
			LayerNode::Group(node) => node.write(file, writer),
			_ => unimplemented!(),
		}
	}
}
