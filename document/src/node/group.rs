use crate as document;
use crate::prelude::*;

#[derive(DocumentNode, Debug, Clone)]
pub struct Group {
	pub id: Uuid,
	pub position: Arc<Vec2<u32>>,
	pub visible: bool,
	pub locked: bool,
	pub folded: bool,
	pub name: Arc<String>,
	pub items: Arc<NodeList>,
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
				name: (*self.name).clone(),
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

impl patch::Patchable for Group {
	fn patch(&self, patch: &dyn patch::Patch) -> Option<Box<dyn Node>> {
		let mut cloned = Box::new(Group {
			id: self.id,
			position: self.position.clone(),
			visible: self.visible,
			locked: self.locked,
			folded: self.folded,
			name: self.name.clone(),
			items: self.items.clone(),
		});
		if patch.target() == self.id {
			if let Some(patch) = patch.downcast::<patch::Rename>() {
				cloned.name = Arc::new(patch.name.clone());
				return Some(cloned);
			} else if let Some(patch) = patch.downcast::<patch::Translate>() {
				cloned.position = Arc::new(patch.position);
				return Some(cloned);
			} else if let Some(patch) = patch.downcast::<patch::SetVisible>() {
				cloned.visible = patch.visibility;
				return Some(cloned);
			} else if let Some(patch) = patch.downcast::<patch::SetLock>() {
				cloned.locked = patch.locked;
				return Some(cloned);
			} else if let Some(patch) = patch.downcast::<patch::SetFold>() {
				cloned.folded = patch.folded;
				return Some(cloned);
			}
		} else {
			let mut mutated = false;
			cloned.items = Arc::new(
				cloned
					.items
					.iter()
					.map(|item| match item.patch(patch) {
						Some(item) => {
							mutated = true;
							<dyn Node>::from(item)
						}
						None => item.clone(),
					})
					.collect(),
			);
			if mutated {
				return Some(cloned);
			}
		}
		None
	}
}

impl parser::v0::ParseNode for Group {
	fn parse_node<'bytes>(
		row: &parser::v0::IndexRow,
		items: NodeList,
		_dependencies: NodeList,
		bytes: &'bytes [u8],
	) -> parser::Result<&'bytes [u8], Self> {
		let mut items = items;
		Ok((
			bytes,
			Group {
				id: row.id,
				position: Arc::new(row.position),
				visible: row.visible,
				locked: row.locked,
				folded: row.folded,
				name: Arc::new(row.name.clone()),
				items: Arc::new(items.drain(..).map(|item| item.clone()).collect()),
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
		row.position = *self.position;
		row.name = (*self.name).clone();
		for item in self.items.iter() {
			dependencies.push(item.clone());
			row.items.push(item.id());
		}
		rows.push(row);
		Ok(0)
	}
}
