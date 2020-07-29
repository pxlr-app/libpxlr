use crate as document;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct CropCommand {
	pub target: Uuid,
	pub offset: Vec2<u32>,
	pub size: Extent2<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct RestoreCanvasCommand {
	pub target: Uuid,
	pub channels: Channel,
	pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct ApplyStencilCommand {
	pub target: Uuid,
	pub offset: Vec2<u32>,
	pub stencil: Stencil,
}
