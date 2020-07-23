use crate::{
	any::{Any, Downcast},
	node::NodeType,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

mod node;
mod palette;
mod sprite;
pub use node::*;
pub use palette::*;
pub use sprite::*;

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
	AddColor(AddColor),
	RemoveColor(RemoveColor),
	MoveColor(MoveColor),
	SetPalette(SetPalette),
	UnsetPalette(UnsetPalette),
	SetColorMode(SetColorMode),
	Crop(Crop),
	RestoreSprite(RestoreSprite),
	RestoreCanvas(RestoreCanvas),
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
			PatchType::AddColor(patch) => patch,
			PatchType::RemoveColor(patch) => patch,
			PatchType::MoveColor(patch) => patch,
			PatchType::SetPalette(patch) => patch,
			PatchType::UnsetPalette(patch) => patch,
			PatchType::SetColorMode(patch) => patch,
			PatchType::Crop(patch) => patch,
			PatchType::RestoreSprite(patch) => patch,
			PatchType::RestoreCanvas(patch) => patch,
		}
	}
}

pub type PatchPair = (PatchType, PatchType);
