use crate::parser;
use async_trait::async_trait;
use document::{sprite::*, Node};
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
			parser::v0::ChunkType::CanvasGrey => CanvasGrey::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, LayerNode::CanvasGrey(node))),
			parser::v0::ChunkType::CanvasRGBA => CanvasRGBA::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, LayerNode::CanvasRGBA(node))),
			parser::v0::ChunkType::CanvasUV => CanvasUV::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, LayerNode::CanvasUV(node))),
			parser::v0::ChunkType::LayerGroup => LayerGroup::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, LayerNode::Group(node))),
			_ => unimplemented!(),
		}
	}
}

#[async_trait]
impl parser::v0::IWriter for LayerNode {
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
			LayerNode::CanvasGrey(node) => node.write(index, storage, offset).await,
			LayerNode::CanvasRGBA(node) => node.write(index, storage, offset).await,
			LayerNode::CanvasUV(node) => node.write(index, storage, offset).await,
			LayerNode::Group(node) => node.write(index, storage, offset).await,
		}
	}
}
