use math::{Extent2, Vec2};
use uuid::Uuid;

pub struct CropLayerPatch {
	pub target: Uuid,
	pub offset: Vec2<u32>,
	pub size: Extent2<u32>,
}
