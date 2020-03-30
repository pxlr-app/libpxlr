use std::rc::Rc;
use uuid::Uuid;
use math::{Vec2};

use crate::document::*;
use crate::node::*;
use crate::patch::*;

pub struct Label {
	pub id: Uuid,
	pub name: Rc<String>,
	pub position: Rc<Vec2<f32>>,
}

impl Label {
	pub fn new(id: Option<Uuid>, name: &str, position: Vec2<f32>) -> Label {
		Label {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			position: Rc::new(position),
		}
	}
}

impl INode for Label {
	fn id(&self) -> Uuid {
		self.id
	}
	fn display(&self) -> String {
		self.name.to_string()
	}
}

impl IDocument for Label {
	fn position(&self) -> Vec2<f32> {
		*(self.position).clone()
	}
	fn patch(&self, patch: &Patch) -> Option<Document> {
		if patch.target == self.id {
			match &patch.payload {
				PatchAction::Rename(new_name) => Some(Document::Label(Label {
					id: self.id,
					name: Rc::new(new_name.to_string()),
					position: Rc::clone(&self.position),
				})),
				_ => None,
			}
		} else {
			None
		}
	}
}