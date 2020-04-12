use uuid::Uuid;

use crate::patch::Patch;

pub struct RemoveLayerPatch {
	pub target: Uuid,
	pub child_id: Uuid,
}

impl Patch for RemoveLayerPatch {
	fn target(&self) -> Uuid {
		self.target
	}
}
