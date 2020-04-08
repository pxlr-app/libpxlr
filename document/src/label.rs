use math::Vec2;
use std::rc::Rc;
use uuid::Uuid;

use crate::document::Document;
use crate::node::Node;
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

impl Node for Label {
	fn id(&self) -> Uuid {
		self.id
	}
}

impl Document for Label {
	fn position(&self) -> Vec2<f32> {
		*(self.position).clone()
	}
}

impl<'a> Renamable<'a> for Label {
	fn rename(&self, new_name: &'a str) -> (RenamePatch, RenamePatch) {
		(
			RenamePatch {
				target: self.id,
				name: new_name.to_owned(),
			},
			RenamePatch {
				target: self.id,
				name: (*self.name).to_owned(),
			},
		)
	}
}

impl Patchable for Label {
	fn patch(&self, patch: &dyn PatchImpl) -> Option<Box<Self>> {
		if patch.target() == self.id {
			if let Some(rename) = patch.as_any().downcast_ref::<RenamePatch>() {
				return Some(Box::new(Label {
					id: self.id,
					name: Rc::new(rename.name.clone()),
					position: self.position.clone(),
				}));
			}
		}
		return None;
	}
}
