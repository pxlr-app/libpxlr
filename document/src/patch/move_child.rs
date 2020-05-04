use crate::patch::{IPatch, PatchMode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveChildPatch {
	pub target: Uuid,
	pub child_id: Uuid,
	pub position: usize,
}

impl IPatch for MoveChildPatch {
	fn mode(&self) -> PatchMode {
		PatchMode::META
	}
}
