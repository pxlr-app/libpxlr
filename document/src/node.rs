use crate::sprite::{CanvasPalette, CanvasRGBA, CanvasUV, LayerGroup, Sprite};
use crate::{Group, Note};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait INode {
	fn is_visible(&self) -> bool {
		true
	}
	fn is_locked(&self) -> bool {
		false
	}
	fn is_folded(&self) -> bool {
		false
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Node {
	Unknown,
	Group(Group),
	Note(Note),
	Sprite(Sprite),
	LayerGroup(LayerGroup),
	CanvasPalette(CanvasPalette),
	CanvasRGBA(CanvasRGBA),
	CanvasUV(CanvasUV),
}

impl Node {
	pub fn id(&self) -> Uuid {
		match self {
			Node::Unknown => Uuid::nil(),
			Node::Note(node) => node.id,
			Node::Group(node) => node.id,
			Node::Sprite(node) => node.id,
			Node::LayerGroup(node) => node.id,
			Node::CanvasPalette(node) => node.id,
			Node::CanvasRGBA(node) => node.id,
			Node::CanvasUV(node) => node.id,
		}
	}
}

impl INode for Node {
	fn is_visible(&self) -> bool {
		match self {
			Node::Unknown => false,
			Node::Note(node) => node.is_visible(),
			Node::Group(node) => node.is_visible(),
			Node::Sprite(node) => node.is_visible(),
			Node::LayerGroup(node) => node.is_visible(),
			Node::CanvasPalette(node) => node.is_visible(),
			Node::CanvasRGBA(node) => node.is_visible(),
			Node::CanvasUV(node) => node.is_visible(),
		}
	}
	fn is_locked(&self) -> bool {
		match self {
			Node::Unknown => false,
			Node::Note(node) => node.is_locked(),
			Node::Group(node) => node.is_locked(),
			Node::Sprite(node) => node.is_locked(),
			Node::LayerGroup(node) => node.is_locked(),
			Node::CanvasPalette(node) => node.is_locked(),
			Node::CanvasRGBA(node) => node.is_locked(),
			Node::CanvasUV(node) => node.is_locked(),
		}
	}
	fn is_folded(&self) -> bool {
		match self {
			Node::Unknown => false,
			Node::Note(node) => node.is_folded(),
			Node::Group(node) => node.is_folded(),
			Node::Sprite(node) => node.is_folded(),
			Node::LayerGroup(node) => node.is_folded(),
			Node::CanvasPalette(node) => node.is_folded(),
			Node::CanvasRGBA(node) => node.is_folded(),
			Node::CanvasUV(node) => node.is_folded(),
		}
	}
}
