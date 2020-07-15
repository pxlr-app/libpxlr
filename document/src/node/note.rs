use crate as document;
use crate::prelude::*;

#[derive(DocumentNode, Debug, Clone, Serialize, Deserialize)]
pub struct Note {
	pub id: Uuid,
	pub position: Rc<Vec2<u32>>,
	pub visible: bool,
	pub locked: bool,
	pub name: Rc<String>,
}

impl Name for Note {
	fn name(&self) -> String {
		(*self.name).clone()
	}
	fn rename(&self, name: String) -> Option<(patch::Rename, patch::Rename)> {
		Some((
			patch::Rename {
				target: self.id,
				name,
			},
			patch::Rename {
				target: self.id,
				name: (*self.name).to_owned(),
			},
		))
	}
}

impl Position for Note {
	fn position(&self) -> Vec2<u32> {
		*self.position
	}
	fn translate(&self, position: Vec2<u32>) -> Option<(patch::Translate, patch::Translate)> {
		Some((
			patch::Translate {
				target: self.id,
				position,
			},
			patch::Translate {
				target: self.id,
				position: *self.position,
			},
		))
	}
}

impl Size for Note {}

impl Visible for Note {
	fn visible(&self) -> bool {
		self.visible
	}
	fn set_visibility(&self, visibility: bool) -> Option<(patch::SetVisible, patch::SetVisible)> {
		Some((
			patch::SetVisible {
				target: self.id,
				visibility,
			},
			patch::SetVisible {
				target: self.id,
				visibility: self.visible,
			},
		))
	}
}

impl Locked for Note {
	fn locked(&self) -> bool {
		self.locked
	}
	fn set_lock(&self, locked: bool) -> Option<(patch::SetLock, patch::SetLock)> {
		Some((
			patch::SetLock {
				target: self.id,
				locked,
			},
			patch::SetLock {
				target: self.id,
				locked: self.locked,
			},
		))
	}
}

impl Folded for Note {}

impl Patchable for Note {
	fn patch(&mut self, patch: &dyn Patch) -> bool {
		if patch.target() == self.id {
			if let Some(patch) = patch.downcast::<patch::Rename>() {
				self.name = Rc::new(patch.name.clone());
				true
			} else if let Some(patch) = patch.downcast::<patch::Translate>() {
				self.position = Rc::new(patch.position);
				true
			} else if let Some(patch) = patch.downcast::<patch::SetVisible>() {
				self.visible = patch.visibility;
				true
			} else if let Some(patch) = patch.downcast::<patch::SetLock>() {
				self.locked = patch.locked;
				true
			} else {
				false
			}
		} else {
			false
		}
	}
}
