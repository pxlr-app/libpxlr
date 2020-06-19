use crate::parser;
use async_trait::async_trait;
use document::{sprite::*, Group, Node, Note};
use futures::io;
use nom::IResult;

#[async_trait]
impl parser::v0::IParser for Node {
	type Output = Node;

	async fn parse<'b>(
		row: &parser::v0::PartitionTableRow,
		children: &mut Vec<Node>,
		bytes: &'b [u8],
	) -> IResult<&'b [u8], Self::Output> {
		match row.chunk_type {
			parser::v0::ChunkType::Group => Group::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, Node::Group(node))),
			parser::v0::ChunkType::Note => Note::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, Node::Note(node))),
			parser::v0::ChunkType::LayerGroup => LayerGroup::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, Node::LayerGroup(node))),
			parser::v0::ChunkType::CanvasGrey => CanvasGrey::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, Node::CanvasGrey(node))),
			parser::v0::ChunkType::CanvasRGBA => CanvasRGBA::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, Node::CanvasRGBA(node))),
			parser::v0::ChunkType::CanvasUV => CanvasUV::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, Node::CanvasUV(node))),
		}
	}
}

#[async_trait]
impl parser::v0::IWriter for Node {
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
			Node::Group(node) => node.write(index, storage, offset).await,
			Node::Note(node) => node.write(index, storage, offset).await,
			Node::LayerGroup(node) => node.write(index, storage, offset).await,
			Node::CanvasGrey(node) => node.write(index, storage, offset).await,
			Node::CanvasRGBA(node) => node.write(index, storage, offset).await,
			Node::CanvasUV(node) => node.write(index, storage, offset).await,
			_ => unimplemented!(),
		}
	}
}
