use std::any::Any;
use std::rc::Rc;

use uuid::Uuid;
use math::{Vec2, Extent2};

use crate::document::Document;
use crate::node::*;
use crate::patch::*;
use crate::sprite::Layer;

pub trait GroupLayer: Node {
	fn patch_rc(&self, patch: &dyn PatchImpl) -> Option<Rc<dyn GroupLayer + 'static>>;
	fn as_any(&self) -> &dyn Any;
}
impl<T> GroupLayer for T
where
	T: Patchable + Layer + Any,
{
	fn patch_rc(&self, patch: &dyn PatchImpl) -> Option<Rc<dyn GroupLayer + 'static>> {
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
	pub children: Rc<Vec<Rc<dyn GroupLayer>>>,
	pub position: Rc<Vec2<f32>>,
	pub size: Rc<Extent2<u16>>,
}

impl Group {
	pub fn new(id: Option<Uuid>, name: &str, children: Vec<Rc<dyn GroupLayer>>, position: Vec2<f32>, size: Extent2<u16>) -> Group {
		Group {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			children: Rc::new(children),
			position: Rc::new(position),
			size: Rc::new(size),
		}
	}
}

impl Node for Group {
	fn id(&self) -> Uuid {
		self.id
	}
}

impl Layer for Group {}

impl Document for Group {
	fn position(&self) -> Vec2<f32> {
		*(self.position).clone()
	}
}

impl<'a> Renamable<'a> for Group {
	fn rename(&self, new_name: &'a str) -> RenamePatch {
		RenamePatch { target: self.id, new_name: new_name.to_owned() }
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
					size: self.size.clone(),
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
					position: Rc::clone(&self.position),
					size: Rc::clone(&self.size),
					children: Rc::new(children),
				}))
			} else {
				None
			}
		}
	}
}