use crate as document;
use crate::prelude::*;

#[derive(DocumentNode, Debug, Clone, Serialize, Deserialize)]
pub struct Note {
	pub id: Uuid,
	pub position: Arc<Vec2<u32>>,
	pub visible: bool,
	pub locked: bool,
	pub name: Arc<String>,
}

impl Name for Note {
	fn name(&self) -> String {
		(*self.name).clone()
	}
	fn rename(&self, name: String) -> Option<patch::PatchPair> {
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

impl Position for Note {
	fn position(&self) -> Vec2<u32> {
		*self.position
	}
	fn translate(&self, position: Vec2<u32>) -> Option<patch::PatchPair> {
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

impl Size for Note {}

impl Visible for Note {
	fn visible(&self) -> bool {
		self.visible
	}
	fn set_visibility(&self, visibility: bool) -> Option<patch::PatchPair> {
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

impl Locked for Note {
	fn locked(&self) -> bool {
		self.locked
	}
	fn set_lock(&self, locked: bool) -> Option<patch::PatchPair> {
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

impl Folded for Note {}

impl patch::Patchable for Note {
	fn patch(&self, patch: &patch::PatchType) -> Option<NodeType> {
		if patch.as_patch().target() == self.id {
			let mut patched = Note {
				id: self.id,
				position: self.position.clone(),
				visible: self.visible,
				locked: self.locked,
				name: self.name.clone(),
			};
			match patch {
				patch::PatchType::Rename(patch) => {
					patched.name = Arc::new(patch.name.clone());
				}
				patch::PatchType::Translate(patch) => {
					patched.position = Arc::new(patch.position);
				}
				patch::PatchType::SetVisible(patch) => {
					patched.visible = patch.visibility;
				}
				patch::PatchType::SetLock(patch) => {
					patched.locked = patch.locked;
				}
				_ => return None,
			};
			Some(NodeType::Note(patched))
		} else {
			None
		}
	}
}

impl parser::v0::ParseNode for Note {
	fn parse_node<'bytes>(
		row: &parser::v0::IndexRow,
		_items: NodeList,
		_dependencies: NodeList,
		bytes: &'bytes [u8],
	) -> parser::Result<&'bytes [u8], NodeRef> {
		Ok((
			bytes,
			Arc::new(NodeType::Note(Note {
				id: row.id,
				position: Arc::new(row.position),
				visible: row.visible,
				locked: row.locked,
				name: Arc::new(row.name.clone()),
			})),
		))
	}
}

impl parser::v0::WriteNode for Note {
	fn write_node<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
		rows: &mut Vec<parser::v0::IndexRow>,
		_dependencies: &mut Vec<NodeRef>,
	) -> io::Result<usize> {
		let mut row = parser::v0::IndexRow::new(self.id);
		row.chunk_type = NodeKind::Note;
		row.chunk_offset = writer.seek(io::SeekFrom::Current(0))?;
		row.visible = self.visible;
		row.locked = self.locked;
		row.position = *self.position;
		row.name = (*self.name).clone();
		rows.push(row);
		Ok(0)
	}
}
