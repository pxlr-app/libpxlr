use crate as document;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct Translate {
	pub target: Uuid,
	pub position: Vec2<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct Resize {
	pub target: Uuid,
	pub size: Extent2<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct SetVisible {
	pub target: Uuid,
	pub visibility: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct SetLock {
	pub target: Uuid,
	pub locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct SetFold {
	pub target: Uuid,
	pub folded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct Rename {
	pub target: Uuid,
	pub name: String,
}
