use uuid::Uuid;

use crate::patch::Patch;

pub struct RenamePatch {
	pub target: Uuid,
	pub name: String,
}

impl Patch for RenamePatch {
	fn target(&self) -> Uuid { self.target }
}

pub trait Renamable<'a> {
	fn rename(&self, new_name: &'a str) -> RenamePatch;
}