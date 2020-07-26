use crate as document;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct Crop {
	pub target: Uuid,
	pub offset: Vec2<u32>,
	pub size: Extent2<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct RestoreCanvas {
	pub target: Uuid,
	pub channels: ColorMode,
	pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct ApplyStencil2 {
	pub target: Uuid,
	pub offset: Vec2<u32>,
	pub channels: ColorMode,
	pub stencil: Stencil2,
}
