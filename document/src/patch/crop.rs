use uuid::Uuid;
use math::{Extent2, Vec2};

use crate::patch::Patch;

pub struct CropPatch {
	pub target: Uuid,
	pub offset: Vec2<u32>,
	pub size: Extent2<u32>,
}

impl Patch for CropPatch {
	fn target(&self) -> Uuid { self.target }
}