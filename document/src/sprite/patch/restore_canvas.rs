use uuid::Uuid;
use math::Extent2;

use crate::patch::Patch;

pub struct RestoreCanvasPatch<T>
where
	T: Clone,
{
	pub target: Uuid,
	pub name: String,
	pub size: Extent2<u32>,
	pub data: Vec<T>,
}

impl<T> Patch for RestoreCanvasPatch<T>
where
	T: Clone,
{
	fn target(&self) -> Uuid {
		self.target
	}
}