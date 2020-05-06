use crate::color::ColorMode;
use crate::document::IDocument;
use crate::parser;
use crate::patch::*;
use crate::sprite::*;
use crate::INode;
use math::interpolation::*;
use math::{Extent2, Vec2};
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::io;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct LayerGroup {
	pub id: Uuid,
	pub is_visible: bool,
	pub is_locked: bool,
	pub is_folded: bool,
	pub name: Rc<String>,
	pub color_mode: ColorMode,
	pub children: Rc<Vec<Rc<LayerNode>>>,
	pub position: Rc<Vec2<f32>>,
	pub size: Rc<Extent2<u32>>,
}

#[derive(Debug)]
pub enum LayerGroupError {
	LayerFound,
	LayerNotFound,
}

impl std::fmt::Display for LayerGroupError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			LayerGroupError::LayerFound => write!(f, "Child already exists in this group."),
			LayerGroupError::LayerNotFound => write!(f, "Child not found in this group."),
		}
	}
}

impl std::error::Error for LayerGroupError {
	fn cause(&self) -> Option<&dyn std::error::Error> {
		None
	}
}

impl LayerGroup {
	pub fn new(
		id: Option<Uuid>,
		name: &str,
		color_mode: ColorMode,
		children: Vec<Rc<LayerNode>>,
		position: Vec2<f32>,
		size: Extent2<u32>,
	) -> LayerGroup {
		LayerGroup {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			is_visible: true,
			is_locked: false,
			is_folded: false,
			name: Rc::new(name.to_owned()),
			color_mode: color_mode,
			children: Rc::new(children),
			position: Rc::new(position),
			size: Rc::new(size),
		}
	}

	pub fn add_layer(&self, add_layer: Rc<LayerNode>) -> Result<(Patch, Patch), LayerGroupError> {
		let index = self
			.children
			.iter()
			.position(|child| Rc::ptr_eq(&child, &add_layer));
		if index.is_some() {
			Err(LayerGroupError::LayerFound)
		} else {
			Ok((
				Patch::AddLayer(AddLayerPatch {
					target: self.id,
					child: add_layer.clone(),
					position: self.children.len(),
				}),
				Patch::RemoveLayer(RemoveLayerPatch {
					target: self.id,
					child_id: add_layer.id(),
				}),
			))
		}
	}

	pub fn remove_layer(&self, child_id: Uuid) -> Result<(Patch, Patch), LayerGroupError> {
		let index = self
			.children
			.iter()
			.position(|child| child.id() == child_id);
		if index.is_none() {
			Err(LayerGroupError::LayerNotFound)
		} else {
			let index = index.unwrap();
			Ok((
				Patch::RemoveLayer(RemoveLayerPatch {
					target: self.id,
					child_id: child_id,
				}),
				Patch::AddLayer(AddLayerPatch {
					target: self.id,
					child: self.children.get(index).unwrap().clone(),
					position: index,
				}),
			))
		}
	}

	pub fn move_layer(
		&self,
		child_id: Uuid,
		position: usize,
	) -> Result<(Patch, Patch), LayerGroupError> {
		let index = self
			.children
			.iter()
			.position(|child| child.id() == child_id);
		if index.is_none() {
			Err(LayerGroupError::LayerNotFound)
		} else {
			let index = index.unwrap();
			Ok((
				Patch::MoveLayer(MoveLayerPatch {
					target: self.id,
					child_id: child_id,
					position: position,
				}),
				Patch::MoveLayer(MoveLayerPatch {
					target: self.id,
					child_id: child_id,
					position: index,
				}),
			))
		}
	}
}

impl ILayer for LayerGroup {
	fn crop(
		&self,
		offset: Vec2<u32>,
		size: Extent2<u32>,
	) -> Result<(Patch, Patch), CropLayerError> {
		if size.w == 0 || size.h == 0 {
			Err(CropLayerError::InvalidSize)
		} else if size.w + offset.x > self.size.w || size.h + offset.y > self.size.h {
			Err(CropLayerError::OutsideRegion)
		} else {
			Ok((
				Patch::CropLayer(CropLayerPatch {
					target: self.id,
					offset: offset,
					size: size,
				}),
				Patch::RestoreLayerGroup(RestoreLayerGroupPatch {
					target: self.id,
					name: (*self.name).to_owned(),
					position: (*self.position).clone(),
					size: (*self.size).clone(),
					children: self
						.children
						.iter()
						.map(|child| child.crop(offset, size).unwrap().1)
						.collect::<Vec<_>>(),
				}),
			))
		}
	}

