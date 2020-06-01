use crate::parser;
use async_trait::async_trait;
use document::{
	sprite::{
		CanvasI, CanvasIXYZ, CanvasRGB, CanvasRGBA, CanvasRGBAXYZ, CanvasUV, LayerGroup, LayerNode,
		Sprite,
	},
	Node,
};
use futures::io;
use nom::IResult;

#[async_trait]
impl parser::v0::IParser for LayerNode {
	type Output = LayerNode;

	async fn parse<'b>(
		row: &parser::v0::PartitionTableRow,
		children: &mut Vec<Node>,
		bytes: &'b [u8],
	) -> IResult<&'b [u8], Self::Output> {
		match row.chunk_type {
			parser::v0::ChunkType::CanvasI => CanvasI::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, LayerNode::CanvasI(node))),
			parser::v0::ChunkType::CanvasIXYZ => CanvasIXYZ::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, LayerNode::CanvasIXYZ(node))),
			parser::v0::ChunkType::CanvasUV => CanvasUV::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, LayerNode::CanvasUV(node))),
			parser::v0::ChunkType::CanvasRGB => CanvasRGB::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, LayerNode::CanvasRGB(node))),
			parser::v0::ChunkType::CanvasRGBA => CanvasRGBA::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, LayerNode::CanvasRGBA(node))),
			parser::v0::ChunkType::CanvasRGBAXYZ => CanvasRGBAXYZ::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, LayerNode::CanvasRGBAXYZ(node))),
			parser::v0::ChunkType::Sprite => Sprite::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, LayerNode::Sprite(node))),
			parser::v0::ChunkType::LayerGroup => LayerGroup::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, LayerNode::Group(node))),
			_ => unimplemented!(),
		}
	}

	async fn write<S>(
		&self,
		index: &mut parser::v0::PartitionIndex,
		storage: &mut S,
		offset: u64,
	) -> io::Result<usize>
	where
		S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
	{
		match self {
			LayerNode::CanvasI(node) => node.write(index, storage, offset).await,
			LayerNode::CanvasIXYZ(node) => node.write(index, storage, offset).await,
			LayerNode::CanvasUV(node) => node.write(index, storage, offset).await,
			LayerNode::CanvasRGB(node) => node.write(index, storage, offset).await,
			LayerNode::CanvasRGBA(node) => node.write(index, storage, offset).await,
			LayerNode::CanvasRGBAXYZ(node) => node.write(index, storage, offset).await,
			LayerNode::Sprite(node) => node.write(index, storage, offset).await,
			LayerNode::Group(node) => node.write(index, storage, offset).await,
		}
	}
}
