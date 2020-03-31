use uuid::Uuid;

pub struct Patch<'a> {
	pub target: Uuid,
	pub payload: PatchAction<'a>,
}

pub enum PatchAction<'a> {
	Rename(&'a str),
	Resize(u32, u32),
}