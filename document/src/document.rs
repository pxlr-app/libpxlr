use crate::parser;
use crate::patch::{IPatchable, Patch};
use crate::sprite::Sprite;
use crate::Group;
use crate::INode;
use crate::Note;
use math::Vec2;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::io;
use uuid::Uuid;

pub trait IDocument {
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

impl INode for DocumentNode {
	fn is_visible(&self) -> bool {
		match self {
			DocumentNode::Note(node) => node.is_visible(),
			DocumentNode::Group(node) => node.is_visible(),
			DocumentNode::Sprite(node) => node.is_visible(),
		}
	}
	fn is_locked(&self) -> bool {
		match self {
			DocumentNode::Note(node) => node.is_locked(),
			DocumentNode::Group(node) => node.is_locked(),
			DocumentNode::Sprite(node) => node.is_locked(),
		}
	}
	fn is_folded(&self) -> bool {
		match self {
			DocumentNode::Note(node) => node.is_folded(),
			DocumentNode::Group(node) => node.is_folded(),
			DocumentNode::Sprite(node) => node.is_folded(),
		}
	}
}

impl parser::v0::IParser for DocumentNode {
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

	fn write<S>(
		&self,
		index: &mut parser::v0::PartitionIndex,
		storage: &mut S,
		offset: u64,
	) -> io::Result<usize>
	where
		S: io::Write,
	{
		match self {
			DocumentNode::Group(node) => node.write(index, storage, offset),
			DocumentNode::Note(node) => node.write(index, storage, offset),
			_ => unimplemented!(),
		}
	}
}
