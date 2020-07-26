use crate as document;
use crate::prelude::*;
use nom::{bytes::complete::take, number::complete::le_u8};

#[derive(SpriteNode, Debug, Clone, Serialize, Deserialize)]
pub struct Canvas {
	pub id: Uuid,
	pub size: Arc<Extent2<u32>>,
	pub palette: Option<Weak<NodeType>>,
	pub display: bool,
	pub locked: bool,
	pub name: Arc<String>,
	pub channels: Channel,
	pub data: Arc<Vec<u8>>,
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
				channels: self.channels,
				data: (*self.data).to_owned(),
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

impl HasChannels for Canvas {
	fn channels(&self) -> Channel {
		self.channels
	}
}

impl Canvas {
	pub fn set_channels(&self, channels: Channel) -> Option<patch::PatchPair> {
		if self.channels == channels {
			None
		} else {
			Some((
				patch::PatchType::SetChannels(patch::SetChannels {
					target: self.id,
					channels,
				}),
				patch::PatchType::SetChannels(patch::SetChannels {
					target: self.id,
					channels: self.channels,
				}),
			))
		}
	}
	pub fn set_palette(&self, palette: Option<NodeRef>) -> Option<patch::PatchPair> {
		Some((
			patch::PatchType::SetPalette(patch::SetPalette {
				target: self.id,
				palette,
			}),
			patch::PatchType::SetPalette(patch::SetPalette {
				target: self.id,
				palette: match self.palette.clone().map(|weak| weak.upgrade()) {
					Some(Some(node)) => Some(node.clone()),
					_ => None,
				},
			}),
		))
	}
	pub fn apply_stencil(
		&self,
		offset: Vec2<u32>,
		channels: Channel,
		stencil: Stencil2,
	) -> Option<patch::PatchPair> {
		Some((
			patch::PatchType::ApplyStencil2(patch::ApplyStencil2 {
				target: self.id,
				offset,
				channels,
				stencil,
			}),
			patch::PatchType::RestoreCanvas(patch::RestoreCanvas {
				target: self.id,
				channels: self.channels,
				data: (*self.data).to_owned(),
			}),
		))
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
					match &patch.palette {
						Some(node) => patched.palette = Some(Arc::downgrade(node)),
						None => patched.palette = None,
					};
				}
				patch::PatchType::SetChannels(patch) => {
					patched.channels = patch.channels;
				}
				patch::PatchType::RestoreCanvas(patch) => {
					patched.channels = patch.channels;
					patched.data = Arc::new(patch.data.to_owned());
				}
				patch::PatchType::Resize(_patch) => unimplemented!(),
				patch::PatchType::Crop(_patch) => unimplemented!(),
				patch::PatchType::ApplyStencil2(_patch) => unimplemented!(),
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
		let (bytes, channels) = Channel::parse(bytes)?;
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
		let (bytes, data) = take(len)(bytes)?;
		Ok((
			bytes,
			Arc::new(NodeType::Canvas(Canvas {
				id: row.id,
				size: Arc::new(row.size),
				palette: palette,
				display: row.display,
				locked: row.locked,
				name: Arc::new(row.name.clone()),
				channels: channels,
				data: Arc::new(data.to_owned()),
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
		let mut size = self.channels.write(writer)?;
		if let Some(Some(palette)) = self.palette.clone().map(|weak| weak.upgrade()) {
			size += writer.write(&1u8.to_le_bytes())?;
			size += palette.as_node().id().write(writer)?;
			dependencies.push(palette.clone());
		} else {
			size += writer.write(&0u8.to_le_bytes())?;
		}
		size += writer.write(&self.data.as_slice())?;

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
