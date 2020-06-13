use crate::color::*;
use crate::patch::*;
use crate::sprite::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(non_upper_case_globals)]

bitflags! {
	pub struct PatchMode: u32 {
		const META = 0b00000001;
		const DATA = 0b00000010;
	}
}

pub trait IPatch {
	fn mode(&self) -> PatchMode {
		PatchMode::META | PatchMode::DATA
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Patch {
	Noop,
	AddChild(AddChildPatch),
	AddLayer(AddLayerPatch),
	ApplyStencilGrey(ApplyStencilPatch<StencilGrey>),
	ApplyStencilRGBA(ApplyStencilPatch<StencilRGBA>),
	ApplyStencilUV(ApplyStencilPatch<StencilUV>),
	ApplyStencilNormal(ApplyStencilPatch<StencilNormal>),
	CropLayer(CropLayerPatch),
	MoveChild(MoveChildPatch),
	MoveLayer(MoveLayerPatch),
	RemoveChild(RemoveChildPatch),
	RemoveLayer(RemoveLayerPatch),
	ResizeLayer(ResizeLayerPatch),
	RestoreLayerCanvasGrey(RestoreLayerCanvasPatch<Grey>),
	RestoreLayerCanvasRGBA(RestoreLayerCanvasPatch<RGBA>),
	RestoreLayerCanvasUV(RestoreLayerCanvasPatch<UV>),
	RestoreLayerGroup(RestoreLayerGroupPatch),
	Rename(RenamePatch),
	SetVisibility(SetVisibilityPatch),
	SetLock(SetLockPatch),
	SetFold(SetFoldPatch),
}

impl Patch {
	pub fn target(&self) -> Uuid {
		match self {
			Patch::Noop => Uuid::nil(),
			Patch::AddChild(patch) => patch.target,
			Patch::AddLayer(patch) => patch.target,
			Patch::ApplyStencilGrey(patch) => patch.target,
			Patch::ApplyStencilRGBA(patch) => patch.target,
			Patch::ApplyStencilUV(patch) => patch.target,
			Patch::ApplyStencilNormal(patch) => patch.target,
			Patch::CropLayer(patch) => patch.target,
			Patch::MoveChild(patch) => patch.target,
			Patch::MoveLayer(patch) => patch.target,
			Patch::RemoveChild(patch) => patch.target,
			Patch::RemoveLayer(patch) => patch.target,
			Patch::ResizeLayer(patch) => patch.target,
			Patch::RestoreLayerCanvasGrey(patch) => patch.target,
			Patch::RestoreLayerCanvasRGBA(patch) => patch.target,
			Patch::RestoreLayerCanvasUV(patch) => patch.target,
			Patch::RestoreLayerGroup(patch) => patch.target,
			Patch::Rename(patch) => patch.target,
			Patch::SetVisibility(patch) => patch.target,
			Patch::SetLock(patch) => patch.target,
			Patch::SetFold(patch) => patch.target,
		}
	}
}

impl IPatch for Patch {
	fn mode(&self) -> PatchMode {
		match self {
			Patch::Noop => PatchMode::empty(),
			Patch::AddChild(patch) => patch.mode(),
			Patch::AddLayer(patch) => patch.mode(),
			Patch::ApplyStencilGrey(patch) => patch.mode(),
			Patch::ApplyStencilRGBA(patch) => patch.mode(),
			Patch::ApplyStencilUV(patch) => patch.mode(),
			Patch::ApplyStencilNormal(patch) => patch.mode(),
			Patch::CropLayer(patch) => patch.mode(),
			Patch::MoveChild(patch) => patch.mode(),
			Patch::MoveLayer(patch) => patch.mode(),
			Patch::RemoveChild(patch) => patch.mode(),
			Patch::RemoveLayer(patch) => patch.mode(),
			Patch::ResizeLayer(patch) => patch.mode(),
			Patch::RestoreLayerCanvasGrey(patch) => patch.mode(),
			Patch::RestoreLayerCanvasRGBA(patch) => patch.mode(),
			Patch::RestoreLayerCanvasUV(patch) => patch.mode(),
			Patch::RestoreLayerGroup(patch) => patch.mode(),
			Patch::Rename(patch) => patch.mode(),
			Patch::SetVisibility(patch) => patch.mode(),
			Patch::SetLock(patch) => patch.mode(),
			Patch::SetFold(patch) => patch.mode(),
		}
	}
}

pub trait IPatchable {
	fn patch(&self, patch: &Patch) -> Option<Box<Self>>;
}
