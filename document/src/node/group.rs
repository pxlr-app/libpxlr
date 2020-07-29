use crate as document;
use crate::prelude::*;

#[derive(DocumentNode, Debug, Clone, Serialize, Deserialize)]
pub struct GroupNode {
	pub id: Uuid,
	pub position: Arc<Vec2<u32>>,
	pub display: bool,
	pub locked: bool,
	pub folded: bool,
	pub name: Arc<String>,
	pub children: Arc<NodeList>,
}

impl Named for GroupNode {
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

impl Positioned for GroupNode {
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

impl Sized for GroupNode {}

impl Displayed for GroupNode {
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

impl Locked for GroupNode {
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

impl Folded for GroupNode {
	fn folded(&self) -> bool {
		self.folded
	}
	fn set_fold(&self, folded: bool) -> Option<CommandPair> {
		Some((
			CommandType::SetFold(SetFoldCommand {
				target: self.id,
				folded,
			}),
			CommandType::SetFold(SetFoldCommand {
				target: self.id,
				folded: self.folded,
			}),
		))
	}
}

impl GroupNode {
	pub fn add_child(&self, child: NodeRef) -> Option<CommandPair> {
		if self
			.children
			.iter()
			.find(|child| child.as_node().id() == child.as_node().id())
			.is_some() || child.as_documentnode().is_none()
		{
			None
		} else {
			Some((
				CommandType::AddChild(AddChildCommand {
					target: self.id,
					child: child.clone(),
				}),
				CommandType::RemoveChild(RemoveChildCommand {
					target: self.id,
					child_id: child.as_node().id(),
				}),
			))
		}
	}
	pub fn remove_child(&self, child_id: Uuid) -> Option<CommandPair> {
		let child = self
			.children
			.iter()
			.find(|child| child.as_node().id() == child.as_node().id());
		match child {
			Some(child) => Some((
				CommandType::RemoveChild(RemoveChildCommand {
					target: self.id,
					child_id: child_id,
				}),
				CommandType::AddChild(AddChildCommand {
					target: self.id,
					child: child.clone(),
				}),
			)),
			None => None,
		}
	}
	pub fn move_child(&self, child_id: Uuid, position: usize) -> Option<CommandPair> {
		let old_position = self
			.children
			.iter()
			.position(|child| child.as_node().id() == child_id);
		match old_position {
			Some(old_position) => Some((
				CommandType::MoveChild(MoveChildCommand {
					target: self.id,
					child_id,
					position,
				}),
				CommandType::MoveChild(MoveChildCommand {
					target: self.id,
					child_id,
					position: old_position,
				}),
			)),
			None => None,
		}
	}
}

impl Executable for GroupNode {
	fn execute(&self, command: &CommandType) -> Option<NodeType> {
		let mut patched = self.clone();
		if command.as_command().target() == self.id {
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
				CommandType::SetFold(command) => {
					patched.folded = command.folded;
				}
				CommandType::AddChild(command) => {
					let mut children: NodeList =
						patched.children.iter().map(|child| child.clone()).collect();
					children.push(command.child.clone());
					patched.children = Arc::new(children);
				}
				CommandType::RemoveChild(command) => {
					let children: NodeList = patched
						.children
						.iter()
						.filter_map(|child| {
							if child.as_node().id() == command.child_id {
								None
							} else {
								Some(child.clone())
							}
						})
						.collect();
					patched.children = Arc::new(children);
				}
				CommandType::MoveChild(command) => {
					let mut children: NodeList =
						patched.children.iter().map(|child| child.clone()).collect();
					let child = children.remove(command.position);
					if command.position > children.len() {
						children.push(child);
					} else {
						children.insert(command.position, child);
					}
					patched.children = Arc::new(children);
				}
				_ => return None,
			};
			Some(NodeType::Group(patched))
		} else {
			let mut mutated = false;
			patched.children = Arc::new(
				patched
					.children
					.iter()
					.map(|child| match child.as_node().execute(command) {
						Some(child) => {
							mutated = true;
							Arc::new(child)
						}
						None => child.clone(),
					})
					.collect(),
			);
			if mutated {
				Some(NodeType::Group(patched))
			} else {
				None
			}
		}
	}
}

impl parser::v0::ParseNode for GroupNode {
	fn parse_node<'bytes>(
		row: &parser::v0::IndexRow,
		mut children: NodeList,
		_dependencies: NodeList,
		bytes: &'bytes [u8],
	) -> parser::Result<&'bytes [u8], NodeRef> {
		Ok((
			bytes,
			Arc::new(NodeType::Group(GroupNode {
				id: row.id,
				position: Arc::new(row.position),
				display: row.display,
				locked: row.locked,
				folded: row.folded,
				name: Arc::new(row.name.clone()),
				children: Arc::new(children.drain(..).map(|child| child.clone()).collect()),
			})),
		))
	}
}

impl parser::v0::WriteNode for GroupNode {
	fn write_node<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
		rows: &mut Vec<parser::v0::IndexRow>,
		dependencies: &mut Vec<NodeRef>,
	) -> io::Result<usize> {
		let mut row = parser::v0::IndexRow::new(self.id);
		row.chunk_type = NodeKind::Group;
		row.chunk_offset = writer.seek(io::SeekFrom::Current(0))?;
		row.display = self.display;
		row.locked = self.locked;
		row.folded = self.folded;
		row.position = *self.position;
		row.name = (*self.name).clone();
		for item in self.children.iter() {
			dependencies.push(item.clone());
			row.children.push(item.as_node().id());
		}
		rows.push(row);
		Ok(0)
	}
}
