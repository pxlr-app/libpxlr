use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveLayerPatch {
	pub target: Uuid,
	pub child_id: Uuid,
	pub position: usize,
}
