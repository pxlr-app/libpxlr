use crate::sprite::LayerNode;
use std::rc::Rc;
use uuid::Uuid;

pub struct AddLayerPatch {
	pub target: Uuid,
	pub child: Rc<LayerNode>,
	pub position: usize,
}
