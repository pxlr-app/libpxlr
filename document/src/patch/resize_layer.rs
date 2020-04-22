use math::interpolation::Interpolation;
use math::Extent2;
use uuid::Uuid;

pub struct ResizeLayerPatch {
	pub target: Uuid,
	pub size: Extent2<u32>,
	pub interpolation: Interpolation,
}

#[derive(Debug)]
pub enum ResizeLayerError {
	InvalidSize,
}

impl std::fmt::Display for ResizeLayerError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			ResizeLayerError::InvalidSize => {
				write!(f, "Could not resize layer, invalid size provided.")
			}
		}
	}
}

impl std::error::Error for ResizeLayerError {
	fn cause(&self) -> Option<&dyn std::error::Error> {
		None
	}
}
