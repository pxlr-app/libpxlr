use crate::file::writer::Writer;
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

impl Writer for Node {
	fn write<W: std::io::Write + std::io::Seek>(
		&self,
		file: &mut crate::file::File,
		writer: &mut W,
	) -> std::io::Result<usize> {
		match self {
			Node::Unknown => Ok(0),
			Node::Note(node) => node.write(file, writer),
			Node::Group(node) => node.write(file, writer),
			Node::Sprite(node) => node.write(file, writer),
			Node::CanvasI(node) => node.write(file, writer),
			Node::CanvasIXYZ(node) => node.write(file, writer),
			Node::CanvasUV(node) => node.write(file, writer),
			Node::CanvasRGB(node) => node.write(file, writer),
			Node::CanvasRGBA(node) => node.write(file, writer),
			Node::CanvasRGBAXYZ(node) => node.write(file, writer),
		}
	}
}
