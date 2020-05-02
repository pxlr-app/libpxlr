use crate::group::Group;
use crate::note::Note;
use crate::parser;
use crate::patch::{Patch, Patchable};
use crate::sprite::Sprite;
use math::Vec2;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::io;
use uuid::Uuid;

pub trait Document {
	fn position(&self) -> Vec2<f32>;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DocumentNode {
	Note(Note),
	Group(Group),
	Sprite(Sprite),
}

impl DocumentNode {
	pub fn id(&self) -> Uuid {
		match self {
			DocumentNode::Note(node) => node.id,
			DocumentNode::Group(node) => node.id,
			DocumentNode::Sprite(node) => node.id,
		}
	}

	pub fn patch(&self, patch: &Patch) -> Option<DocumentNode> {
		match self {
			DocumentNode::Note(node) => node.patch(&patch).map(|node| DocumentNode::Note(*node)),
			DocumentNode::Group(node) => node.patch(&patch).map(|node| DocumentNode::Group(*node)),
			DocumentNode::Sprite(node) => {
				node.patch(&patch).map(|node| DocumentNode::Sprite(*node))
			}
		}
	}
}

impl parser::v0::PartitionTableParse for DocumentNode {
	type Output = DocumentNode;

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
				.map(|(bytes, node)| (bytes, DocumentNode::Group(node))),
			parser::v0::ChunkType::Note => Note::parse(index, row, storage, bytes)
				.map(|(bytes, node)| (bytes, DocumentNode::Note(node))),
			_ => unimplemented!(),
		}
	}

	fn write<S>(&self, index: &mut parser::v0::PartitionIndex, storage: &mut S) -> io::Result<usize>
	where
		S: io::Write + io::Seek,
	{
		match self {
			DocumentNode::Group(node) => node.write(index, storage),
			DocumentNode::Note(node) => node.write(index, storage),
			_ => unimplemented!(),
		}
	}
}
