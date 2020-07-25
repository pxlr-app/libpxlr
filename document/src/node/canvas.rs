use crate as document;
use crate::prelude::*;
use nom::{bytes::complete::take, multi::many_m_n, number::complete::le_u8};

#[derive(SpriteNode, Debug, Clone, Serialize, Deserialize)]
pub struct Canvas {
	pub id: Uuid,
	pub size: Arc<Extent2<u32>>,
	pub color_mode: ColorMode,
	pub palette: Option<Weak<NodeType>>,
	pub display: bool,
	pub locked: bool,
	pub name: Arc<String>,
	pub color: Arc<Vec<u8>>,
	pub alpha: Arc<Vec<Grey>>,
	pub has_normal: bool,
	pub normal: Arc<Vec<XYZ>>,
}

impl Named for Canvas {
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

impl Sized for Canvas {
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

impl Cropable for Canvas {
	fn crop(&self, offset: Vec2<u32>, size: Extent2<u32>) -> Option<patch::PatchPair> {
		Some((
			patch::PatchType::Crop(patch::Crop {
				target: self.id,
				offset,
				size,
			}),
			patch::PatchType::RestoreCanvas(patch::RestoreCanvas {
				target: self.id,
				color: (*self.color).to_owned(),
				alpha: (*self.alpha).to_owned(),
				normal: (*self.normal).to_owned(),
			}),
		))
	}
}

impl Displayed for Canvas {
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

impl Locked for Canvas {
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

impl Folded for Canvas {}

impl HasColorMode for Canvas {
	fn color_mode(&self) -> ColorMode {
		self.color_mode
	}
}

impl patch::Patchable for Canvas {
	fn patch(&self, patch: &patch::PatchType) -> Option<NodeType> {
		let mut patched = self.clone();
		if patch.as_patch().target() == self.id {
			match patch {
				patch::PatchType::Rename(patch) => {
					patched.name = Arc::new(patch.name.clone());
				}
				patch::PatchType::SetVisible(patch) => {
					patched.display = patch.visibility;
				}
				patch::PatchType::SetLock(patch) => {
					patched.locked = patch.locked;
				}
				patch::PatchType::SetPalette(patch) => {
					patched.palette = Some(Arc::downgrade(&patch.palette));
				}
				patch::PatchType::UnsetPalette(_) => {
					patched.palette = None;
				}
				patch::PatchType::SetColorMode(patch) => {
					patched.color_mode = patch.color_mode;
				}
				_ => return None,
			};
			Some(NodeType::Canvas(patched))
		} else {
			None
		}
	}
}

impl parser::v0::ParseNode for Canvas {
	fn parse_node<'bytes>(
		row: &parser::v0::IndexRow,
		_children: NodeList,
		dependencies: NodeList,
		bytes: &'bytes [u8],
	) -> parser::Result<&'bytes [u8], NodeRef> {
		use parser::Parse;
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
		let len = (row.size.w as usize) * (row.size.h as usize);
		let (bytes, flags) = le_u8(bytes)?;
		let has_normal = flags > 0;
		let (bytes, alpha) = many_m_n(len, len, Grey::parse)(bytes)?;
		let (bytes, color) = take(len)(bytes)?;
		let (bytes, normal) = if has_normal {
			many_m_n(len, len, XYZ::parse)(bytes)?
		} else {
			(bytes, Vec::new())
		};
		Ok((
			bytes,
			Arc::new(NodeType::Canvas(Canvas {
				id: row.id,
				size: Arc::new(row.size),
				color_mode: color_mode,
				palette: palette,
				display: row.display,
				locked: row.locked,
				name: Arc::new(row.name.clone()),
				color: Arc::new(color.to_owned()),
				alpha: Arc::new(alpha),
				has_normal,
				normal: Arc::new(normal),
			})),
		))
	}
}

impl parser::v0::WriteNode for Canvas {
	fn write_node<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
		rows: &mut Vec<parser::v0::IndexRow>,
		dependencies: &mut Vec<NodeRef>,
	) -> io::Result<usize> {
		use parser::Write;
		let mut size = self.color_mode.write(writer)?;
		if let Some(palette) = &self.palette {
			if let Some(palette) = palette.upgrade() {
				size += writer.write(&1u8.to_le_bytes())?;
				size += palette.as_node().id().write(writer)?;
				dependencies.push(palette.clone());
			} else {
				size += writer.write(&0u8.to_le_bytes())?;
			}
		} else {
			size += writer.write(&0u8.to_le_bytes())?;
		}
		let flags = if self.has_normal { 1u8 } else { 0u8 };
		size += writer.write(&flags.to_le_bytes())?;
		size += writer.write(&self.color.as_slice())?;
		for a in self.alpha.iter() {
			size += a.write(writer)?;
		}
		if self.has_normal {
			for n in self.normal.iter() {
				size += n.write(writer)?;
			}
		}

		let mut row = parser::v0::IndexRow::new(self.id);
		row.chunk_type = NodeKind::Sprite;
		row.chunk_offset = writer.seek(io::SeekFrom::Current(0))?;
		row.chunk_size = size as u32;
		row.display = self.display;
		row.locked = self.locked;
		row.name = (*self.name).clone();
		rows.push(row);
		Ok(size)
	}
}
