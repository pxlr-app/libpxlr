use crate as document;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Patch)]
pub struct RestoreCanvas {
	pub target: Uuid,
	pub color: Vec<u8>,
	pub normal: Vec<XYZ>,
}
