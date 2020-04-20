use uuid::Uuid;

use crate::patch::*;

pub enum Patch
{
	Noop,
	AddChild(AddChildPatch),
	AddLayer(AddLayerPatch),
	ApplyStencil(ApplyStencilPatch),
	CropLayer(CropLayerPatch),
	MoveChild(MoveChildPatch),
	MoveLayer(MoveLayerPatch),
	RemoveChild(RemoveChildPatch),
	RemoveLayer(RemoveLayerPatch),
	ResizeLayer(ResizeLayerPatch),
	RestoreLayerCanvas(RestoreLayerCanvasPatch),
	RestoreLayerGroup(RestoreLayerGroupPatch),
	Rename(RenamePatch),
}

impl Patch
{
	pub fn target(&self) -> Uuid {
		match self {
			Patch::Noop => Uuid::nil(),
			Patch::AddChild(patch) => patch.target,
			Patch::AddLayer(patch) => patch.target,
			Patch::ApplyStencil(patch) => patch.target,
			Patch::CropLayer(patch) => patch.target,
			Patch::MoveChild(patch) => patch.target,
			Patch::MoveLayer(patch) => patch.target,
			Patch::RemoveChild(patch) => patch.target,
			Patch::RemoveLayer(patch) => patch.target,
			Patch::ResizeLayer(patch) => patch.target,
			Patch::RestoreLayerCanvas(patch) => patch.target,
			Patch::RestoreLayerGroup(patch) => patch.target,
			Patch::Rename(patch) => patch.target,
		}
	}
}

pub trait Patchable {
	fn patch(&self, patch: &Patch) -> Option<Box<Self>>;
}
