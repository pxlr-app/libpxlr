use math::{Extent2, Vec2};
use uuid::Uuid;

use crate::patch::Patch;

pub struct RestoreLayerGroupPatch {
	pub target: Uuid,
	pub name: String,
	pub position: Vec2<f32>,
	pub size: Extent2<u32>,
	pub children: Vec<Patch>,
}