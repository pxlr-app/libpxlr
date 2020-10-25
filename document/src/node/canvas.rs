use crate as document;
use crate::prelude::*;
use nom::number::complete::{le_f32, le_u8};

#[derive(SpriteNode, Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanvasNode {
	pub id: Uuid,
	pub size: Arc<Extent2<u32>>,
	pub palette: Option<Weak<NodeType>>,
	pub display: bool,
	pub locked: bool,
	pub name: Arc<String>,
	pub opacity: f32,
	pub channels: Channel,
	pub canvas: Arc<Canvas>,
}

impl Named for CanvasNode {
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

impl Sized for CanvasNode {
	fn size(&self) -> Extent2<u32> {
		*self.size
	}
	fn resize(&self, target: Extent2<u32>, interpolation: Interpolation) -> Option<CommandPair> {
		Some((
			CommandType::Resize(ResizeCommand {
				target: self.id,
				size: target,
				interpolation,
			}),
			CommandType::RestoreCanvas(RestoreCanvasCommand {
				target: self.id,
				channels: self.channels,
				canvas: (*self.canvas).clone(),
			}),
		))
	}
}

impl Cropable for CanvasNode {
	fn crop(&self, region: Rect<i32, u32>) -> Option<CommandPair> {
		Some((
			CommandType::Crop(CropCommand {
				target: self.id,
				region,
			}),
			CommandType::RestoreCanvas(RestoreCanvasCommand {
				target: self.id,
				channels: self.channels,
				canvas: (*self.canvas).clone(),
			}),
		))
	}
}

impl Flippable for CanvasNode {
	fn flip(&self, axis: FlipAxis) -> Option<CommandPair> {
		Some((
			CommandType::Flip(FlipCommand {
				target: self.id,
				axis,
			}),
			CommandType::RestoreCanvas(RestoreCanvasCommand {
				target: self.id,
				channels: self.channels,
				canvas: (*self.canvas).clone(),
			}),
		))
	}
}

impl Displayed for CanvasNode {
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

impl Locked for CanvasNode {
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

impl Folded for CanvasNode {}

impl Transparent for CanvasNode {
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

impl HasChannels for CanvasNode {
	fn channels(&self) -> Channel {
		self.channels
	}
}

impl CanvasNode {
	pub fn set_channels(&self, channels: Channel) -> Option<CommandPair> {
		if self.channels == channels {
			None
		} else {
			Some((
				CommandType::SetComponents(SetComponentsCommand {
					target: self.id,
					channels,
				}),
				CommandType::SetComponents(SetComponentsCommand {
					target: self.id,
					channels: self.channels,
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
	pub fn apply_stencil(&self, offset: Vec2<u32>, stencil: Stencil) -> Option<CommandPair> {
		Some((
			CommandType::ApplyStencil(ApplyStencilCommand {
				target: self.id,
				offset,
				stencil,
			}),
			CommandType::RestoreCanvas(RestoreCanvasCommand {
				target: self.id,
				channels: self.channels,
				canvas: (*self.canvas).clone(),
			}),
		))
	}
}

impl Executable for CanvasNode {
	fn execute(&self, command: &CommandType) -> Option<NodeType> {
		let mut patched = self.clone();
		if command.as_command().target() == self.id {
			match command {
				CommandType::Rename(command) => {
					patched.name = Arc::new(command.name.clone());
				}
				CommandType::SetVisible(command) => {
					patched.display = command.visibility;
				}
				CommandType::SetLock(command) => {
					patched.locked = command.locked;
				}
				CommandType::SetPaletteNode(command) => {
					match &command.palette {
						Some(node) => patched.palette = Some(Arc::downgrade(node)),
						None => patched.palette = None,
					};
				}
				CommandType::SetOpacity(command) => {
					patched.opacity = command.opacity;
				}
				CommandType::SetComponents(command) => {
					patched.channels = command.channels;
				}
				CommandType::RestoreCanvas(command) => {
					patched.channels = command.channels;
					patched.canvas = Arc::new(command.canvas.clone());
				}
				CommandType::Resize(patch) => {
					patched.canvas =
						Arc::new(patched.canvas.resize(patch.size, patch.interpolation));
				}
				CommandType::Crop(patch) => {
					patched.canvas = Arc::new(patched.canvas.crop(patch.region));
				}
				CommandType::Flip(patch) => {
					patched.canvas = Arc::new(patched.canvas.flip(patch.axis));
				}
				CommandType::ApplyStencil(_patch) => unimplemented!(),
				_ => return None,
			};
			Some(NodeType::Canvas(patched))
		} else {
			None
		}
	}
}

impl parser::v0::ParseNode for CanvasNode {
	fn parse_node<'bytes>(
		row: &parser::v0::IndexRow,
		_children: NodeList,
		dependencies: NodeList,
		bytes: &'bytes [u8],
	) -> parser::Result<&'bytes [u8], NodeRef> {
		use parser::Parse;
		let (bytes, opacity) = le_f32(bytes)?;
		let (bytes, channels) = le_u8(bytes)?;
		let channels = Channel::from_bits(channels).unwrap();
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
		let (bytes, canvas) = Canvas::parse(bytes)?;
		Ok((
			bytes,
			Arc::new(NodeType::Canvas(CanvasNode {
				id: row.id,
				size: Arc::new(row.size),
				palette: palette,
				display: row.display,
				locked: row.locked,
				name: Arc::new(row.name.clone()),
				opacity: opacity,
				channels: channels,
				canvas: Arc::new(canvas),
			})),
		))
	}
}

impl parser::v0::WriteNode for CanvasNode {
	fn write_node<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
		rows: &mut Vec<parser::v0::IndexRow>,
		dependencies: &mut Vec<NodeRef>,
	) -> io::Result<usize> {
		use parser::Write;
		let chunk_offset = writer.seek(io::SeekFrom::Current(0))?;
		let mut size = 5usize;
		writer.write(&self.opacity.to_le_bytes())?;
		writer.write(&self.channels.bits().to_le_bytes())?;
		if let Some(Some(palette)) = self.palette.clone().map(|weak| weak.upgrade()) {
			size += writer.write(&1u8.to_le_bytes())?;
			size += palette.as_node().id().write(writer)?;
			dependencies.push(palette.clone());
		} else {
			size += writer.write(&0u8.to_le_bytes())?;
		}
		size += self.canvas.write(writer)?;

		let mut row = parser::v0::IndexRow::new(self.id);
		row.chunk_type = NodeKind::Canvas;
		row.chunk_offset = chunk_offset;
		row.chunk_size = size as u32;
		row.display = self.display;
		row.locked = self.locked;
		row.name = (*self.name).clone();
		rows.push(row);
		Ok(size)
	}
}
