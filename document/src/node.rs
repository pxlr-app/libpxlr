use crate::sprite::{
	CanvasI, CanvasIXYZ, CanvasRGB, CanvasRGBA, CanvasRGBAXYZ, CanvasUV, LayerGroup,
};
use crate::{Group, Note};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum Node {
	Unknown,
	Group(Group),
	Note(Note),
	Sprite(LayerGroup),
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
			Node::CanvasI(node) => node.id,
			Node::CanvasIXYZ(node) => node.id,
			Node::CanvasUV(node) => node.id,
			Node::CanvasRGB(node) => node.id,
			Node::CanvasRGBA(node) => node.id,
			Node::CanvasRGBAXYZ(node) => node.id,
		}
	}
}
