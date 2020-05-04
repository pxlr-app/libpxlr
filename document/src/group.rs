use crate::parser;
use crate::patch::*;
use crate::INode;
use crate::{DocumentNode, IDocument};
use math::{Extent2, Vec2};
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::io;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
	pub id: Uuid,
	pub is_visible: bool,
	pub is_locked: bool,
	pub is_folded: bool,
	pub name: Rc<String>,
	pub children: Rc<Vec<Rc<DocumentNode>>>,
	pub position: Rc<Vec2<f32>>,
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
		children: Vec<Rc<DocumentNode>>,
	) -> Group {
		Group {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			is_visible: true,
			is_locked: false,
			is_folded: false,
			name: Rc::new(name.to_owned()),
			position: Rc::new(position),
			children: Rc::new(children),
		}
	}

	pub fn add_child(&self, add_child: Rc<DocumentNode>) -> Result<(Patch, Patch), GroupError> {
		let index = self
			.children
			.iter()
			.position(|child| Rc::ptr_eq(&child, &add_child));
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
					name: Rc::new(patch.name.clone()),
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
						children: Rc::new(children),
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
						children: Rc::new(children),
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
				return Some(Box::new(Group {
					id: self.id,
					is_visible: self.is_visible,
					is_locked: self.is_locked,
					is_folded: self.is_folded,
					name: Rc::clone(&self.name),
					children: Rc::new(children),
					position: Rc::clone(&self.position),
				}));
			}
		}
		return None;
	}
}

impl parser::v0::IParser for Group {
	type Output = Group;

	fn parse<'b, S>(
		index: &parser::v0::PartitionIndex,
		row: &parser::v0::PartitionTableRow,
		storage: &mut S,
		bytes: &'b [u8],
	) -> IResult<&'b [u8], Self::Output>
	where
		S: io::Read + io::Seek,
	{
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
					<DocumentNode as parser::v0::IParser>::parse(index, row, storage, &bytes[..])
						.expect("Could not parse node.");
				Rc::new(node)
			})
			.collect::<Vec<_>>();
		Ok((
			bytes,
			Group {
				id: row.id,
				is_visible: row.is_visible,
				is_locked: row.is_locked,
				is_folded: row.is_folded,
				name: Rc::new(String::from(&row.name)),
				position: Rc::new(row.position),
				children: Rc::new(children),
			},
		))
	}

	fn write<S>(&self, index: &mut parser::v0::PartitionIndex, storage: &mut S) -> io::Result<usize>
	where
		S: io::Write + io::Seek,
	{
		let offset = storage.seek(io::SeekFrom::Current(0))?;
		let mut size: usize = 0;
		for child in self.children.iter() {
			size += child.write(index, storage)?;
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
