use crate as document;
use crate::prelude::*;
use nom::{multi::many_m_n, number::complete::le_u8};

#[derive(DocumentNode, Debug, Clone, Serialize, Deserialize)]
pub struct PaletteNode {
	pub id: Uuid,
	pub position: Arc<Vec2<u32>>,
	pub display: bool,
	pub locked: bool,
	pub name: Arc<String>,
	pub colors: Arc<Vec<RGB>>,
}

impl Named for PaletteNode {
	fn name(&self) -> String {
		(*self.name).clone()
	}
	fn rename(&self, name: String) -> Option<CommandPair> {
		Some((
			CommandType::Rename(RenameCommand {
				target: self.id,
				name,
			}),
			CommandType::Rename(RenameCommand {
				target: self.id,
				name: (*self.name).clone(),
			}),
		))
	}
}

impl Positioned for PaletteNode {
	fn position(&self) -> Vec2<u32> {
		*self.position
	}
	fn translate(&self, position: Vec2<u32>) -> Option<CommandPair> {
		Some((
			CommandType::Translate(TranslateCommand {
				target: self.id,
				position,
			}),
			CommandType::Translate(TranslateCommand {
				target: self.id,
				position: *self.position,
			}),
		))
	}
}

impl Sized for PaletteNode {}

impl Displayed for PaletteNode {
	fn display(&self) -> bool {
		self.display
	}
	fn set_display(&self, visibility: bool) -> Option<CommandPair> {
		Some((
			CommandType::SetVisible(SetVisibleCommand {
				target: self.id,
				visibility,
			}),
			CommandType::SetVisible(SetVisibleCommand {
				target: self.id,
				visibility: self.display,
			}),
		))
	}
}

impl Locked for PaletteNode {
	fn locked(&self) -> bool {
		self.locked
	}
	fn set_lock(&self, locked: bool) -> Option<CommandPair> {
		Some((
			CommandType::SetLock(SetLockCommand {
				target: self.id,
				locked,
			}),
			CommandType::SetLock(SetLockCommand {
				target: self.id,
				locked: self.locked,
			}),
		))
	}
}

impl Folded for PaletteNode {}

impl PaletteNode {
	pub fn add_color(&self, color: RGB) -> Option<CommandPair> {
		if self.colors.iter().find(|c| *c == &color).is_some() {
			None
		} else {
			Some((
				CommandType::AddColor(AddColorCommand {
					target: self.id,
					color: color.clone(),
				}),
				CommandType::RemoveColor(RemoveColorCommand {
					target: self.id,
					color,
				}),
			))
		}
	}
	pub fn remove_color(&self, color: RGB) -> Option<CommandPair> {
		let color = self.colors.iter().find(|c| *c == &color);
		match color {
			Some(color) => Some((
				CommandType::RemoveColor(RemoveColorCommand {
					target: self.id,
					color: color.clone(),
				}),
				CommandType::AddColor(AddColorCommand {
					target: self.id,
					color: color.clone(),
				}),
			)),
			None => None,
		}
	}
	pub fn move_color(&self, color: RGB, position: usize) -> Option<CommandPair> {
		let old_position = self.colors.iter().position(|c| c == &color);
		match old_position {
			Some(old_position) => Some((
				CommandType::MoveColor(MoveColorCommand {
					target: self.id,
					color: color.clone(),
					position,
				}),
				CommandType::MoveColor(MoveColorCommand {
					target: self.id,
					color: color,
					position: old_position,
				}),
			)),
			None => None,
		}
	}
}

impl Executable for PaletteNode {
	fn execute(&self, command: &CommandType) -> Option<NodeType> {
		if command.as_command().target() == self.id {
			let mut patched = PaletteNode {
				id: self.id,
				position: self.position.clone(),
				display: self.display,
				locked: self.locked,
				name: self.name.clone(),
				colors: self.colors.clone(),
			};
			match command {
				CommandType::Rename(command) => {
					patched.name = Arc::new(command.name.clone());
				}
				CommandType::Translate(command) => {
					patched.position = Arc::new(command.position);
				}
				CommandType::SetVisible(command) => {
					patched.display = command.visibility;
				}
				CommandType::SetLock(command) => {
					patched.locked = command.locked;
				}
				CommandType::AddColor(command) => {
					let mut colors: Vec<RGB> =
						patched.colors.iter().map(|color| color.clone()).collect();
					colors.push(command.color.clone());
					patched.colors = Arc::new(colors);
				}
				CommandType::RemoveColor(command) => {
					let colors: Vec<RGB> = patched
						.colors
						.iter()
						.filter_map(|color| {
							if color == &command.color {
								None
							} else {
								Some(color.clone())
							}
						})
						.collect();
					patched.colors = Arc::new(colors);
				}
				CommandType::MoveColor(command) => {
					let mut colors: Vec<RGB> =
						patched.colors.iter().map(|color| color.clone()).collect();
					if let Some(index) = colors.iter().position(|color| color == &command.color) {
						let color = colors.remove(index);
						if command.position > colors.len() {
							colors.push(color);
						} else {
							colors.insert(command.position, color);
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

impl parser::v0::ParseNode for PaletteNode {
	fn parse_node<'bytes>(
		row: &parser::v0::IndexRow,
		_items: NodeList,
		_dependencies: NodeList,
		bytes: &'bytes [u8],
	) -> parser::Result<&'bytes [u8], NodeRef> {
		use parser::Parse;
		let (bytes, len) = le_u8(bytes)?;
		let (bytes, colors) = many_m_n(len as usize, len as usize, RGB::parse)(bytes)?;
		Ok((
			bytes,
			Arc::new(NodeType::Palette(PaletteNode {
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

impl parser::v0::WriteNode for PaletteNode {
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
