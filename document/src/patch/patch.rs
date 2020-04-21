use uuid::Uuid;

use crate::patch::*;

pub enum Patch {
	Noop,
	AddChild(AddChildPatch),
	AddLayer(AddLayerPatch),
	ApplyStencilI(ApplyStencilIPatch),
	ApplyStencilUV(ApplyStencilUVPatch),
	ApplyStencilRGB(ApplyStencilRGBPatch),
	ApplyStencilRGBA(ApplyStencilRGBAPatch),
	ApplyStencilRGBAXYZ(ApplyStencilRGBAXYZPatch),
	CropLayer(CropLayerPatch),
	MoveChild(MoveChildPatch),
	MoveLayer(MoveLayerPatch),
	RemoveChild(RemoveChildPatch),
	RemoveLayer(RemoveLayerPatch),
	ResizeLayer(ResizeLayerPatch),
	RestoreLayerCanvasI(RestoreLayerCanvasIPatch),
	RestoreLayerCanvasUV(RestoreLayerCanvasUVPatch),
	RestoreLayerCanvasRGB(RestoreLayerCanvasRGBPatch),
	RestoreLayerCanvasRGBA(RestoreLayerCanvasRGBAPatch),
	RestoreLayerCanvasRGBAXYZ(RestoreLayerCanvasRGBAXYZPatch),
	RestoreLayerGroup(RestoreLayerGroupPatch),
	Rename(RenamePatch),
}

impl Patch {
	pub fn target(&self) -> Uuid {
		match self {
			Patch::Noop => Uuid::nil(),
			Patch::AddChild(patch) => patch.target,
			Patch::AddLayer(patch) => patch.target,
			Patch::ApplyStencilUV(patch) => patch.target,
			Patch::ApplyStencilI(patch) => patch.target,
			Patch::ApplyStencilRGB(patch) => patch.target,
			Patch::ApplyStencilRGBA(patch) => patch.target,
			Patch::ApplyStencilRGBAXYZ(patch) => patch.target,
			Patch::CropLayer(patch) => patch.target,
			Patch::MoveChild(patch) => patch.target,
			Patch::MoveLayer(patch) => patch.target,
			Patch::RemoveChild(patch) => patch.target,
			Patch::RemoveLayer(patch) => patch.target,
			Patch::ResizeLayer(patch) => patch.target,
			Patch::RestoreLayerCanvasI(patch) => patch.target,
			Patch::RestoreLayerCanvasUV(patch) => patch.target,
			Patch::RestoreLayerCanvasRGB(patch) => patch.target,
			Patch::RestoreLayerCanvasRGBA(patch) => patch.target,
			Patch::RestoreLayerCanvasRGBAXYZ(patch) => patch.target,
			Patch::RestoreLayerGroup(patch) => patch.target,
			Patch::Rename(patch) => patch.target,
		}
	}
}

pub trait Patchable {
	fn patch(&self, patch: &Patch) -> Option<Box<Self>>;
}
