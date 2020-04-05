use std::any::Any;

use uuid::Uuid;

pub trait Patch {
	fn target(&self) -> Uuid;
}

pub trait PatchImpl: Patch {
	fn as_any(&self) -> &dyn Any;
}

impl<T> PatchImpl for T
where
	T: Patch + Any,
{
	fn as_any(&self) -> &dyn Any {
		self
	}
}

pub trait Patchable {
	fn patch(&self, patch: &dyn PatchImpl) -> Option<Box<Self>>;
}