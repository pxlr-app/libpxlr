use crate::patch::{IPatchable, Patch};
use crate::sprite::Sprite;
use crate::{Group, INode, Node, Note};
use math::Vec2;
use serde::{Deserialize, Serialize};
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

impl std::convert::TryFrom<Node> for DocumentNode {
	type Error = &'static str;

	fn try_from(node: Node) -> Result<Self, Self::Error> {
		match node {
			Node::Group(node) => Ok(DocumentNode::Group(node)),
			Node::Note(node) => Ok(DocumentNode::Note(node)),
			Node::Sprite(node) => Ok(DocumentNode::Sprite(node)),
			Node::LayerGroup(node) => Ok(DocumentNode::Sprite(node)),
			_ => Err("Node is not a valid DocumentNode."),
		}
	}
}
