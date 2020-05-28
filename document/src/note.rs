use crate::document::IDocument;
use crate::patch::*;
use crate::INode;
use math::Vec2;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
	pub id: Uuid,
	pub is_visible: bool,
	pub is_locked: bool,
	pub note: Arc<String>,
	pub position: Arc<Vec2<f32>>,
}

impl Note {
	pub fn new(id: Option<Uuid>, note: &str, position: Vec2<f32>) -> Note {
		Note {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			is_visible: true,
			is_locked: false,
			note: Arc::new(note.to_owned()),
			position: Arc::new(position),
		}
	}
}

impl IDocument for Note {
	fn position(&self) -> Vec2<f32> {
		*(self.position).clone()
	}
}

impl INode for Note {
	fn is_visible(&self) -> bool {
		self.is_visible
	}
	fn is_locked(&self) -> bool {
		self.is_locked
	}
}

impl<'a> Renamable<'a> for Note {
	fn rename(&self, new_name: &'a str) -> Result<(Patch, Patch), RenameError> {
		if *self.note == new_name {
			Err(RenameError::Unchanged)
		} else {
			Ok((
				Patch::Rename(RenamePatch {
					target: self.id,
					name: new_name.to_owned(),
				}),
				Patch::Rename(RenamePatch {
					target: self.id,
					name: (*self.note).to_owned(),
				}),
			))
		}
	}
}

impl IVisible for Note {
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

impl ILockable for Note {
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

impl IPatchable for Note {
	fn patch(&self, patch: &Patch) -> Option<Box<Self>> {
		if patch.target() == self.id {
			return match patch {
				Patch::Rename(patch) => Some(Box::new(Note {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: self.is_locked,
					note: Arc::new(patch.name.clone()),
					position: self.position.clone(),
				})),
				Patch::SetVisibility(patch) => Some(Box::new(Note {
					id: self.id,
					is_visible: patch.visibility,
					is_locked: self.is_locked,
					note: self.note.clone(),
					position: self.position.clone(),
				})),
				Patch::SetLock(patch) => Some(Box::new(Note {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: patch.lock,
					note: self.note.clone(),
					position: self.position.clone(),
				})),
				_ => None,
			};
		}
		return None;
	}
}
