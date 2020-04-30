use crate::parser;
use crate::sprite::{
	CanvasI, CanvasIXYZ, CanvasRGB, CanvasRGBA, CanvasRGBAXYZ, CanvasUV, LayerGroup, Sprite,
};
use crate::{Group, Note};
use async_std::io;
use async_trait::async_trait;
use nom::IResult;
use serde::{Deserialize, Serialize};
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

#[async_trait(?Send)]
impl<S> parser::v0::PartitionTableParse<S> for Node
where
	S: io::Read + io::Write + io::Seek + std::marker::Unpin,
{
	type Output = Node;

	async fn parse<'b>(
		index: &parser::v0::PartitionIndex,
		row: &parser::v0::PartitionTableRow,
		storage: &mut S,
		bytes: &'b [u8],
	) -> IResult<&'b [u8], Self::Output> {
		match row.chunk_type {
			parser::v0::ChunkType::Group => Group::parse(index, row, storage, bytes)
				.await
				.map(|(bytes, node)| (bytes, Node::Group(node))),
			parser::v0::ChunkType::Note => Note::parse(index, row, storage, bytes)
				.await
				.map(|(bytes, node)| (bytes, Node::Note(node))),
			parser::v0::ChunkType::LayerGroup => LayerGroup::parse(index, row, storage, bytes)
				.await
				.map(|(bytes, node)| (bytes, Node::LayerGroup(node))),
			parser::v0::ChunkType::Sprite => Sprite::parse(index, row, storage, bytes)
				.await
				.map(|(bytes, node)| (bytes, Node::Sprite(node))),
			parser::v0::ChunkType::CanvasI => CanvasI::parse(index, row, storage, bytes)
				.await
				.map(|(bytes, node)| (bytes, Node::CanvasI(node))),
			parser::v0::ChunkType::CanvasIXYZ => CanvasIXYZ::parse(index, row, storage, bytes)
				.await
				.map(|(bytes, node)| (bytes, Node::CanvasIXYZ(node))),
			parser::v0::ChunkType::CanvasUV => CanvasUV::parse(index, row, storage, bytes)
				.await
				.map(|(bytes, node)| (bytes, Node::CanvasUV(node))),
			parser::v0::ChunkType::CanvasRGB => CanvasRGB::parse(index, row, storage, bytes)
				.await
				.map(|(bytes, node)| (bytes, Node::CanvasRGB(node))),
			parser::v0::ChunkType::CanvasRGBA => CanvasRGBA::parse(index, row, storage, bytes)
				.await
				.map(|(bytes, node)| (bytes, Node::CanvasRGBA(node))),
			parser::v0::ChunkType::CanvasRGBA => CanvasRGBA::parse(index, row, storage, bytes)
				.await
				.map(|(bytes, node)| (bytes, Node::CanvasRGBA(node))),
			_ => unimplemented!(),
		}
	}

	async fn write(
		&self,
		index: &mut parser::v0::PartitionIndex,
		storage: &mut S,
	) -> io::Result<usize> {
		match self {
			Node::Group(node) => node.write(index, storage).await,
			Node::Note(node) => node.write(index, storage).await,
			Node::Sprite(node) => node.write(index, storage).await,
			Node::LayerGroup(node) => node.write(index, storage).await,
			Node::CanvasI(node) => node.write(index, storage).await,
			Node::CanvasIXYZ(node) => node.write(index, storage).await,
			Node::CanvasUV(node) => node.write(index, storage).await,
			Node::CanvasRGB(node) => node.write(index, storage).await,
			Node::CanvasRGBA(node) => node.write(index, storage).await,
			Node::CanvasRGBAXYZ(node) => node.write(index, storage).await,
			_ => unimplemented!(),
		}
	}
}
