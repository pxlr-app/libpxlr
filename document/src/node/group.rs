use crate as document;
use crate::prelude::*;

#[derive(DocumentNode, Debug, Clone, Serialize, Deserialize)]
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
	fn rename(&self, name: String) -> Option<(patch::PatchType, patch::PatchType)> {
		Some((
			patch::PatchType::Rename(patch::Rename {
				target: self.id,
				name,
			}),
			patch::PatchType::Rename(patch::Rename {
				target: self.id,
				name: (*self.name).clone(),
			}),
		))
	}
}

impl Position for Group {
	fn position(&self) -> Vec2<u32> {
		*self.position
	}
	fn translate(&self, position: Vec2<u32>) -> Option<(patch::PatchType, patch::PatchType)> {
		Some((
			patch::PatchType::Translate(patch::Translate {
				target: self.id,
				position,
			}),
			patch::PatchType::Translate(patch::Translate {
				target: self.id,
				position: *self.position,
			}),
		))
	}
}

impl Size for Group {}

impl Visible for Group {
	fn visible(&self) -> bool {
		self.visible
	}
	fn set_visibility(&self, visibility: bool) -> Option<(patch::PatchType, patch::PatchType)> {
		Some((
			patch::PatchType::SetVisible(patch::SetVisible {
				target: self.id,
				visibility,
			}),
			patch::PatchType::SetVisible(patch::SetVisible {
				target: self.id,
				visibility: self.visible,
			}),
		))
	}
}

impl Locked for Group {
	fn locked(&self) -> bool {
		self.locked
	}
	fn set_lock(&self, locked: bool) -> Option<(patch::PatchType, patch::PatchType)> {
		Some((
			patch::PatchType::SetLock(patch::SetLock {
				target: self.id,
				locked,
			}),
			patch::PatchType::SetLock(patch::SetLock {
				target: self.id,
				locked: self.locked,
			}),
		))
	}
}

impl Folded for Group {
	fn folded(&self) -> bool {
		self.folded
	}
	fn set_fold(&self, folded: bool) -> Option<(patch::PatchType, patch::PatchType)> {
		Some((
			patch::PatchType::SetFold(patch::SetFold {
				target: self.id,
				folded,
			}),
			patch::PatchType::SetFold(patch::SetFold {
				target: self.id,
				folded: self.folded,
			}),
		))
	}
}

impl patch::Patchable for Group {
	fn patch(&self, patch: &patch::PatchType) -> Option<NodeType> {
		let mut cloned = Group {
			id: self.id,
			position: self.position.clone(),
			visible: self.visible,
			locked: self.locked,
			folded: self.folded,
			name: self.name.clone(),
			items: self.items.clone(),
		};
		if patch.as_patch().target() == self.id {
			match patch {
				patch::PatchType::Rename(patch) => {
					cloned.name = Arc::new(patch.name.clone());
				}
				patch::PatchType::Translate(patch) => {
					cloned.position = Arc::new(patch.position);
				}
				patch::PatchType::SetVisible(patch) => {
					cloned.visible = patch.visibility;
				}
				patch::PatchType::SetLock(patch) => {
					cloned.locked = patch.locked;
				}
				patch::PatchType::SetFold(patch) => {
					cloned.folded = patch.folded;
				}
				_ => return None,
			};
			Some(NodeType::Group(cloned))
		} else {
			let mut mutated = false;
			cloned.items = Arc::new(
				cloned
					.items
					.iter()
					.map(|item| match item.as_node().patch(patch) {
						Some(item) => {
							mutated = true;
							Arc::new(item)
						}
						None => item.clone(),
					})
					.collect(),
			);
			if mutated {
				Some(NodeType::Group(cloned))
			} else {
				None
			}
		}
	}
}

impl parser::v0::ParseNode for Group {
	fn parse_node<'bytes>(
		row: &parser::v0::IndexRow,
		items: NodeList,
		_dependencies: NodeList,
		bytes: &'bytes [u8],
	) -> parser::Result<&'bytes [u8], NodeRef> {
		let mut items = items;
		Ok((
			bytes,
			Arc::new(NodeType::Group(Group {
				id: row.id,
				position: Arc::new(row.position),
				visible: row.visible,
				locked: row.locked,
				folded: row.folded,
				name: Arc::new(row.name.clone()),
				items: Arc::new(items.drain(..).map(|item| item.clone()).collect()),
			})),
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
			row.items.push(item.as_node().id());
		}
		rows.push(row);
		Ok(0)
	}
}
