use std::rc::Rc;

use uuid::Uuid;

use crate::patch::Patch;
use crate::sprite::GroupLayer;

pub struct AddLayerPatch {
	pub target: Uuid,
	pub child: Rc<dyn GroupLayer>,
	pub position: usize,
}

impl Patch for AddLayerPatch {
	fn target(&self) -> Uuid {
		self.target
	}
}
