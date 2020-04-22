use crate::document::DocumentNode;
use std::rc::Rc;
use uuid::Uuid;

pub struct AddChildPatch {
	pub target: Uuid,
	pub child: Rc<DocumentNode>,
	pub position: usize,
}
