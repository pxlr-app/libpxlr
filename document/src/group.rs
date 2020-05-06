use crate::parser;
use crate::patch::*;
use crate::INode;
use crate::{DocumentNode, IDocument};
use async_std::io;
use async_trait::async_trait;
use math::{Extent2, Vec2};
use nom::IResult;
use parallel_stream::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
	pub id: Uuid,
	pub is_visible: bool,
	pub is_locked: bool,
	pub is_folded: bool,
	pub name: Arc<String>,
	pub children: Arc<Vec<Arc<DocumentNode>>>,
	pub position: Arc<Vec2<f32>>,
}

#[derive(Debug)]
pub enum GroupError {
	ChildFound,
	ChildNotFound,
}

impl std::fmt::Display for GroupError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			GroupError::ChildFound => write!(f, "Child already exists in this group."),
			GroupError::ChildNotFound => write!(f, "Child not found in this group."),
		}
	}
}

impl std::error::Error for GroupError {
	fn cause(&self) -> Option<&dyn std::error::Error> {
		None
	}
}

impl Group {
	pub fn new(
		id: Option<Uuid>,
		name: &str,
		position: Vec2<f32>,
		children: Vec<Arc<DocumentNode>>,
	) -> Group {
		Group {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			is_visible: true,
			is_locked: false,
			is_folded: false,
			name: Arc::new(name.to_owned()),
			position: Arc::new(position),
			children: Arc::new(children),
		}
	}

	pub fn add_child(&self, add_child: Arc<DocumentNode>) -> Result<(Patch, Patch), GroupError> {
		let index = self
			.children
			.iter()
			.position(|child| Arc::ptr_eq(&child, &add_child));
		if index.is_some() {
			Err(GroupError::ChildFound)
		} else {
			Ok((
				Patch::AddChild(AddChildPatch {
					target: self.id,
					child: add_child.clone(),
					position: self.children.len(),
				}),
				Patch::RemoveChild(RemoveChildPatch {
					target: self.id,
					child_id: add_child.id(),
				}),
			))
		}
	}

	pub fn remove_child(&self, child_id: Uuid) -> Result<(Patch, Patch), GroupError> {
		let index = self
			.children
			.iter()
			.position(|child| child.id() == child_id);
		if index.is_none() {
			Err(GroupError::ChildNotFound)
		} else {
			let index = index.unwrap();
			Ok((
				Patch::RemoveChild(RemoveChildPatch {
					target: self.id,
					child_id: child_id,
				}),
				Patch::AddChild(AddChildPatch {
					target: self.id,
					child: self.children.get(index).unwrap().clone(),
					position: index,
				}),
			))
		}
	}

	pub fn move_child(
		&self,
		child_id: Uuid,
		position: usize,
	) -> Result<(Patch, Patch), GroupError> {
		let index = self
			.children
			.iter()
			.position(|child| child.id() == child_id);
		if index.is_none() {
			Err(GroupError::ChildNotFound)
		} else {
			let index = index.unwrap();
			Ok((
				Patch::MoveChild(MoveChildPatch {
					target: self.id,
					child_id: child_id,
					position: position,
				}),
				Patch::MoveChild(MoveChildPatch {
					target: self.id,
					child_id: child_id,
					position: index,
				}),
			))
		}
	}
}

impl IDocument for Group {
	fn position(&self) -> Vec2<f32> {
		*(self.position).clone()
	}
}

