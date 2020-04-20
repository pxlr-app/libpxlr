use math::Vec2;
use uuid::Uuid;

use crate::patch::{Patchable, Patch};
use crate::group::Group;
use crate::node::Node;
use crate::note::Note;
use crate::sprite::Sprite;

pub trait Document: Node {
	fn position(&self) -> Vec2<f32>;
}

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
			DocumentNode::Sprite(node) => node.patch(&patch).map(|node| DocumentNode::Sprite(*node)),
		}
	}
}