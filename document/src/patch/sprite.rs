use crate as document;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct SetPalette {
	pub target: Uuid,
	pub palette: NodeRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct UnsetPalette {
	pub target: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct SetColorMode {
	pub target: Uuid,
	pub color_mode: ColorMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct Crop {
	pub target: Uuid,
	pub offset: Vec2<u32>,
	pub size: Extent2<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct RestoreSprite {
	pub target: Uuid,
	pub children: Vec<patch::PatchType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct RestoreCanvas {
	pub target: Uuid,
	pub color: Vec<u8>,
	pub normal: Vec<XYZ>,
}
