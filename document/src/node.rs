use crate::parser;
use crate::sprite::{
	CanvasI, CanvasIXYZ, CanvasRGB, CanvasRGBA, CanvasRGBAXYZ, CanvasUV, LayerGroup, Sprite,
};
use crate::{Group, Note};
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::io;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum Node {
	Unknown,
	Group(Group),
	Note(Note),
	Sprite(Sprite),
	LayerGroup(LayerGroup),
	CanvasI(CanvasI),
	CanvasIXYZ(CanvasIXYZ),
	CanvasUV(CanvasUV),
	CanvasRGB(CanvasRGB),
	CanvasRGBA(CanvasRGBA),
	CanvasRGBAXYZ(CanvasRGBAXYZ),
}

impl Node {
	pub fn id(&self) -> Uuid {
		match self {
			Node::Unknown => Uuid::nil(),
			Node::Note(node) => node.id,
			Node::Group(node) => node.id,
			Node::Sprite(node) => node.id,
			Node::LayerGroup(node) => node.id,
			Node::CanvasI(node) => node.id,
			Node::CanvasIXYZ(node) => node.id,
			Node::CanvasUV(node) => node.id,
			Node::CanvasRGB(node) => node.id,
			Node::CanvasRGBA(node) => node.id,
			Node::CanvasRGBAXYZ(node) => node.id,
		}
	}
}

impl parser::v0::PartitionTableParse for Node {
	type Output = Node;

	fn parse<'a, 'b>(
		file: &mut parser::v0::Database<'a>,
		row: &parser::v0::PartitionTableRow,
		bytes: &'b [u8],
	) -> IResult<&'b [u8], Self::Output> {
		match row.chunk_type {
			parser::v0::ChunkType::Group => {
				Group::parse(file, row, bytes).map(|(bytes, node)| (bytes, Node::Group(node)))
			}
			parser::v0::ChunkType::Note => {
				Note::parse(file, row, bytes).map(|(bytes, node)| (bytes, Node::Note(node)))
			}
			parser::v0::ChunkType::LayerGroup => LayerGroup::parse(file, row, bytes)
				.map(|(bytes, node)| (bytes, Node::LayerGroup(node))),
			parser::v0::ChunkType::Sprite => {
				Sprite::parse(file, row, bytes).map(|(bytes, node)| (bytes, Node::Sprite(node)))
			}
			parser::v0::ChunkType::CanvasI => {
				CanvasI::parse(file, row, bytes).map(|(bytes, node)| (bytes, Node::CanvasI(node)))
			}
			parser::v0::ChunkType::CanvasIXYZ => CanvasIXYZ::parse(file, row, bytes)
				.map(|(bytes, node)| (bytes, Node::CanvasIXYZ(node))),
			parser::v0::ChunkType::CanvasUV => {
				CanvasUV::parse(file, row, bytes).map(|(bytes, node)| (bytes, Node::CanvasUV(node)))
			}
			parser::v0::ChunkType::CanvasRGB => CanvasRGB::parse(file, row, bytes)
				.map(|(bytes, node)| (bytes, Node::CanvasRGB(node))),
			parser::v0::ChunkType::CanvasRGBA => CanvasRGBA::parse(file, row, bytes)
				.map(|(bytes, node)| (bytes, Node::CanvasRGBA(node))),
			parser::v0::ChunkType::CanvasRGBA => CanvasRGBA::parse(file, row, bytes)
				.map(|(bytes, node)| (bytes, Node::CanvasRGBA(node))),
			_ => unimplemented!(),
		}
	}

	fn write<'a, W: io::Write + io::Seek>(
		&self,
		file: &mut parser::v0::Database<'a>,
		writer: &mut W,
	) -> io::Result<usize> {
		match self {
			Node::Group(node) => node.write(file, writer),
			Node::Note(node) => node.write(file, writer),
			Node::Sprite(node) => node.write(file, writer),
			Node::LayerGroup(node) => node.write(file, writer),
			Node::CanvasI(node) => node.write(file, writer),
			Node::CanvasIXYZ(node) => node.write(file, writer),
			Node::CanvasUV(node) => node.write(file, writer),
			Node::CanvasRGB(node) => node.write(file, writer),
			Node::CanvasRGBA(node) => node.write(file, writer),
			Node::CanvasRGBAXYZ(node) => node.write(file, writer),
			_ => unimplemented!(),
		}
	}
}
