use crate::patch::IPatch;
use crate::sprite::LayerNode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddLayerPatch {
	pub target: Uuid,
	pub child: Arc<LayerNode>,
	pub position: usize,
}

impl IPatch for AddLayerPatch {}
