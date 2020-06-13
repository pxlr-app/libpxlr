use crate::parser;
use async_trait::async_trait;
use document::{sprite::LayerGroup, DocumentNode, Group, Node, Note};
use futures::io;
use nom::IResult;

#[async_trait]
impl parser::v0::IParser for DocumentNode {
	type Output = DocumentNode;

	async fn parse<'b>(
		row: &parser::v0::PartitionTableRow,
		children: &mut Vec<Node>,
		bytes: &'b [u8],
	) -> IResult<&'b [u8], Self::Output> {
		match row.chunk_type {
			parser::v0::ChunkType::Group => Group::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, DocumentNode::Group(node))),
			parser::v0::ChunkType::Note => Note::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, DocumentNode::Note(node))),
			parser::v0::ChunkType::LayerGroup => LayerGroup::parse(row, children, bytes)
				.await
				.map(|(bytes, node)| (bytes, DocumentNode::Sprite(node))),
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
			DocumentNode::Group(node) => node.write(index, storage, offset).await,
			DocumentNode::Note(node) => node.write(index, storage, offset).await,
			DocumentNode::Sprite(node) => node.write(index, storage, offset).await,
		}
	}
}
