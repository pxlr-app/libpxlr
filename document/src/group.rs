use std::any::Any;
use std::rc::Rc;

use uuid::Uuid;
use math::{Vec2};

use crate::document::*;
use crate::node::*;
use crate::patch::*;

pub trait GroupChild: Node {
	fn patch_rc(&self, patch: &dyn PatchImpl) -> Option<Rc<dyn GroupChild + 'static>>;
	fn as_any(&self) -> &dyn Any;
}
impl<T> GroupChild for T
where
	T: Patchable + Document + Any,
{
	fn patch_rc(&self, patch: &dyn PatchImpl) -> Option<Rc<dyn GroupChild + 'static>> {
		match self.patch(patch) {
			Some(new_self) => Some(Rc::new(*new_self)),
			None => None
		}
	}
	fn as_any(&self) -> &dyn Any {
		self
	}
}

pub struct Group {
	pub id: Uuid,
	pub name: Rc<String>,
	pub children: Rc<Vec<Rc<dyn GroupChild>>>,
	pub position: Rc<Vec2<f32>>,
}

impl Group {
	pub fn new(id: Option<Uuid>, name: &str, position: Vec2<f32>, children: Vec<Rc<dyn GroupChild>>) -> Group {
		Group {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			position: Rc::new(position),
			children: Rc::new(children)
		}
	}
}

impl Node for Group {
	fn id(&self) -> Uuid {
		self.id
	}
}

impl Document for Group {
	fn position(&self) -> Vec2<f32> {
		*(self.position).clone()
	}
}

impl Patchable for Group {
	fn patch(&self, patch: &dyn PatchImpl) -> Option<Box<Self>> {
		if patch.target() == self.id {
			if let Some(rename) = patch.as_any().downcast_ref::<RenamePatch>() {
				Some(Box::new(Group {
					id: self.id,
					name: Rc::new(rename.new_name.clone()),
					position: self.position.clone(),
					children: self.children.clone()
				}))
			} else {
				None
			}
		} else {
			let mut mutated = false;
			let children = self.children.iter().map(|child| {
				match child.patch_rc(patch) {
					Some(new_child) => {
						mutated = true;
						new_child
					},
					None => child.clone()
				}
			}).collect::<Vec<_>>();
			
			if mutated {
				Some(Box::new(Group {
					id: self.id,
					name: Rc::clone(&self.name),
					children: Rc::new(children),
					position: Rc::clone(&self.position),
				}))
			} else {
				None
			}
		}
	}
}