use uuid::Uuid;

pub struct Patch<'a> {
	pub target: Uuid,
	pub payload: PatchAction<'a>,
}

pub enum PatchAction<'a> {
	Rename(&'a str),
	Resize(u32, u32),
}

pub trait Patchable<T> {
	fn patch(&self, patch: &Patch) -> Option<T>;
}