use crate as document;
use crate::prelude::*;

#[derive(DocumentNode, Debug, Clone, Default, Serialize, Deserialize)]
pub struct NoteNode {
	pub id: Uuid,
	pub position: Arc<Vec2<u32>>,
	pub display: bool,
	pub locked: bool,
	pub name: Arc<String>,
}

impl Named for NoteNode {
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

impl Positioned for NoteNode {
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

impl Sized for NoteNode {}

impl Displayed for NoteNode {
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

impl Locked for NoteNode {
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

impl Folded for NoteNode {}

impl Executable for NoteNode {
	fn execute(&self, command: &CommandType) -> Option<NodeType> {
		if command.as_command().target() == self.id {
			let mut patched = self.clone();
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
				_ => return None,
			};
			Some(NodeType::Note(patched))
		} else {
			None
		}
	}
}

impl parser::v0::ParseNode for NoteNode {
	fn parse_node<'bytes>(
		row: &parser::v0::IndexRow,
		_items: NodeList,
		_dependencies: NodeList,
		bytes: &'bytes [u8],
	) -> parser::Result<&'bytes [u8], NodeRef> {
		Ok((
			bytes,
			Arc::new(NodeType::Note(NoteNode {
				id: row.id,
				position: Arc::new(row.position),
				display: row.display,
				locked: row.locked,
				name: Arc::new(row.name.clone()),
			})),
		))
	}
}

impl parser::v0::WriteNode for NoteNode {
	fn write_node<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
		rows: &mut Vec<parser::v0::IndexRow>,
		_dependencies: &mut Vec<NodeRef>,
	) -> io::Result<usize> {
		let mut row = parser::v0::IndexRow::new(self.id);
		row.chunk_type = NodeKind::Note;
		row.chunk_offset = writer.seek(io::SeekFrom::Current(0))?;
		row.display = self.display;
		row.locked = self.locked;
		row.position = *self.position;
		row.name = (*self.name).clone();
		rows.push(row);
		Ok(0)
	}
}
