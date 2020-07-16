use crate as document;
use crate::prelude::*;
use std::{cell::RefCell, io};

#[derive(DocumentNode, Debug, Clone, Serialize, Deserialize)]
pub struct Group {
	pub id: Uuid,
	pub position: Vec2<u32>,
	pub visible: bool,
	pub locked: bool,
	pub folded: bool,
	pub name: String,
	pub items: NodeList,
}

impl Name for Group {
	fn name(&self) -> String {
		self.name.clone()
	}
	fn rename(&self, name: String) -> Option<(patch::Rename, patch::Rename)> {
		Some((
			patch::Rename {
				target: self.id,
				name,
			},
			patch::Rename {
				target: self.id,
				name: self.name.clone(),
			},
		))
	}
}

impl Position for Group {
	fn position(&self) -> Vec2<u32> {
		self.position
	}
	fn translate(&self, position: Vec2<u32>) -> Option<(patch::Translate, patch::Translate)> {
		Some((
			patch::Translate {
				target: self.id,
				position,
			},
			patch::Translate {
				target: self.id,
				position: self.position,
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
	fn patch(&mut self, patch: &dyn Patch) {
		if patch.target() == self.id {
			if let Some(patch) = patch.downcast::<patch::Rename>() {
				self.name = patch.name.clone();
			} else if let Some(patch) = patch.downcast::<patch::Translate>() {
				self.position = patch.position;
			} else if let Some(patch) = patch.downcast::<patch::SetVisible>() {
				self.visible = patch.visibility;
			} else if let Some(patch) = patch.downcast::<patch::SetLock>() {
				self.locked = patch.locked;
			} else if let Some(patch) = patch.downcast::<patch::SetFold>() {
				self.folded = patch.folded;
			}
		} else {
			for item in self.items.iter() {
				item.borrow_mut().patch(patch);
			}
		}
	}
}

impl parser::v0::ParseNode for Group {
	fn parse_node<'bytes>(
		row: &parser::v0::IndexRow,
		items: Vec<Box<dyn Node>>,
		_dependencies: NodeList,
		bytes: &'bytes [u8],
	) -> parser::Result<&'bytes [u8], Self> {
		let mut items = items;
		Ok((
			bytes,
			Group {
				id: row.id,
				position: row.position,
				visible: row.visible,
				locked: row.locked,
				folded: row.folded,
				name: row.name.clone(),
				items: items
					.drain(..)
					.map(|item| Rc::new(RefCell::new(item)))
					.collect(),
			},
		))
	}
}

impl parser::v0::WriteNode for Group {
	fn write_node<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
		rows: &mut Vec<parser::v0::IndexRow>,
		dependencies: &mut Vec<NodeRef>,
	) -> io::Result<usize> {
		let mut row = parser::v0::IndexRow::new(self.id);
		row.chunk_offset = writer.seek(io::SeekFrom::Current(0))?;
		row.visible = self.visible;
		row.locked = self.locked;
		row.folded = self.folded;
		row.position = self.position;
		row.name = self.name.clone();
		for item in self.items.iter() {
			dependencies.push(item.clone());
			row.items.push(item.borrow().id());
		}
		rows.push(row);
		Ok(0)
	}
}
