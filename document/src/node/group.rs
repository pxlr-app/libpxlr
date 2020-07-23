use crate as document;
use crate::prelude::*;

#[derive(DocumentNode, Debug, Clone, Serialize, Deserialize)]
pub struct Group {
	pub id: Uuid,
	pub position: Arc<Vec2<u32>>,
	pub display: bool,
	pub locked: bool,
	pub folded: bool,
	pub name: Arc<String>,
	pub children: Arc<NodeList>,
}

impl Named for Group {
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

impl Positioned for Group {
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

impl Sized for Group {}

impl Displayed for Group {
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

impl Locked for Group {
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

impl Folded for Group {
	fn folded(&self) -> bool {
		self.folded
	}
	fn set_fold(&self, folded: bool) -> Option<patch::PatchPair> {
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

impl Group {
	pub fn add_child(&self, child: NodeRef) -> Option<patch::PatchPair> {
		if self
			.children
			.iter()
			.find(|child| child.as_node().id() == child.as_node().id())
			.is_some() || child.as_documentnode().is_none()
		{
			None
		} else {
			Some((
				patch::PatchType::AddChild(patch::AddChild {
					target: self.id,
					child: child.clone(),
				}),
				patch::PatchType::RemoveChild(patch::RemoveChild {
					target: self.id,
					child_id: child.as_node().id(),
				}),
			))
		}
	}
	pub fn remove_child(&self, child_id: Uuid) -> Option<patch::PatchPair> {
		let child = self
			.children
			.iter()
			.find(|child| child.as_node().id() == child.as_node().id());
		match child {
			Some(child) => Some((
				patch::PatchType::RemoveChild(patch::RemoveChild {
					target: self.id,
					child_id: child_id,
				}),
				patch::PatchType::AddChild(patch::AddChild {
					target: self.id,
					child: child.clone(),
				}),
			)),
			None => None,
		}
	}
	pub fn move_child(&self, child_id: Uuid, position: usize) -> Option<patch::PatchPair> {
		let old_position = self
			.children
			.iter()
			.position(|child| child.as_node().id() == child_id);
		match old_position {
			Some(old_position) => Some((
				patch::PatchType::MoveChild(patch::MoveChild {
					target: self.id,
					child_id,
					position,
				}),
				patch::PatchType::MoveChild(patch::MoveChild {
					target: self.id,
					child_id,
					position: old_position,
				}),
			)),
			None => None,
		}
	}
}

impl patch::Patchable for Group {
	fn patch(&self, patch: &patch::PatchType) -> Option<NodeType> {
		let mut patched = self.clone();
		if patch.as_patch().target() == self.id {
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
				patch::PatchType::SetFold(patch) => {
					patched.folded = patch.folded;
				}
				patch::PatchType::AddChild(patch) => {
					let mut children: NodeList =
						patched.children.iter().map(|child| child.clone()).collect();
					children.push(patch.child.clone());
					patched.children = Arc::new(children);
				}
				patch::PatchType::RemoveChild(patch) => {
					let children: NodeList = patched
						.children
						.iter()
						.filter_map(|child| {
							if child.as_node().id() == patch.child_id {
								None
							} else {
								Some(child.clone())
							}
						})
						.collect();
					patched.children = Arc::new(children);
				}
				patch::PatchType::MoveChild(patch) => {
					let mut children: NodeList =
						patched.children.iter().map(|child| child.clone()).collect();
					let child = children.remove(patch.position);
					if patch.position > children.len() {
						children.push(child);
					} else {
						children.insert(patch.position, child);
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
					.map(|child| match child.as_node().patch(patch) {
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

impl parser::v0::ParseNode for Group {
	fn parse_node<'bytes>(
		row: &parser::v0::IndexRow,
		children: NodeList,
		_dependencies: NodeList,
		bytes: &'bytes [u8],
	) -> parser::Result<&'bytes [u8], NodeRef> {
		let mut children = children;
		Ok((
			bytes,
			Arc::new(NodeType::Group(Group {
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

impl parser::v0::WriteNode for Group {
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
