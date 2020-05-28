use crate::sprite::{
	CanvasI, CanvasIXYZ, CanvasRGB, CanvasRGBA, CanvasRGBAXYZ, CanvasUV, LayerGroup, Sprite,
};
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
	CanvasI(CanvasI),
	CanvasIXYZ(CanvasIXYZ),
	CanvasUV(CanvasUV),
	CanvasRGB(CanvasRGB),
	CanvasRGBA(CanvasRGBA),
	CanvasRGBAXYZ(CanvasRGBAXYZ),
}

impl Node {
	pub fn id(&self) -> Uuid {
		match self {
			Node::Unknown => Uuid::nil(),
			Node::Note(node) => node.id,
			Node::Group(node) => node.id,
			Node::Sprite(node) => node.id,
			Node::LayerGroup(node) => node.id,
			Node::CanvasI(node) => node.id,
			Node::CanvasIXYZ(node) => node.id,
			Node::CanvasUV(node) => node.id,
			Node::CanvasRGB(node) => node.id,
			Node::CanvasRGBA(node) => node.id,
			Node::CanvasRGBAXYZ(node) => node.id,
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
			Node::CanvasI(node) => node.is_visible(),
			Node::CanvasIXYZ(node) => node.is_visible(),
			Node::CanvasUV(node) => node.is_visible(),
			Node::CanvasRGB(node) => node.is_visible(),
			Node::CanvasRGBA(node) => node.is_visible(),
			Node::CanvasRGBAXYZ(node) => node.is_visible(),
		}
	}
	fn is_locked(&self) -> bool {
		match self {
			Node::Unknown => false,
			Node::Note(node) => node.is_locked(),
			Node::Group(node) => node.is_locked(),
			Node::Sprite(node) => node.is_locked(),
			Node::LayerGroup(node) => node.is_locked(),
			Node::CanvasI(node) => node.is_locked(),
			Node::CanvasIXYZ(node) => node.is_locked(),
			Node::CanvasUV(node) => node.is_locked(),
			Node::CanvasRGB(node) => node.is_locked(),
			Node::CanvasRGBA(node) => node.is_locked(),
			Node::CanvasRGBAXYZ(node) => node.is_locked(),
		}
	}
	fn is_folded(&self) -> bool {
		match self {
			Node::Unknown => false,
			Node::Note(node) => node.is_folded(),
			Node::Group(node) => node.is_folded(),
			Node::Sprite(node) => node.is_folded(),
			Node::LayerGroup(node) => node.is_folded(),
			Node::CanvasI(node) => node.is_folded(),
			Node::CanvasIXYZ(node) => node.is_folded(),
			Node::CanvasUV(node) => node.is_folded(),
			Node::CanvasRGB(node) => node.is_folded(),
			Node::CanvasRGBA(node) => node.is_folded(),
			Node::CanvasRGBAXYZ(node) => node.is_folded(),
		}
	}
}
