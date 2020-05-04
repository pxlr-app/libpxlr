use crate::document::DocumentNode;
use crate::patch::IPatch;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddChildPatch {
	pub target: Uuid,
	pub child: Rc<DocumentNode>,
	pub position: usize,
}

impl IPatch for AddChildPatch {}
