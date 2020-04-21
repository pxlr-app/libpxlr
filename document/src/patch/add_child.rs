use std::rc::Rc;
use uuid::Uuid;

use crate::document::DocumentNode;

pub struct AddChildPatch {
	pub target: Uuid,
	pub child: Rc<DocumentNode>,
	pub position: usize,
}
