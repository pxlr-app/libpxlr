use crate::patch::IPatch;
use math::interpolation::Interpolation;
use math::Extent2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResizeLayerPatch {
	pub target: Uuid,
	pub size: Extent2<u32>,
	pub interpolation: Interpolation,
}

impl IPatch for ResizeLayerPatch {}

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
