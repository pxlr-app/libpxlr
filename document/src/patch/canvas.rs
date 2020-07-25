use crate as document;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct Crop {
	pub target: Uuid,
	pub offset: Vec2<u32>,
	pub size: Extent2<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct RestoreCanvas {
	pub target: Uuid,
	pub color: Vec<u8>,
	pub alpha: Vec<Grey>,
	pub normal: Vec<XYZ>,
}