	fn resize(
		&self,
		size: Extent2<u32>,
		interpolation: Interpolation,
	) -> Result<(Patch, Patch), ResizeLayerError> {
		if size.w == 0 || size.h == 0 {
			Err(ResizeLayerError::InvalidSize)
		} else {
			Ok((
				Patch::ResizeLayer(ResizeLayerPatch {
					target: self.id,
					size: size,
					interpolation: interpolation,
				}),
				Patch::RestoreLayerGroup(RestoreLayerGroupPatch {
					target: self.id,
					name: (*self.name).to_owned(),
					position: (*self.position).clone(),
					size: (*self.size).clone(),
					children: self
						.children
						.iter()
						.map(|child| child.resize(size, interpolation).unwrap().1)
						.collect::<Vec<_>>(),
				}),
			))
		}
	}
}

impl IDocument for LayerGroup {
	fn position(&self) -> Vec2<f32> {
		*(self.position).clone()
	}
}

impl INode for LayerGroup {
	fn is_visible(&self) -> bool {
		self.is_visible
	}
	fn is_locked(&self) -> bool {
		self.is_locked
	}
	fn is_folded(&self) -> bool {
		self.is_folded
	}
}

impl<'a> Renamable<'a> for LayerGroup {
	fn rename(&self, new_name: &'a str) -> Result<(Patch, Patch), RenameError> {
		if *self.name == new_name {
			Err(RenameError::Unchanged)
		} else {
			Ok((
				Patch::Rename(RenamePatch {
					target: self.id,
					name: new_name.to_owned(),
				}),
				Patch::Rename(RenamePatch {
					target: self.id,
					name: (*self.name).to_owned(),
				}),
			))
		}
	}
}

impl IVisible for LayerGroup {
	fn set_visibility(&self, visible: bool) -> Result<(Patch, Patch), SetVisibilityError> {
		if self.is_visible == visible {
			Err(SetVisibilityError::Unchanged)
		} else {
			Ok((
				Patch::SetVisibility(SetVisibilityPatch {
					target: self.id,
					visibility: visible,
				}),
				Patch::SetVisibility(SetVisibilityPatch {
					target: self.id,
					visibility: self.is_visible,
				}),
			))
		}
	}
}

impl ILockable for LayerGroup {
	fn set_lock(&self, lock: bool) -> Result<(Patch, Patch), SetLockError> {
		if self.is_locked == lock {
			Err(SetLockError::Unchanged)
		} else {
			Ok((
				Patch::SetLock(SetLockPatch {
					target: self.id,
					lock: lock,
				}),
				Patch::SetLock(SetLockPatch {
					target: self.id,
					lock: self.is_locked,
				}),
			))
		}
	}
}

impl IFoldable for LayerGroup {
	fn set_fold(&self, folded: bool) -> Result<(Patch, Patch), SetFoldError> {
		if self.is_folded == folded {
			Err(SetFoldError::Unchanged)
		} else {
			Ok((
				Patch::SetFold(SetFoldPatch {
					target: self.id,
					folded: folded,
				}),
				Patch::SetFold(SetFoldPatch {
					target: self.id,
					folded: self.is_folded,
				}),
			))
		}
	}
}

