use crate as document;
use crate::prelude::*;
use nom::{multi::many_m_n, number::complete::le_u8};

#[derive(DocumentNode, Debug, Clone, Serialize, Deserialize)]
pub struct Palette {
	pub id: Uuid,
	pub position: Arc<Vec2<u32>>,
	pub display: bool,
	pub locked: bool,
	pub name: Arc<String>,
	pub colors: Arc<Vec<RGBA>>,
}

impl Named for Palette {
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

impl Positioned for Palette {
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

impl Sized for Palette {}

impl Displayed for Palette {
	fn display(&self) -> bool {
		self.display
	}
	fn set_display(&self, visibility: bool) -> Option<patch::PatchPair> {
		Some((
			patch::PatchType::SetVisible(patch::SetVisible {
				target: self.id,
				visibility,
			}),
			patch::PatchType::SetVisible(patch::SetVisible {
				target: self.id,
				visibility: self.display,
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

impl Palette {
	pub fn add_color(&self, color: RGBA) -> Option<patch::PatchPair> {
		if self.colors.iter().find(|c| *c == &color).is_some() {
			None
		} else {
			Some((
				patch::PatchType::AddColor(patch::AddColor {
					target: self.id,
					color: color.clone(),
				}),
				patch::PatchType::RemoveColor(patch::RemoveColor {
					target: self.id,
					color,
				}),
			))
		}
	}
	pub fn remove_color(&self, color: RGBA) -> Option<patch::PatchPair> {
		let color = self.colors.iter().find(|c| *c == &color);
		match color {
			Some(color) => Some((
				patch::PatchType::RemoveColor(patch::RemoveColor {
					target: self.id,
					color: color.clone(),
				}),
				patch::PatchType::AddColor(patch::AddColor {
					target: self.id,
					color: color.clone(),
				}),
			)),
			None => None,
		}
	}
	pub fn move_color(&self, color: RGBA, position: usize) -> Option<patch::PatchPair> {
		let old_position = self.colors.iter().position(|c| c == &color);
		match old_position {
			Some(old_position) => Some((
				patch::PatchType::MoveColor(patch::MoveColor {
					target: self.id,
					color: color.clone(),
					position,
				}),
				patch::PatchType::MoveColor(patch::MoveColor {
					target: self.id,
					color: color,
					position: old_position,
				}),
			)),
			None => None,
		}
	}
}

impl patch::Patchable for Palette {
	fn patch(&self, patch: &patch::PatchType) -> Option<NodeType> {
		if patch.as_patch().target() == self.id {
			let mut patched = Palette {
				id: self.id,
				position: self.position.clone(),
				display: self.display,
				locked: self.locked,
				name: self.name.clone(),
				colors: self.colors.clone(),
			};
			match patch {
				patch::PatchType::Rename(patch) => {
					patched.name = Arc::new(patch.name.clone());
				}
				patch::PatchType::Translate(patch) => {
					patched.position = Arc::new(patch.position);
				}
				patch::PatchType::SetVisible(patch) => {
					patched.display = patch.visibility;
				}
				patch::PatchType::SetLock(patch) => {
					patched.locked = patch.locked;
				}
				patch::PatchType::AddColor(patch) => {
					let mut colors: Vec<RGBA> =
						patched.colors.iter().map(|color| color.clone()).collect();
					colors.push(patch.color.clone());
					patched.colors = Arc::new(colors);
				}
				patch::PatchType::RemoveColor(patch) => {
					let colors: Vec<RGBA> = patched
						.colors
						.iter()
						.filter_map(|color| {
							if color == &patch.color {
								None
							} else {
								Some(color.clone())
							}
						})
						.collect();
					patched.colors = Arc::new(colors);
				}
				patch::PatchType::MoveColor(patch) => {
					let mut colors: Vec<RGBA> =
						patched.colors.iter().map(|color| color.clone()).collect();
					if let Some(index) = colors.iter().position(|color| color == &patch.color) {
						let color = colors.remove(index);
						if patch.position > colors.len() {
							colors.push(color);
						} else {
							colors.insert(patch.position, color);
						}
						patched.colors = Arc::new(colors);
					} else {
						return None;
					}
				}
				_ => return None,
			};
			Some(NodeType::Palette(patched))
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
				display: row.display,
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
		row.display = self.display;
		row.locked = self.locked;
		row.position = *self.position;
		row.name = (*self.name).clone();
		rows.push(row);
		Ok(0)
	}
}