impl INode for Group {
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

impl<'a> Renamable<'a> for Group {
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

impl IVisible for Group {
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

impl ILockable for Group {
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

impl IFoldable for Group {
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

impl IPatchable for Group {
	fn patch(&self, patch: &Patch) -> Option<Box<Self>> {
		if patch.target() == self.id {
			return match patch {
				Patch::Rename(patch) => Some(Box::new(Group {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: self.is_locked,
					is_folded: self.is_folded,
					name: Arc::new(patch.name.clone()),
					position: self.position.clone(),
					children: self.children.clone(),
				})),
				Patch::SetVisibility(patch) => Some(Box::new(Group {
					id: self.id,
					is_visible: patch.visibility,
					is_locked: self.is_locked,
					is_folded: self.is_folded,
					name: self.name.clone(),
					position: self.position.clone(),
					children: self.children.clone(),
				})),
				Patch::SetLock(patch) => Some(Box::new(Group {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: patch.lock,
					is_folded: self.is_folded,
					name: self.name.clone(),
					position: self.position.clone(),
					children: self.children.clone(),
				})),
				Patch::SetFold(patch) => Some(Box::new(Group {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: self.is_locked,
					is_folded: patch.folded,
					name: self.name.clone(),
					position: self.position.clone(),
					children: self.children.clone(),
				})),
				Patch::AddChild(patch) => {
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
					Some(Box::new(Group {
						id: self.id,
						is_visible: self.is_visible,
						is_locked: self.is_locked,
						is_folded: self.is_folded,
						name: self.name.clone(),
						position: self.position.clone(),
						children: Arc::new(children),
					}))
				}
				Patch::RemoveChild(patch) => {
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
					Some(Box::new(Group {
						id: self.id,
						is_visible: self.is_visible,
						is_locked: self.is_locked,
						is_folded: self.is_folded,
						name: self.name.clone(),
						position: self.position.clone(),
						children: Arc::new(children),
					}))
				}
				Patch::MoveChild(patch) => {
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
					Some(Box::new(Group {
						id: self.id,
						is_visible: self.is_visible,
						is_locked: self.is_locked,
						is_folded: self.is_folded,
						name: self.name.clone(),
						position: self.position.clone(),
						children: Arc::new(children),
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
						Arc::new(new_child)
					}
					None => child.clone(),
				})
				.collect::<Vec<_>>();
			if mutated {
				return Some(Box::new(Group {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: self.is_locked,
					is_folded: self.is_folded,
					name: Arc::clone(&self.name),
					children: Arc::new(children),
					position: Arc::clone(&self.position),
				}));
			}
		}
		return None;
	}
}

#[async_trait]
impl parser::v0::IParser for Group {
	type Output = Group;

	async fn parse<'b, S>(
		index: &parser::v0::PartitionIndex,
		row: &parser::v0::PartitionTableRow,
		storage: &mut S,
		bytes: &'b [u8],
	) -> IResult<&'b [u8], Self::Output>
	where
		S: parser::ReadAt + std::marker::Send + std::marker::Unpin,
	{
		let mut children: Vec<Arc<DocumentNode>> = Vec::new();
		for i in row.children.iter() {
			let row = index
				.rows
				.get(*i as usize)
				.expect("Count not retrieve children.");
			let chunk_size = row.chunk_size;
			let chunk_offset = row.chunk_offset;
			let mut bytes: Vec<u8> = Vec::with_capacity(chunk_size as usize);
			storage
				.read_at(io::SeekFrom::Start(chunk_offset), &mut bytes)
				.await
				.expect("Could not read chunk data.");
			let (_, node) =
				<DocumentNode as parser::v0::IParser>::parse(index, row, storage, &bytes[..])
					.await
					.expect("Could not parse node.");
			children.push(Arc::new(node));
		}
		Ok((
			bytes,
			Group {
				id: row.id,
				is_visible: row.is_visible,
				is_locked: row.is_locked,
				is_folded: row.is_folded,
				name: Arc::new(String::from(&row.name)),
				position: Arc::new(row.position),
				children: Arc::new(children),
			},
		))
	}

	async fn write<S>(
		&self,
		index: &mut parser::v0::PartitionIndex,
		storage: &mut S,
		offset: u64,
	) -> io::Result<usize>
	where
		S: io::Write + std::marker::Send + std::marker::Unpin,
	{
		let mut size: usize = 0;
		for child in self.children.iter() {
			size += child.write(index, storage, offset + (size as u64)).await?;
		}
		let children = self
			.children
			.iter()
			.map(|c| *index.index_uuid.get(&c.id()).unwrap() as u32)
			.collect::<Vec<_>>();
		if let Some(i) = index.index_uuid.get(&self.id) {
			let mut row = index.rows.get_mut(*i).unwrap();
			row.chunk_offset = offset;
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
				chunk_offset: offset,
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
