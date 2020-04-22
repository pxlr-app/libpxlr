use crate::sprite::LayerNode;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddLayerPatch {
	pub target: Uuid,
	pub child: Rc<LayerNode>,
	pub position: usize,
}
