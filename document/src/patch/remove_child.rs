use crate::patch::{IPatch, PatchMode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveChildPatch {
	pub target: Uuid,
	pub child_id: Uuid,
}

impl IPatch for RemoveChildPatch {
	fn mode(&self) -> PatchMode {
		PatchMode::META
	}
}
