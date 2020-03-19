#[macro_use]
extern crate downcast_rs;

use std::rc::Rc;
use uuid::Uuid;
// use im::Vector as List;
use math::{Rect, Vec2};
use downcast_rs::Downcast;

pub struct Patch {
	target: Uuid,
	payload: PatchPayload,
}

pub enum PatchPayload {
	Rename(String),
	Resize(u32, u32),
}

pub trait Patchable: Downcast {
	fn patch(&self, patch: &Patch) -> Option<Box<dyn Patchable>>;
}
impl_downcast!(Patchable);

pub trait Document: Patchable {
	fn get_bounds(&self) -> Rect;
}

pub struct Group {
	pub id: Uuid,
	pub name: Rc<String>,
	pub children: Rc<Vec<Rc<Box<dyn Patchable>>>>,
	pub position: Rc<Vec2>,
}

impl Group {
	fn new(id: Option<Uuid>, name: &str, position: Vec2, children: Vec<Rc<Box<dyn Patchable>>>) -> Group {
		Group {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			position: Rc::new(position),
			children: Rc::new(children)
		}
	}
}

impl Patchable for Group {
	fn patch(&self, patch: &Patch) -> Option<Box<dyn Patchable>> {
		if patch.target == self.id {
			match &patch.payload {
				PatchPayload::Rename(new_name) => Some(Box::new(Group {
					id: self.id,
					name: Rc::new(new_name.clone()),
					children: Rc::clone(&self.children),
					position: Rc::clone(&self.position),
				})),
				_ => None,
			}
		} else {
			let mut mutated = false;
			let children = self.children.iter().map(|child| {
				if let Some(new_child) = child.patch(patch) {
					mutated = true;
					Rc::new(new_child)
				} else {
					Rc::clone(child)
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

impl Document for Group {
	fn get_bounds(&self) -> Rect {
		Rect::new(*self.position, *self.position)
	}
}

pub struct Label {
	pub id: Uuid,
	pub name: Rc<String>,
	pub position: Rc<Vec2>,
}

impl Patchable for Label {
	fn patch(&self, patch: &Patch) -> Option<Box<dyn Patchable>> {
		if patch.target == self.id {
			match &patch.payload {
				PatchPayload::Rename(new_name) => Some(Box::new(Label {
					id: self.id,
					name: Rc::new(new_name.clone()),
					position: Rc::clone(&self.position),
				})),
				_ => None,
			}
		} else {
			None
		}
	}
}

impl Document for Label {
	fn get_bounds(&self) -> Rect {
		Rect::new(*self.position, *self.position)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use uuid::Uuid;
	use std::any::Any;

	#[test]
	fn it_adds() {
		let root = Group::new(None, "Root", Vec2::new(0., 0.), vec![] );
		let fork = root.patch(&Patch { target: root.id, payload: PatchPayload::Rename("Ruut".to_owned()) }).unwrap();
		let blp = fork.downcast_ref::<Group>().unwrap();
		assert_eq!(root.name, blp.name);
		// assert_eq!(Vector2::new(1.0, 1.0) + Vector2::new(1.0, 1.0), Vector2::new(2.0, 2.0));
		// assert_eq!(Vector2::new(0.0, 0.0) + 2.0, Vector2::new(2.0, 2.0));
		// let mut v1 = Vector2::new(0.0, 0.0);
		// v1 += 2.0;
		// assert_eq!(v1, Vector2::new(2.0, 2.0));
	}
}
