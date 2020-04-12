use uuid::Uuid;

use crate::patch::Patch;

pub struct RemoveChildPatch {
	pub target: Uuid,
	pub child_id: Uuid,
}

impl Patch for RemoveChildPatch {
	fn target(&self) -> Uuid {
		self.target
	}
}
