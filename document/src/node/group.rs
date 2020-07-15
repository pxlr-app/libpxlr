use crate as document;
use crate::prelude::*;
use std::cell::RefCell;

#[derive(DocumentNode, Debug, Clone, Serialize, Deserialize)]
pub struct Group {
	pub id: Uuid,
	pub position: Rc<Vec2<u32>>,
	pub size: Rc<Extent2<u32>>,
	pub visible: bool,
	pub locked: bool,
	pub folded: bool,
	pub name: Rc<String>,
	pub items: Rc<Vec<Rc<RefCell<Box<dyn Node>>>>>,
}

impl Name for Group {
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

impl Position for Group {
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

impl Size for Group {}

impl Visible for Group {
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

impl Locked for Group {
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

impl Folded for Group {
	fn folded(&self) -> bool {
		self.folded
	}
	fn set_fold(&self, folded: bool) -> Option<(patch::SetFold, patch::SetFold)> {
		Some((
			patch::SetFold {
				target: self.id,
				folded,
			},
			patch::SetFold {
				target: self.id,
				folded: self.folded,
			},
		))
	}
}

impl Patchable for Group {
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
			} else if let Some(patch) = patch.downcast::<patch::SetFold>() {
				self.folded = patch.folded;
				true
			} else {
				false
			}
		} else {
			let mut patched = false;
			let items = self
				.items
				.iter()
				.map(|item| {
					patched |= item.borrow_mut().patch(patch);
					item.clone()
				})
				.collect::<Vec<_>>();
			if patched {
				self.items = Rc::new(items);
				true
			} else {
				false
			}
		}
	}
}
