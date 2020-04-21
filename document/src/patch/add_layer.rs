use std::rc::Rc;
use uuid::Uuid;

use crate::sprite::LayerNode;

pub struct AddLayerPatch {
	pub target: Uuid,
	pub child: Rc<LayerNode>,
	pub position: usize,
}
