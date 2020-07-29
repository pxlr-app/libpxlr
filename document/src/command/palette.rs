use crate as document;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct AddColorCommand {
	pub target: Uuid,
	pub color: RGB,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct RemoveColorCommand {
	pub target: Uuid,
	pub color: RGB,
}

#[derive(Debug, Clone, Serialize, Deserialize, Command)]
pub struct MoveColorCommand {
	pub target: Uuid,
	pub color: RGB,
	pub position: usize,
}
