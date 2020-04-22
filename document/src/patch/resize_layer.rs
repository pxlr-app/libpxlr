use math::interpolation::Interpolation;
use math::Extent2;
use uuid::Uuid;

pub struct ResizeLayerPatch {
	pub target: Uuid,
	pub size: Extent2<u32>,
	pub interpolation: Interpolation,
}
