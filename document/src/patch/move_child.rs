use uuid::Uuid;

use crate::patch::Patch;

pub struct MoveChildPatch {
	pub target: Uuid,
	pub child_id: Uuid,
	pub position: usize,
}

impl Patch for MoveChildPatch {
	fn target(&self) -> Uuid {
		self.target
	}
}