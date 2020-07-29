use crate as document;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct TranslateCommand {
	pub target: Uuid,
	pub position: Vec2<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct ResizeCommand {
	pub target: Uuid,
	pub size: Extent2<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct SetVisibleCommand {
	pub target: Uuid,
	pub visibility: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct SetLockCommand {
	pub target: Uuid,
	pub locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct SetFoldCommand {
	pub target: Uuid,
	pub folded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct RenameCommand {
	pub target: Uuid,
	pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct AddChildCommand {
	pub target: Uuid,
	pub child: NodeRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct RemoveChildCommand {
	pub target: Uuid,
	pub child_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct MoveChildCommand {
	pub target: Uuid,
	pub child_id: Uuid,
	pub position: usize,
}
