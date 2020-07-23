use crate as document;
use crate::prelude::*;
use nom::{multi::many_m_n, number::complete::le_u8};

#[derive(DocumentNode, Debug, Clone, Serialize, Deserialize)]
pub struct Palette {
	pub id: Uuid,
	pub position: Arc<Vec2<u32>>,
	pub visible: bool,
	pub locked: bool,
	pub name: Arc<String>,
	pub colors: Arc<Vec<RGBA>>,
}

impl Name for Palette {
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

impl Position for Palette {
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

impl Size for Palette {}

impl Visible for Palette {
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

impl Locked for Palette {
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

impl Folded for Palette {}

impl patch::Patchable for Palette {
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

impl parser::v0::ParseNode for Palette {
	fn parse_node<'bytes>(
		row: &parser::v0::IndexRow,
		_items: NodeList,
		_dependencies: NodeList,
		bytes: &'bytes [u8],
	) -> parser::Result<&'bytes [u8], NodeRef> {
		use parser::Parse;
		let (bytes, len) = le_u8(bytes)?;
		let (bytes, colors) = many_m_n(len as usize, len as usize, RGBA::parse)(bytes)?;
		Ok((
			bytes,
			Arc::new(NodeType::Palette(Palette {
				id: row.id,
				position: Arc::new(row.position),
				visible: row.visible,
				locked: row.locked,
				name: Arc::new(row.name.clone()),
				colors: Arc::new(colors),
			})),
		))
	}
}

impl parser::v0::WriteNode for Palette {
	fn write_node<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
		rows: &mut Vec<parser::v0::IndexRow>,
		_dependencies: &mut Vec<NodeRef>,
	) -> io::Result<usize> {
		use parser::Write;
		writer.write(&(self.colors.len() as u8).to_le_bytes())?;
		for color in self.colors.iter() {
			color.write(writer)?;
		}
		let mut row = parser::v0::IndexRow::new(self.id);
		row.chunk_type = NodeKind::Palette;
		row.chunk_offset = writer.seek(io::SeekFrom::Current(0))?;
		row.visible = self.visible;
		row.locked = self.locked;
		row.position = *self.position;
		row.name = (*self.name).clone();
		rows.push(row);
		Ok(0)
	}
}
