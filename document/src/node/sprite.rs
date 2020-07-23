use crate as document;
use crate::prelude::*;
use nom::number::complete::le_u8;

#[derive(DocumentNode, Debug, Clone, Serialize, Deserialize)]
pub struct Sprite {
	pub id: Uuid,
	pub position: Arc<Vec2<u32>>,
	pub size: Arc<Extent2<u32>>,
	pub color_mode: ColorMode,
	pub palette: Option<Weak<NodeType>>,
	pub visible: bool,
	pub locked: bool,
	pub folded: bool,
	pub name: Arc<String>,
	pub children: Arc<NodeList>,
}

impl Name for Sprite {
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

impl Position for Sprite {
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

impl Size for Sprite {
	fn size(&self) -> Extent2<u32> {
		*self.size
	}
	fn resize(&self, target: Extent2<u32>) -> Option<patch::PatchPair> {
		Some((
			patch::PatchType::Resize(patch::Resize {
				target: self.id,
				size: target,
			}),
			patch::PatchType::Resize(patch::Resize {
				target: self.id,
				size: *self.size,
			}),
		))
	}
}

impl Visible for Sprite {
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

impl Locked for Sprite {
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

impl Folded for Sprite {
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

impl SpriteNode for Sprite {}

impl Sprite {
	pub fn add_child(&self, child: NodeRef) -> Option<patch::PatchPair> {
		if self
			.children
			.iter()
			.find(|child| child.as_node().id() == child.as_node().id())
			.is_some() || child.as_spritenode().is_none()
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
	pub fn set_palette(&self, palette: NodeRef) -> Option<patch::PatchPair> {
		Some((
			patch::PatchType::SetPalette(patch::SetPalette {
				target: self.id,
				palette,
			}),
			patch::PatchType::UnsetPalette(patch::UnsetPalette { target: self.id }),
		))
	}
	pub fn unset_palette(&self) -> Option<patch::PatchPair> {
		if let Some(palette) = &self.palette {
			if let Some(palette) = palette.upgrade() {
				Some((
					patch::PatchType::UnsetPalette(patch::UnsetPalette { target: self.id }),
					patch::PatchType::SetPalette(patch::SetPalette {
						target: self.id,
						palette: palette.clone(),
					}),
				))
			} else {
				None
			}
		} else {
			None
		}
	}
}

impl patch::Patchable for Sprite {
	fn patch(&self, patch: &patch::PatchType) -> Option<NodeType> {
		let mut patched = Sprite {
			id: self.id,
			position: self.position.clone(),
			size: self.size.clone(),
			color_mode: self.color_mode.clone(),
			palette: self.palette.clone(),
			visible: self.visible,
			locked: self.locked,
			folded: self.folded,
			name: self.name.clone(),
			children: self.children.clone(),
		};
		if patch.as_patch().target() == self.id {
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
					if let Some(index) = children
						.iter()
						.position(|child| child.as_node().id() == patch.child_id)
					{
						let child = children.remove(index);
						if patch.position > children.len() {
							children.push(child);
						} else {
							children.insert(patch.position, child);
						}
						patched.children = Arc::new(children);
					} else {
						return None;
					}
				}
				patch::PatchType::SetPalette(patch) => {
					patched.palette = Some(Arc::downgrade(&patch.palette));
				}
				patch::PatchType::UnsetPalette(_) => {
					patched.palette = None;
				}
				_ => return None,
			};
			Some(NodeType::Sprite(patched))
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
				Some(NodeType::Sprite(patched))
			} else {
				None
			}
		}
	}
}

impl parser::v0::ParseNode for Sprite {
	fn parse_node<'bytes>(
		row: &parser::v0::IndexRow,
		children: NodeList,
		dependencies: NodeList,
		bytes: &'bytes [u8],
	) -> parser::Result<&'bytes [u8], NodeRef> {
		use parser::Parse;
		let mut children = children;
		let (bytes, color_mode) = ColorMode::parse(bytes)?;
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
			Arc::new(NodeType::Sprite(Sprite {
				id: row.id,
				position: Arc::new(row.position),
				size: Arc::new(row.size),
				color_mode: color_mode,
				palette: palette,
				visible: row.visible,
				locked: row.locked,
				folded: row.folded,
				name: Arc::new(row.name.clone()),
				children: Arc::new(children.drain(..).map(|child| child.clone()).collect()),
			})),
		))
	}
}

impl parser::v0::WriteNode for Sprite {
	fn write_node<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
		rows: &mut Vec<parser::v0::IndexRow>,
		dependencies: &mut Vec<NodeRef>,
	) -> io::Result<usize> {
		use parser::Write;
		let size = self.color_mode.write(writer)?;
		if let Some(palette) = &self.palette {
			if let Some(palette) = palette.upgrade() {
				writer.write(&1u8.to_le_bytes())?;
				palette.as_node().id().write(writer)?;
				dependencies.push(palette.clone());
			} else {
				writer.write(&0u8.to_le_bytes())?;
			}
		} else {
			writer.write(&0u8.to_le_bytes())?;
		}
		let mut row = parser::v0::IndexRow::new(self.id);
		row.chunk_type = NodeKind::Sprite;
		row.chunk_offset = writer.seek(io::SeekFrom::Current(0))?;
		row.chunk_size = size as u32;
		row.visible = self.visible;
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
