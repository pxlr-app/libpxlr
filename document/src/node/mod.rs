use crate::{
	any::{Any, Downcast},
	parser, patch,
};
use math::{Extent2, Vec2};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, io, sync::Arc};
use uuid::Uuid;
mod group;
mod note;
pub use group::*;
pub use note::*;

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
}

impl NodeType {
	pub fn as_node(&self) -> &dyn Node {
		match self {
			NodeType::Note(node) => node,
			NodeType::Group(node) => node,
		}
	}
	pub fn as_documentnode(&self) -> Option<&dyn DocumentNode> {
		match self {
			NodeType::Note(node) => Some(node),
			NodeType::Group(node) => Some(node),
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
		match row.chunk_type.as_ref() {
			"Note" => <Note as parser::v0::ParseNode>::parse_node(row, items, dependencies, bytes),
			"Group" => {
				<Group as parser::v0::ParseNode>::parse_node(row, items, dependencies, bytes)
			}
			_ => Err(nom::Err::Incomplete(nom::Needed::Unknown)),
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
		}
	}
}

pub trait Name {
	fn name(&self) -> String {
		"".into()
	}
	fn rename(&self, _name: String) -> Option<(patch::PatchType, patch::PatchType)> {
		None
	}
}
pub trait Position {
	fn position(&self) -> Vec2<u32> {
		Vec2::new(0, 0)
	}
	fn translate(&self, _target: Vec2<u32>) -> Option<(patch::PatchType, patch::PatchType)> {
		None
	}
}
pub trait Size {
	fn size(&self) -> Extent2<u32> {
		Extent2::new(0, 0)
	}
	fn resize(&self, _target: Extent2<u32>) -> Option<(patch::PatchType, patch::PatchType)> {
		None
	}
}
pub trait Visible {
	fn visible(&self) -> bool {
		true
	}
	fn set_visibility(&self, _visible: bool) -> Option<(patch::PatchType, patch::PatchType)> {
		None
	}
}
pub trait Locked {
	fn locked(&self) -> bool {
		false
	}
	fn set_lock(&self, _locked: bool) -> Option<(patch::PatchType, patch::PatchType)> {
		None
	}
}
pub trait Folded {
	fn folded(&self) -> bool {
		false
	}
	fn set_fold(&self, _folded: bool) -> Option<(patch::PatchType, patch::PatchType)> {
		None
	}
}

pub trait DocumentNode: Node + Name + Position + Size + Visible + Locked + Folded {}
impl Downcast for dyn DocumentNode {}
