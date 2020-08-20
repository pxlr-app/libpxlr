use crate as document;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct SetPaletteNodeCommand {
	pub target: Uuid,
	pub palette: Option<NodeRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct SetOpacityCommand {
	pub target: Uuid,
	pub opacity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct SetComponentsCommand {
	pub target: Uuid,
	pub components: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct RestoreCanvasGroupCommand {
	pub target: Uuid,
	pub children: Vec<CommandType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct CropCommand {
	pub target: Uuid,
	pub offset: Vec2<u32>,
	pub size: Extent2<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct RestoreCanvasCommand {
	pub target: Uuid,
	pub components: u8,
	pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct ApplyStencilCommand {
	pub target: Uuid,
	pub offset: Vec2<u32>,
	pub stencil: Stencil,
}
