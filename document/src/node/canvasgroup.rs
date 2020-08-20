use crate as document;
use crate::prelude::*;
use nom::number::complete::{le_f32, le_u8};

#[derive(DocumentNode, Debug, Clone, Serialize, Deserialize)]
pub struct CanvasGroupNode {
	pub id: Uuid,
	pub position: Arc<Vec2<u32>>,
	pub size: Arc<Extent2<u32>>,
	pub opacity: f32,
	pub components: u8,
	pub palette: Option<Weak<NodeType>>,
	pub display: bool,
	pub locked: bool,
	pub folded: bool,
	pub name: Arc<String>,
	pub children: Arc<NodeList>,
}

impl Named for CanvasGroupNode {
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

impl Positioned for CanvasGroupNode {
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

impl Sized for CanvasGroupNode {
	fn size(&self) -> Extent2<u32> {
		*self.size
	}
	fn resize(&self, target: Extent2<u32>) -> Option<CommandPair> {
		Some((
			CommandType::Resize(ResizeCommand {
				target: self.id,
				size: target,
			}),
			CommandType::Resize(ResizeCommand {
				target: self.id,
				size: *self.size,
			}),
		))
	}
}

impl Displayed for CanvasGroupNode {
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

impl Locked for CanvasGroupNode {
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

impl Folded for CanvasGroupNode {
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

impl Cropable for CanvasGroupNode {
	fn crop(&self, offset: Vec2<u32>, size: Extent2<u32>) -> Option<CommandPair> {
		Some((
			CommandType::Crop(CropCommand {
				target: self.id,
				offset,
				size,
			}),
			CommandType::RestoreSprite(RestoreCanvasGroupCommand {
				target: self.id,
				children: self
					.children
					.iter()
					.map(|child| child.as_spritenode().unwrap().crop(offset, size).unwrap().1)
					.collect(),
			}),
		))
	}
}

impl SpriteNode for CanvasGroupNode {}

impl Transparent for CanvasGroupNode {
	fn opacity(&self) -> f32 {
		self.opacity
	}
	fn set_opacity(&self, opacity: f32) -> Option<CommandPair> {
		Some((
			CommandType::SetOpacity(SetOpacityCommand {
				target: self.id,
				opacity,
			}),
			CommandType::SetOpacity(SetOpacityCommand {
				target: self.id,
				opacity: self.opacity,
			}),
		))
	}
}

impl HasComponents for CanvasGroupNode {
	fn components(&self) -> u8 {
		self.components
	}
}

impl CanvasGroupNode {
	pub fn add_child(&self, child: NodeRef) -> Option<CommandPair> {
		if self
			.children
			.iter()
			.find(|child| child.as_node().id() == child.as_node().id())
			.is_some() || child.as_spritenode().is_none()
			|| child.as_spritenode().unwrap().components() != self.components
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
	pub fn set_components(&self, components: u8) -> Option<CommandPair> {
		if self.components == components {
			None
		} else {
			Some((
				CommandType::SetComponents(SetComponentsCommand {
					target: self.id,
					components,
				}),
				CommandType::SetComponents(SetComponentsCommand {
					target: self.id,
					components: self.components,
				}),
			))
		}
	}
	pub fn set_palette(&self, palette: Option<NodeRef>) -> Option<CommandPair> {
		Some((
			CommandType::SetPaletteNode(SetPaletteNodeCommand {
				target: self.id,
				palette,
			}),
			CommandType::SetPaletteNode(SetPaletteNodeCommand {
				target: self.id,
				palette: match self.palette.clone().map(|weak| weak.upgrade()) {
					Some(Some(node)) => Some(node.clone()),
					_ => None,
				},
			}),
		))
	}
}

impl Executable for CanvasGroupNode {
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
					if let Some(index) = children
						.iter()
						.position(|child| child.as_node().id() == command.child_id)
					{
						let child = children.remove(index);
						if command.position > children.len() {
							children.push(child);
						} else {
							children.insert(command.position, child);
						}
						patched.children = Arc::new(children);
					} else {
						return None;
					}
				}
				CommandType::SetPaletteNode(command) => {
					let children = patched
						.children
						.iter()
						.map(|child| {
							match child.as_node().execute(&CommandType::SetPaletteNode(
								SetPaletteNodeCommand {
									target: child.as_node().id(),
									palette: command.palette.clone(),
								},
							)) {
								Some(new_child) => Arc::new(new_child),
								None => child.clone(),
							}
						})
						.collect();
					patched.children = Arc::new(children);
					match &command.palette {
						Some(node) => patched.palette = Some(Arc::downgrade(node)),
						None => patched.palette = None,
					};
				}
				CommandType::SetOpacity(command) => {
					patched.opacity = command.opacity;
				}
				CommandType::SetComponents(command) => {
					let children = patched
						.children
						.iter()
						.map(|child| {
							match child.as_node().execute(&CommandType::SetComponents(
								SetComponentsCommand {
									target: child.as_node().id(),
									components: command.components,
								},
							)) {
								Some(new_child) => Arc::new(new_child),
								None => child.clone(),
							}
						})
						.collect();
					patched.children = Arc::new(children);
					patched.components = command.components;
				}
				_ => return None,
			};
			Some(NodeType::CanvasGroup(patched))
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
				Some(NodeType::CanvasGroup(patched))
			} else {
				None
			}
		}
	}
}

impl parser::v0::ParseNode for CanvasGroupNode {
	fn parse_node<'bytes>(
		row: &parser::v0::IndexRow,
		mut children: NodeList,
		dependencies: NodeList,
		bytes: &'bytes [u8],
	) -> parser::Result<&'bytes [u8], NodeRef> {
		use parser::Parse;
		let (bytes, opacity) = le_f32(bytes)?;
		let (bytes, components) = le_u8(bytes)?;
		let (bytes, has_palette) = le_u8(bytes)?;
		let (bytes, palette) = if has_palette == 1 {
			let (bytes, palette_id) = Uuid::parse(bytes)?;
			let palette = dependencies
				.iter()
				.find(|node| node.as_node().id() == palette_id)
				.map(|node| Arc::downgrade(node));
			(bytes, palette)
		} else {
			(bytes, None)
		};
		Ok((
			bytes,
			Arc::new(NodeType::CanvasGroup(CanvasGroupNode {
				id: row.id,
				position: Arc::new(row.position),
				size: Arc::new(row.size),
				opacity: opacity,
				components: components,
				palette: palette,
				display: row.display,
				locked: row.locked,
				folded: row.folded,
				name: Arc::new(row.name.clone()),
				children: Arc::new(children.drain(..).map(|child| child.clone()).collect()),
			})),
		))
	}
}

impl parser::v0::WriteNode for CanvasGroupNode {
	fn write_node<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
		rows: &mut Vec<parser::v0::IndexRow>,
		dependencies: &mut Vec<NodeRef>,
	) -> io::Result<usize> {
		use parser::Write;
		let mut size = 5usize;
		writer.write(&self.opacity.to_le_bytes())?;
		writer.write(&self.components.to_le_bytes())?;
		if let Some(Some(palette)) = self.palette.clone().map(|weak| weak.upgrade()) {
			size += writer.write(&1u8.to_le_bytes())?;
			size += palette.as_node().id().write(writer)?;
			dependencies.push(palette.clone());
		} else {
			size += writer.write(&0u8.to_le_bytes())?;
		}

		let mut row = parser::v0::IndexRow::new(self.id);
		row.chunk_type = NodeKind::CanvasGroup;
		row.chunk_offset = writer.seek(io::SeekFrom::Current(0))?;
		row.chunk_size = size as u32;
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
		Ok(size)
	}
}
