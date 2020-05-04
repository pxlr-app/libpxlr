use crate::patch::IPatch;
use crate::patch::Patch;
use math::{Extent2, Vec2};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreLayerGroupPatch {
	pub target: Uuid,
	pub name: String,
	pub position: Vec2<f32>,
	pub size: Extent2<u32>,
	pub children: Vec<Patch>,
}

impl IPatch for RestoreLayerGroupPatch {}
