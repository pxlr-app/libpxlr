use math::Vec2;
use std::rc::Rc;
use uuid::Uuid;

use crate::document::Document;
use crate::node::Node;
use crate::patch::*;

pub struct Note {
	pub id: Uuid,
	pub name: Rc<String>,
	pub position: Rc<Vec2<f32>>,
}

impl Note {
	pub fn new(id: Option<Uuid>, name: &str, position: Vec2<f32>) -> Note {
		Note {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			position: Rc::new(position),
		}
	}
}

impl Node for Note {
	fn id(&self) -> Uuid {
		self.id
	}
}

impl Document for Note {
	fn position(&self) -> Vec2<f32> {
		*(self.position).clone()
	}
}

impl<'a> Renamable<'a> for Note {
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

impl Patchable for Note {
	fn patch(&self, patch: &dyn PatchImpl) -> Option<Box<Self>> {
		if patch.target() == self.id {
			if let Some(rename) = patch.as_any().downcast_ref::<RenamePatch>() {
				return Some(Box::new(Note {
					id: self.id,
					name: Rc::new(rename.name.clone()),
					position: self.position.clone(),
				}));
			}
		}
		return None;
	}
}
