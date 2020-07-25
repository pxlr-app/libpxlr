use crate as document;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct SetPalette {
	pub target: Uuid,
	pub palette: Option<NodeRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct SetColorMode {
	pub target: Uuid,
	pub color_mode: ColorMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct RestoreSprite {
	pub target: Uuid,
	pub children: Vec<patch::PatchType>,
}
