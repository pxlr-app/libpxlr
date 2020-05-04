use crate::patch::{IPatch, PatchMode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveLayerPatch {
	pub target: Uuid,
	pub child_id: Uuid,
}

impl IPatch for RemoveLayerPatch {
	fn mode(&self) -> PatchMode {
		PatchMode::META
	}
}
