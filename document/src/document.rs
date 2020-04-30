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

	fn parse<'a, 'b>(
		file: &mut parser::v0::Database<'a>,
		row: &parser::v0::PartitionTableRow,
		bytes: &'b [u8],
	) -> IResult<&'b [u8], Self::Output> {
		match row.chunk_type {
			parser::v0::ChunkType::Group => Group::parse(file, row, bytes)
				.map(|(bytes, node)| (bytes, DocumentNode::Group(node))),
			parser::v0::ChunkType::Note => {
				Note::parse(file, row, bytes).map(|(bytes, node)| (bytes, DocumentNode::Note(node)))
			}
			_ => unimplemented!(),
		}
	}

	fn write<'a, W: io::Write + io::Seek>(
		&self,
		file: &mut parser::v0::Database<'a>,
		writer: &mut W,
	) -> io::Result<usize> {
		match self {
			DocumentNode::Group(node) => node.write(file, writer),
			DocumentNode::Note(node) => node.write(file, writer),
			_ => unimplemented!(),
		}
	}
}
