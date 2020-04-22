use crate::document::*;
use crate::node::*;
use crate::patch::*;
use math::Vec2;
use std::rc::Rc;
use uuid::Uuid;

pub struct Group {
	pub id: Uuid,
	pub name: Rc<String>,
	pub children: Rc<Vec<Rc<DocumentNode>>>,
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
		children: Vec<Rc<DocumentNode>>,
	) -> Group {
		Group {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			position: Rc::new(position),
			children: Rc::new(children),
		}
	}

	pub fn add_child(&self, add_child: Rc<DocumentNode>) -> Result<(Patch, Patch), GroupError> {
		let index = self
			.children
			.iter()
			.position(|child| Rc::ptr_eq(&child, &add_child));
		if index.is_some() {
			Err(GroupError::ChildFound)
		} else {
			Ok((
				Patch::AddChild(AddChildPatch {
					target: self.id,
					child: add_child.clone(),
					position: self.children.len(),
				}),
				Patch::RemoveChild(RemoveChildPatch {
					target: self.id,
					child_id: add_child.id(),
				}),
			))
		}
	}

	pub fn remove_child(&self, child_id: Uuid) -> Result<(Patch, Patch), GroupError> {
		let index = self
			.children
			.iter()
			.position(|child| child.id() == child_id);
		if index.is_none() {
			Err(GroupError::ChildNotFound)
		} else {
			let index = index.unwrap();
			Ok((
				Patch::RemoveChild(RemoveChildPatch {
					target: self.id,
					child_id: child_id,
				}),
				Patch::AddChild(AddChildPatch {
					target: self.id,
					child: self.children.get(index).unwrap().clone(),
					position: index,
				}),
			))
		}
	}

	pub fn move_child(
		&self,
		child_id: Uuid,
		position: usize,
	) -> Result<(Patch, Patch), GroupError> {
		let index = self
			.children
			.iter()
			.position(|child| child.id() == child_id);
		if index.is_none() {
			Err(GroupError::ChildNotFound)
		} else {
			let index = index.unwrap();
			Ok((
				Patch::MoveChild(MoveChildPatch {
					target: self.id,
					child_id: child_id,
					position: position,
				}),
				Patch::MoveChild(MoveChildPatch {
					target: self.id,
					child_id: child_id,
					position: index,
				}),
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
	fn rename(&self, new_name: &'a str) -> (Patch, Patch) {
		(
			Patch::Rename(RenamePatch {
				target: self.id,
				name: new_name.to_owned(),
			}),
			Patch::Rename(RenamePatch {
				target: self.id,
				name: (*self.name).to_owned(),
			}),
		)
	}
}

impl Patchable for Group {
	fn patch(&self, patch: &Patch) -> Option<Box<Self>> {
		if patch.target() == self.id {
			return match patch {
				Patch::Rename(patch) => Some(Box::new(Group {
					id: self.id,
					name: Rc::new(patch.name.clone()),
					position: self.position.clone(),
					children: self.children.clone(),
				})),
				Patch::AddChild(patch) => {
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
					Some(Box::new(Group {
						id: self.id,
						name: self.name.clone(),
						position: self.position.clone(),
						children: Rc::new(children),
					}))
				}
				Patch::RemoveChild(patch) => {
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
					Some(Box::new(Group {
						id: self.id,
						name: self.name.clone(),
						position: self.position.clone(),
						children: Rc::new(children),
					}))
				}
				Patch::MoveChild(patch) => {
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
					Some(Box::new(Group {
						id: self.id,
						name: self.name.clone(),
						position: self.position.clone(),
						children: Rc::new(children),
					}))
				}
				_ => None,
			};
		} else {
			let mut mutated = false;
			let children = self
				.children
				.iter()
				.map(|child| match child.patch(patch) {
					Some(new_child) => {
						mutated = true;
						Rc::new(new_child)
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
