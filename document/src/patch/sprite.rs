use crate as document;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct SetPalette {
	pub target: Uuid,
	pub palette: Option<NodeRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct SetChannels {
	pub target: Uuid,
	pub channels: ColorMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct RestoreSprite {
	pub target: Uuid,
	pub children: Vec<patch::PatchType>,
}
