use math::{Extent2, Vec2};
use uuid::Uuid;

use crate::patch::{Patch, PatchImpl};

pub struct RestoreGroupPatch {
	pub target: Uuid,
	pub name: String,
	pub position: Vec2<f32>,
	pub size: Extent2<u32>,
	pub children: Vec<Box<dyn PatchImpl>>,
}

impl Patch for RestoreGroupPatch {
	fn target(&self) -> Uuid {
		self.target
	}
}