impl IPatchable for LayerGroup {
	fn patch(&self, patch: &Patch) -> Option<Box<Self>> {
		if patch.target() == self.id {
			return match patch {
				Patch::Rename(patch) => Some(Box::new(LayerGroup {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: self.is_locked,
					is_folded: self.is_folded,
					name: Rc::new(patch.name.clone()),
					color_mode: self.color_mode,
					position: self.position.clone(),
					size: self.size.clone(),
					children: self.children.clone(),
				})),
				Patch::SetVisibility(patch) => Some(Box::new(LayerGroup {
					id: self.id,
					is_visible: patch.visibility,
					is_locked: self.is_locked,
					is_folded: self.is_folded,
					name: self.name.clone(),
					color_mode: self.color_mode,
					position: self.position.clone(),
					size: self.size.clone(),
					children: self.children.clone(),
				})),
				Patch::SetLock(patch) => Some(Box::new(LayerGroup {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: patch.lock,
					is_folded: self.is_folded,
					name: self.name.clone(),
					color_mode: self.color_mode,
					position: self.position.clone(),
					size: self.size.clone(),
					children: self.children.clone(),
				})),
				Patch::SetFold(patch) => Some(Box::new(LayerGroup {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: self.is_locked,
					is_folded: patch.folded,
					name: self.name.clone(),
					color_mode: self.color_mode,
					position: self.position.clone(),
					size: self.size.clone(),
					children: self.children.clone(),
				})),
				Patch::AddLayer(patch) => {
					assert_eq!(patch.child.color_mode(), self.color_mode);
					let mut children = self
						.children
						.iter()
						.map(|child| child.clone())
						.collect::<Vec<_>>();
					if patch.position > children.len() {
						children.push(patch.child.clone());
					} else {
						children.insert(patch.position, patch.child.clone());
					}
					Some(Box::new(LayerGroup {
						id: self.id,
						is_visible: self.is_visible,
						is_locked: self.is_locked,
						is_folded: self.is_folded,
						name: self.name.clone(),
						color_mode: self.color_mode,
						position: self.position.clone(),
						size: self.size.clone(),
						children: Rc::new(children),
					}))
				}
				Patch::RemoveLayer(patch) => {
					let children = self
						.children
						.iter()
						.filter_map(|child| {
							if child.id() == patch.child_id {
								None
							} else {
								Some(child.clone())
							}
						})
						.collect::<Vec<_>>();
					Some(Box::new(LayerGroup {
						id: self.id,
						is_visible: self.is_visible,
						is_locked: self.is_locked,
						is_folded: self.is_folded,
						name: self.name.clone(),
						color_mode: self.color_mode,
						position: self.position.clone(),
						size: self.size.clone(),
						children: Rc::new(children),
					}))
				}
				Patch::MoveLayer(patch) => {
					let mut children = self
						.children
						.iter()
						.map(|child| child.clone())
						.collect::<Vec<_>>();
					let index = children
						.iter()
						.position(|child| child.id() == patch.child_id)
						.unwrap();
					let child = children.remove(index);
					if patch.position > children.len() {
						children.push(child);
					} else {
						children.insert(patch.position, child);
					}
					Some(Box::new(LayerGroup {
						id: self.id,
						is_visible: self.is_visible,
						is_locked: self.is_locked,
						is_folded: self.is_folded,
						name: self.name.clone(),
						color_mode: self.color_mode,
						position: self.position.clone(),
						size: self.size.clone(),
						children: Rc::new(children),
					}))
				}
				Patch::CropLayer(patch) => {
					let children = self
						.children
						.iter()
						.map(|child| {
							match child.patch(&Patch::CropLayer(CropLayerPatch {
								target: child.id(),
								..*patch
							})) {
								Some(new_child) => Rc::new(new_child),
								None => child.clone(),
							}
						})
						.collect::<Vec<_>>();
					Some(Box::new(LayerGroup {
						id: self.id,
						is_visible: self.is_visible,
						is_locked: self.is_locked,
						is_folded: self.is_folded,
						name: Rc::clone(&self.name),
						color_mode: self.color_mode,
						position: Rc::clone(&self.position),
						size: Rc::new(patch.size),
						children: Rc::new(children),
					}))
				}
				Patch::ResizeLayer(patch) => {
					let children = self
						.children
						.iter()
						.map(|child| {
							match child.patch(&Patch::ResizeLayer(ResizeLayerPatch {
								target: child.id(),
								..*patch
							})) {
								Some(new_child) => Rc::new(new_child),
								None => child.clone(),
							}
						})
						.collect::<Vec<_>>();
					Some(Box::new(LayerGroup {
						id: self.id,
						is_visible: self.is_visible,
						is_locked: self.is_locked,
						is_folded: self.is_folded,
						name: Rc::clone(&self.name),
						color_mode: self.color_mode,
						position: Rc::clone(&self.position),
						size: Rc::new(patch.size),
						children: Rc::new(children),
					}))
				}
				_ => None,
			};
		} else {
			let mut mutated = false;
			let children = self
				.children
				.iter()
				.map(|child| match child.patch(patch) {
					Some(new_child) => {
						mutated = true;
						Rc::new(new_child)
					}
					None => child.clone(),
				})
				.collect::<Vec<_>>();
			if mutated {
				return Some(Box::new(LayerGroup {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: self.is_locked,
					is_folded: self.is_folded,
					name: Rc::clone(&self.name),
					color_mode: self.color_mode,
					position: Rc::clone(&self.position),
					size: Rc::clone(&self.size),
					children: Rc::new(children),
				}));
			}
		}
		return None;
	}
}

