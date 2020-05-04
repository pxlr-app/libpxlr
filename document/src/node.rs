use crate::parser;
use crate::sprite::{
	CanvasI, CanvasIXYZ, CanvasRGB, CanvasRGBA, CanvasRGBAXYZ, CanvasUV, LayerGroup, Sprite,
};
use crate::{Group, Note};
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::io;
use uuid::Uuid;

pub trait INode {
	fn is_visible(&self) -> bool {
		true
	}
	fn is_locked(&self) -> bool {
		false
	}
	fn is_folded(&self) -> bool {
		false
	}
}

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

impl INode for Node {
	fn is_visible(&self) -> bool {
		match self {
			Node::Unknown => false,
			Node::Note(node) => node.is_visible(),
			Node::Group(node) => node.is_visible(),
			Node::Sprite(node) => node.is_visible(),
			Node::LayerGroup(node) => node.is_visible(),
			Node::CanvasI(node) => node.is_visible(),
			Node::CanvasIXYZ(node) => node.is_visible(),
			Node::CanvasUV(node) => node.is_visible(),
			Node::CanvasRGB(node) => node.is_visible(),
			Node::CanvasRGBA(node) => node.is_visible(),
			Node::CanvasRGBAXYZ(node) => node.is_visible(),
		}
	}
	fn is_locked(&self) -> bool {
		match self {
			Node::Unknown => false,
			Node::Note(node) => node.is_locked(),
			Node::Group(node) => node.is_locked(),
			Node::Sprite(node) => node.is_locked(),
			Node::LayerGroup(node) => node.is_locked(),
			Node::CanvasI(node) => node.is_locked(),
			Node::CanvasIXYZ(node) => node.is_locked(),
			Node::CanvasUV(node) => node.is_locked(),
			Node::CanvasRGB(node) => node.is_locked(),
			Node::CanvasRGBA(node) => node.is_locked(),
			Node::CanvasRGBAXYZ(node) => node.is_locked(),
		}
	}
	fn is_folded(&self) -> bool {
		match self {
			Node::Unknown => false,
			Node::Note(node) => node.is_folded(),
			Node::Group(node) => node.is_folded(),
			Node::Sprite(node) => node.is_folded(),
			Node::LayerGroup(node) => node.is_folded(),
			Node::CanvasI(node) => node.is_folded(),
			Node::CanvasIXYZ(node) => node.is_folded(),
			Node::CanvasUV(node) => node.is_folded(),
			Node::CanvasRGB(node) => node.is_folded(),
			Node::CanvasRGBA(node) => node.is_folded(),
			Node::CanvasRGBAXYZ(node) => node.is_folded(),
		}
	}
}

impl parser::v0::PartitionTableParse for Node {
	type Output = Node;

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
			parser::v0::ChunkType::Group => Group::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, Node::Group(node))),
			parser::v0::ChunkType::Note => Note::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, Node::Note(node))),
			parser::v0::ChunkType::LayerGroup => LayerGroup::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, Node::LayerGroup(node))),
			parser::v0::ChunkType::Sprite => Sprite::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, Node::Sprite(node))),
			parser::v0::ChunkType::CanvasI => CanvasI::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, Node::CanvasI(node))),
			parser::v0::ChunkType::CanvasIXYZ => CanvasIXYZ::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, Node::CanvasIXYZ(node))),
			parser::v0::ChunkType::CanvasUV => CanvasUV::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, Node::CanvasUV(node))),
			parser::v0::ChunkType::CanvasRGB => CanvasRGB::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, Node::CanvasRGB(node))),
			parser::v0::ChunkType::CanvasRGBA => CanvasRGBA::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, Node::CanvasRGBA(node))),
			parser::v0::ChunkType::CanvasRGBA => CanvasRGBA::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, Node::CanvasRGBA(node))),
			_ => unimplemented!(),
		}
	}

	fn write<S>(&self, index: &mut parser::v0::PartitionIndex, storage: &mut S) -> io::Result<usize>
	where
		S: io::Write + io::Seek,
	{
		match self {
			Node::Group(node) => node.write(index, storage),
			Node::Note(node) => node.write(index, storage),
			Node::Sprite(node) => node.write(index, storage),
			Node::LayerGroup(node) => node.write(index, storage),
			Node::CanvasI(node) => node.write(index, storage),
			Node::CanvasIXYZ(node) => node.write(index, storage),
			Node::CanvasUV(node) => node.write(index, storage),
			Node::CanvasRGB(node) => node.write(index, storage),
			Node::CanvasRGBA(node) => node.write(index, storage),
			Node::CanvasRGBAXYZ(node) => node.write(index, storage),
			_ => unimplemented!(),
		}
	}
}
