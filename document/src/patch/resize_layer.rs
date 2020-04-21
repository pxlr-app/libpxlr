use math::Extent2;
use uuid::Uuid;

use crate::sprite::Interpolation;

pub struct ResizeLayerPatch {
	pub target: Uuid,
	pub size: Extent2<u32>,
	pub interpolation: Interpolation,
}
