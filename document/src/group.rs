use crate::patch::*;
use crate::{DocumentNode, IDocument, INode};
use math::Vec2;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
	pub id: Uuid,
	pub is_visible: bool,
	pub is_locked: bool,
	pub is_folded: bool,
	pub name: Arc<String>,
	pub children: Arc<Vec<Arc<DocumentNode>>>,
	pub position: Arc<Vec2<f32>>,
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
		children: Vec<Arc<DocumentNode>>,
	) -> Group {
		Group {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			is_visible: true,
			is_locked: false,
			is_folded: false,
			name: Arc::new(name.to_owned()),
			position: Arc::new(position),
			children: Arc::new(children),
		}
	}

	pub fn add_child(&self, add_child: Arc<DocumentNode>) -> Result<(Patch, Patch), GroupError> {
		let index = self
			.children
			.iter()
			.position(|child| Arc::ptr_eq(&child, &add_child));
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

impl IDocument for Group {
	fn position(&self) -> Vec2<f32> {
		*(self.position).clone()
	}
}

impl INode for Group {
	fn is_visible(&self) -> bool {
		self.is_visible
	}
	fn is_locked(&self) -> bool {
		self.is_locked
	}
	fn is_folded(&self) -> bool {
		self.is_folded
	}
}

impl<'a> Renamable<'a> for Group {
	fn rename(&self, new_name: &'a str) -> Result<(Patch, Patch), RenameError> {
		if *self.name == new_name {
			Err(RenameError::Unchanged)
		} else {
			Ok((
				Patch::Rename(RenamePatch {
					target: self.id,
					name: new_name.to_owned(),
				}),
				Patch::Rename(RenamePatch {
					target: self.id,
					name: (*self.name).to_owned(),
				}),
			))
		}
	}
}

impl IVisible for Group {
	fn set_visibility(&self, visible: bool) -> Result<(Patch, Patch), SetVisibilityError> {
		if self.is_visible == visible {
			Err(SetVisibilityError::Unchanged)
		} else {
			Ok((
				Patch::SetVisibility(SetVisibilityPatch {
					target: self.id,
					visibility: visible,
				}),
				Patch::SetVisibility(SetVisibilityPatch {
					target: self.id,
					visibility: self.is_visible,
				}),
			))
		}
	}
}

impl ILockable for Group {
	fn set_lock(&self, lock: bool) -> Result<(Patch, Patch), SetLockError> {
		if self.is_locked == lock {
			Err(SetLockError::Unchanged)
		} else {
			Ok((
				Patch::SetLock(SetLockPatch {
					target: self.id,
					lock: lock,
				}),
				Patch::SetLock(SetLockPatch {
					target: self.id,
					lock: self.is_locked,
				}),
			))
		}
	}
}

impl IFoldable for Group {
	fn set_fold(&self, folded: bool) -> Result<(Patch, Patch), SetFoldError> {
		if self.is_folded == folded {
			Err(SetFoldError::Unchanged)
		} else {
			Ok((
				Patch::SetFold(SetFoldPatch {
					target: self.id,
					folded: folded,
				}),
				Patch::SetFold(SetFoldPatch {
					target: self.id,
					folded: self.is_folded,
				}),
			))
		}
	}
}

impl IPatchable for Group {
	fn patch(&self, patch: &Patch) -> Option<Box<Self>> {
		if patch.target() == self.id {
			return match patch {
				Patch::Rename(patch) => Some(Box::new(Group {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: self.is_locked,
					is_folded: self.is_folded,
					name: Arc::new(patch.name.clone()),
					position: self.position.clone(),
					children: self.children.clone(),
				})),
				Patch::SetVisibility(patch) => Some(Box::new(Group {
					id: self.id,
					is_visible: patch.visibility,
					is_locked: self.is_locked,
					is_folded: self.is_folded,
					name: self.name.clone(),
					position: self.position.clone(),
					children: self.children.clone(),
				})),
				Patch::SetLock(patch) => Some(Box::new(Group {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: patch.lock,
					is_folded: self.is_folded,
					name: self.name.clone(),
					position: self.position.clone(),
					children: self.children.clone(),
				})),
				Patch::SetFold(patch) => Some(Box::new(Group {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: self.is_locked,
					is_folded: patch.folded,
					name: self.name.clone(),
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
						is_visible: self.is_visible,
						is_locked: self.is_locked,
						is_folded: self.is_folded,
						name: self.name.clone(),
						position: self.position.clone(),
						children: Arc::new(children),
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
						is_visible: self.is_visible,
						is_locked: self.is_locked,
						is_folded: self.is_folded,
						name: self.name.clone(),
						position: self.position.clone(),
						children: Arc::new(children),
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
						is_visible: self.is_visible,
						is_locked: self.is_locked,
						is_folded: self.is_folded,
						name: self.name.clone(),
						position: self.position.clone(),
						children: Arc::new(children),
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
						Arc::new(new_child)
					}
					None => child.clone(),
				})
				.collect::<Vec<_>>();
			if mutated {
				return Some(Box::new(Group {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: self.is_locked,
					is_folded: self.is_folded,
					name: Arc::clone(&self.name),
					children: Arc::new(children),
					position: Arc::clone(&self.position),
				}));
			}
		}
		return None;
	}
}
