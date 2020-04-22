use math::{Extent2, Vec2};
use uuid::Uuid;

pub struct CropLayerPatch {
	pub target: Uuid,
	pub offset: Vec2<u32>,
	pub size: Extent2<u32>,
}

#[derive(Debug)]
pub enum CropLayerError {
	InvalidSize,
	OutsideRegion,
}

impl std::fmt::Display for CropLayerError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			CropLayerError::InvalidSize => {
				write!(f, "Could not crop layer, invalid size provided.")
			}
			CropLayerError::OutsideRegion => write!(
				f,
				"Could not crop layer, final region is outside the canvas."
			),
		}
	}
}

impl std::error::Error for CropLayerError {
	fn cause(&self) -> Option<&dyn std::error::Error> {
		None
	}
}
