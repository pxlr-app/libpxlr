use crate::{
	any::{Any, Downcast},
	node::NodeType,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

mod node;
pub use node::*;

pub trait Patch: Any + Debug {
	fn target(&self) -> Uuid;
}
impl Downcast for dyn Patch {}

pub trait Patchable {
	fn patch(&self, patch: &PatchType) -> Option<NodeType>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatchType {
	Translate(Translate),
	Resize(Resize),
	SetVisible(SetVisible),
	SetLock(SetLock),
	SetFold(SetFold),
	Rename(Rename),
	AddChild(AddChild),
	RemoveChild(RemoveChild),
	MoveChild(MoveChild),
}

impl PatchType {
	pub fn as_patch(&self) -> &dyn Patch {
		match self {
			PatchType::Translate(patch) => patch,
			PatchType::Resize(patch) => patch,
			PatchType::SetVisible(patch) => patch,
			PatchType::SetLock(patch) => patch,
			PatchType::SetFold(patch) => patch,
			PatchType::Rename(patch) => patch,
			PatchType::AddChild(patch) => patch,
			PatchType::RemoveChild(patch) => patch,
			PatchType::MoveChild(patch) => patch,
		}
	}
}

pub type PatchPair = (PatchType, PatchType);
