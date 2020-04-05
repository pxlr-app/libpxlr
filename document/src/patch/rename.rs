use uuid::Uuid;

use crate::patch::Patch;

pub struct RenamePatch {
	pub target: Uuid,
	pub new_name: String,
}

impl Patch for RenamePatch {
	fn target(&self) -> Uuid { self.target }
}