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
