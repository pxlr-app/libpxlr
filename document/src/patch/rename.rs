use crate::patch::Patch;
use uuid::Uuid;

pub struct RenamePatch {
	pub target: Uuid,
	pub name: String,
}

pub trait Renamable<'a> {
	fn rename(&self, new_name: &'a str) -> (Patch, Patch);
}
