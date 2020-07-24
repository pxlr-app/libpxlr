use crate as document;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct AddColor {
	pub target: Uuid,
	pub color: RGB,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct RemoveColor {
	pub target: Uuid,
	pub color: RGB,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct MoveColor {
	pub target: Uuid,
	pub color: RGB,
	pub position: usize,
}
