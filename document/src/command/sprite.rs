use crate as document;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct SetPaletteCommand {
	pub target: Uuid,
	pub palette: Option<NodeRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct SetChannelsCommand {
	pub target: Uuid,
	pub channels: Channel,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct RestoreSpriteCommand {
	pub target: Uuid,
	pub children: Vec<CommandType>,
}
