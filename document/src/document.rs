use crate::group::Group;
use crate::note::Note;
use crate::patch::{Patch, Patchable};
use crate::sprite::Sprite;
use math::Vec2;
use serde::{Deserialize, Serialize};
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

// impl Writer for DocumentNode {
// 	fn write<W: std::io::Write + std::io::Seek>(
// 		&self,
// 		file: &mut crate::file::File,
// 		writer: &mut W,
// 	) -> std::io::Result<usize> {
// 		match self {
// 			DocumentNode::Note(node) => node.write(file, writer),
// 			DocumentNode::Group(node) => node.write(file, writer),
// 			DocumentNode::Sprite(node) => node.write(file, writer),
// 		}
// 	}
// }
