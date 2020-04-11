use std::any::Any;
use std::rc::Rc;

use math::Vec2;
use uuid::Uuid;

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
			None => None,
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

#[derive(Debug)]
pub enum GroupError {
	ChildFound,
	ChildNotFound,
}

impl std::fmt::Display for GroupError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			GroupError::ChildFound => write!(f, "Child already exists in this group."),
			GroupError::ChildNotFound => write!(f, "Child not found in this group."),
		}
	}
}

impl std::error::Error for GroupError {
	fn cause(&self) -> Option<&dyn std::error::Error> {
		None
	}
}

impl Group {
	pub fn new(
		id: Option<Uuid>,
		name: &str,
		position: Vec2<f32>,
		children: Vec<Rc<dyn GroupChild>>,
	) -> Group {
		Group {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			position: Rc::new(position),
			children: Rc::new(children),
		}
	}

	pub fn add_child(
		&self,
		add_child: Rc<dyn GroupChild>,
	) -> Result<(AddChildPatch, RemoveChildPatch), GroupError> {
		let index = self
			.children
			.iter()
			.position(|child| Rc::ptr_eq(&child, &add_child));
		if index.is_some() {
			Err(GroupError::ChildFound)
		} else {
			Ok((
				AddChildPatch {
					target: self.id,
					child: add_child.clone(),
					position: self.children.len(),
				},
				RemoveChildPatch {
					target: self.id,
					child_id: add_child.id(),
				},
			))
		}
	}

	pub fn remove_child(
		&self,
		child_id: Uuid,
	) -> Result<(RemoveChildPatch, AddChildPatch), GroupError> {
		let index = self
			.children
			.iter()
			.position(|child| child.id() == child_id);
		if index.is_none() {
			Err(GroupError::ChildNotFound)
		} else {
			let index = index.unwrap();
			Ok((
				RemoveChildPatch {
					target: self.id,
					child_id: child_id,
				},
				AddChildPatch {
					target: self.id,
					child: self.children.get(index).unwrap().clone(),
					position: index,
				},
			))
		}
	}

	pub fn move_child(
		&self,
		child_id: Uuid,
		position: usize,
	) -> Result<(MoveChildPatch, MoveChildPatch), GroupError> {
		let index = self
			.children
			.iter()
			.position(|child| child.id() == child_id);
		if index.is_none() {
			Err(GroupError::ChildNotFound)
		} else {
			let index = index.unwrap();
			Ok((
				MoveChildPatch {
					target: self.id,
					child_id: child_id,
					position: position,
				},
				MoveChildPatch {
					target: self.id,
					child_id: child_id,
					position: index,
				},
			))
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

impl<'a> Renamable<'a> for Group {
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

impl Patchable for Group {
	fn patch(&self, patch: &dyn PatchImpl) -> Option<Box<Self>> {
		if patch.target() == self.id {
			if let Some(patch) = patch.as_any().downcast_ref::<RenamePatch>() {
				return Some(Box::new(Group {
					id: self.id,
					name: Rc::new(patch.name.clone()),
					position: self.position.clone(),
					children: self.children.clone(),
				}));
			} else if let Some(patch) = patch.as_any().downcast_ref::<AddChildPatch>() {
				let mut children = self
					.children
					.iter()
					.map(|child| child.clone())
					.collect::<Vec<_>>();
				if patch.position > children.len() {
					children.push(patch.child.clone());
				} else {
					children.insert(patch.position, patch.child.clone());
				}
				return Some(Box::new(Group {
					id: self.id,
					name: self.name.clone(),
					position: self.position.clone(),
					children: Rc::new(children),
				}));
			} else if let Some(patch) = patch.as_any().downcast_ref::<RemoveChildPatch>() {
				let children = self
					.children
					.iter()
					.filter_map(|child| {
						if child.id() == patch.child_id {
							None
						} else {
							Some(child.clone())
						}
					})
					.collect::<Vec<_>>();
				return Some(Box::new(Group {
					id: self.id,
					name: self.name.clone(),
					position: self.position.clone(),
					children: Rc::new(children),
				}));
			} else if let Some(patch) = patch.as_any().downcast_ref::<MoveChildPatch>() {
				let mut children = self
					.children
					.iter()
					.map(|child| child.clone())
					.collect::<Vec<_>>();
				let index = children
					.iter()
					.position(|child| child.id() == patch.child_id)
					.unwrap();
				let child = children.remove(index);
				if patch.position > children.len() {
					children.push(child);
				} else {
					children.insert(patch.position, child);
				}
				return Some(Box::new(Group {
					id: self.id,
					name: self.name.clone(),
					position: self.position.clone(),
					children: Rc::new(children),
				}));
			}
		} else {
			let mut mutated = false;
			let children = self
				.children
				.iter()
				.map(|child| match child.patch_rc(patch) {
					Some(new_child) => {
						mutated = true;
						new_child
					}
					None => child.clone(),
				})
				.collect::<Vec<_>>();
			if mutated {
				return Some(Box::new(Group {
					id: self.id,
					name: Rc::clone(&self.name),
					children: Rc::new(children),
					position: Rc::clone(&self.position),
				}));
			}
		}
		return None;
	}
}