impl parser::v0::IParser for LayerGroup {
	type Output = LayerGroup;

	fn parse<'b, S>(
		index: &parser::v0::PartitionIndex,
		row: &parser::v0::PartitionTableRow,
		storage: &mut S,
		bytes: &'b [u8],
	) -> IResult<&'b [u8], Self::Output>
	where
		S: io::Read + io::Seek,
	{
		let (bytes, color_mode) = <ColorMode as parser::IParser>::parse(bytes)?;
		let children = row
			.children
			.iter()
			.map(|i| {
				let row = index
					.rows
					.get(*i as usize)
					.expect("Could not retrieve children in index.");
				let size = row.chunk_size;
				let offset = row.chunk_offset;
				let mut bytes: Vec<u8> = Vec::with_capacity(size as usize);
				storage
					.seek(io::SeekFrom::Start(offset))
					.expect("Could not seek to chunk.");
				storage
					.read(&mut bytes)
					.expect("Could not read chunk data.");
				let (_, node) =
					<LayerNode as parser::v0::IParser>::parse(index, row, storage, &bytes[..])
						.expect("Could not parse node.");
				Rc::new(node)
			})
			.collect::<Vec<_>>();
		Ok((
			bytes,
			LayerGroup {
				id: row.id,
				is_visible: row.is_visible,
				is_locked: row.is_locked,
				is_folded: row.is_folded,
				name: Rc::new(String::from(&row.name)),
				color_mode: color_mode,
				children: Rc::new(children),
				position: Rc::new(row.position),
				size: Rc::new(row.size),
			},
		))
	}

	fn write<S>(
		&self,
		index: &mut parser::v0::PartitionIndex,
		storage: &mut S,
		offset: u64,
	) -> io::Result<usize>
	where
		S: io::Write,
	{
		let mut size: usize = 0;
		for child in self.children.iter() {
			size += child.write(index, storage, offset + (size as u64))?;
		}
		let children = self
			.children
			.iter()
			.map(|c| *index.index_uuid.get(&c.id()).unwrap() as u32)
			.collect::<Vec<_>>();
		if let Some(i) = index.index_uuid.get(&self.id) {
			let mut row = index.rows.get_mut(*i).unwrap();
			row.chunk_offset = offset as u64;
			row.chunk_size = 0;
			row.is_visible = self.is_visible;
			row.is_locked = self.is_locked;
			row.is_folded = self.is_folded;
			row.position = *self.position;
			row.name = String::from(&*self.name);
			row.children = children;
		} else {
			let row = parser::v0::PartitionTableRow {
				id: self.id,
				chunk_type: parser::v0::ChunkType::Group,
				chunk_offset: offset as u64,
				chunk_size: 0,
				is_root: false,
				is_visible: self.is_visible,
				is_locked: self.is_locked,
				is_folded: self.is_folded,
				position: *self.position,
				size: Extent2::new(0, 0),
				name: String::from(&*self.name),
				children: children,
				preview: Vec::new(),
			};
			index.index_uuid.insert(row.id, index.rows.len());
			index.rows.push(row);
		}
		Ok(size)
	}
}
