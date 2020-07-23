use crate::prelude::*;
use math::{Extent2, Vec2};
use nom::number::complete::le_u16;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
mod canvas;
mod group;
mod note;
mod palette;
mod sprite;
pub use canvas::*;
pub use group::*;
pub use note::*;
pub use palette::*;
pub use sprite::*;

pub trait Node: Any + Debug + patch::Patchable {
	fn id(&self) -> Uuid;
}
impl Downcast for dyn Node {}

pub type NodeRef = Arc<NodeType>;
pub type NodeList = Vec<NodeRef>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
	Note(Note),
	Group(Group),
	Palette(Palette),
	Sprite(Sprite),
	Canvas(Canvas),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeKind {
	Note,
	Group,
	Palette,
	Sprite,
	Canvas,
}

impl parser::Parse for NodeKind {
	fn parse(bytes: &[u8]) -> nom::IResult<&[u8], NodeKind> {
		let (bytes, idx) = le_u16(bytes)?;
		match idx {
			0 => Ok((bytes, NodeKind::Group)),
			1 => Ok((bytes, NodeKind::Note)),
			2 => Ok((bytes, NodeKind::Palette)),
			3 => Ok((bytes, NodeKind::Sprite)),
			4 => Ok((bytes, NodeKind::Canvas)),
			_ => Err(nom::Err::Error((bytes, nom::error::ErrorKind::NoneOf))),
		}
	}
}

impl parser::Write for NodeKind {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		let idx: u16 = match self {
			NodeKind::Group => 0,
			NodeKind::Note => 1,
			NodeKind::Palette => 2,
			NodeKind::Sprite => 3,
			NodeKind::Canvas => 4,
		};
		writer.write_all(&idx.to_le_bytes())?;
		Ok(2)
	}
}

impl NodeType {
	pub fn as_node(&self) -> &dyn Node {
		match self {
			NodeType::Note(node) => node,
			NodeType::Group(node) => node,
			NodeType::Palette(node) => node,
			NodeType::Sprite(node) => node,
			NodeType::Canvas(node) => node,
		}
	}
	pub fn as_documentnode(&self) -> Option<&dyn DocumentNode> {
		match self {
			NodeType::Note(node) => Some(node),
			NodeType::Group(node) => Some(node),
			NodeType::Palette(node) => Some(node),
			NodeType::Sprite(node) => Some(node),
			_ => None,
		}
	}
	pub fn as_spritenode(&self) -> Option<&dyn SpriteNode> {
		match self {
			NodeType::Sprite(node) => Some(node),
			NodeType::Canvas(node) => Some(node),
			_ => None,
		}
	}
}

impl parser::v0::ParseNode for NodeType {
	fn parse_node<'bytes>(
		row: &parser::v0::IndexRow,
		items: NodeList,
		dependencies: NodeList,
		bytes: &'bytes [u8],
	) -> parser::Result<&'bytes [u8], NodeRef> {
		match row.chunk_type {
			NodeKind::Note => {
				<Note as parser::v0::ParseNode>::parse_node(row, items, dependencies, bytes)
			}
			NodeKind::Group => {
				<Group as parser::v0::ParseNode>::parse_node(row, items, dependencies, bytes)
			}
			NodeKind::Palette => {
				<Palette as parser::v0::ParseNode>::parse_node(row, items, dependencies, bytes)
			}
			NodeKind::Sprite => {
				<Sprite as parser::v0::ParseNode>::parse_node(row, items, dependencies, bytes)
			}
			NodeKind::Canvas => {
				<Canvas as parser::v0::ParseNode>::parse_node(row, items, dependencies, bytes)
			}
		}
	}
}

impl parser::v0::WriteNode for NodeType {
	fn write_node<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
		rows: &mut Vec<parser::v0::IndexRow>,
		dependencies: &mut Vec<NodeRef>,
	) -> io::Result<usize> {
		match self {
			NodeType::Note(node) => node.write_node(writer, rows, dependencies),
			NodeType::Group(node) => node.write_node(writer, rows, dependencies),
			NodeType::Palette(node) => node.write_node(writer, rows, dependencies),
			NodeType::Sprite(node) => node.write_node(writer, rows, dependencies),
			NodeType::Canvas(node) => node.write_node(writer, rows, dependencies),
		}
	}
}

pub trait Named {
	fn name(&self) -> String {
		"".into()
	}
	fn rename(&self, _name: String) -> Option<patch::PatchPair> {
		None
	}
}
pub trait Positioned {
	fn position(&self) -> Vec2<u32> {
		Vec2::new(0, 0)
	}
	fn translate(&self, _target: Vec2<u32>) -> Option<patch::PatchPair> {
		None
	}
}
pub trait Sized {
	fn size(&self) -> Extent2<u32> {
		Extent2::new(0, 0)
	}
	fn resize(&self, _target: Extent2<u32>) -> Option<patch::PatchPair> {
		None
	}
}
pub trait Displayed {
	fn display(&self) -> bool {
		true
	}
	fn set_display(&self, _display: bool) -> Option<patch::PatchPair> {
		None
	}
}
pub trait Locked {
	fn locked(&self) -> bool {
		false
	}
	fn set_lock(&self, _locked: bool) -> Option<patch::PatchPair> {
		None
	}
}
pub trait Folded {
	fn folded(&self) -> bool {
		false
	}
	fn set_fold(&self, _folded: bool) -> Option<patch::PatchPair> {
		None
	}
}
pub trait Cropable {
	fn crop(&self, _offset: Vec2<u32>, _size: Extent2<u32>) -> Option<patch::PatchPair> {
		None
	}
}
pub trait HasColorMode {
	fn color_mode(&self) -> ColorMode;
}

pub trait DocumentNode: Node + Named + Positioned + Sized + Displayed + Locked + Folded {}
impl Downcast for dyn DocumentNode {}

pub trait SpriteNode:
	Node + Named + Sized + Cropable + Displayed + Locked + Folded + HasColorMode
{
}
impl Downcast for dyn SpriteNode {}
