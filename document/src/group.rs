use crate::document::*;
use crate::parser;
use crate::parser::v0::PartitionTableParse;
use crate::patch::*;
use crate::Node;
use math::{Extent2, Vec2};
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::io;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
	pub id: Uuid,
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

impl Document for Group {
	fn position(&self) -> Vec2<f32> {
		*(self.position).clone()
	}
}

impl<'a> Renamable<'a> for Group {
	fn rename(&self, new_name: &'a str) -> Result<(Patch, Patch), RenameError> {
		if *self.name == new_name {
			Err(RenameError::SameName)
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

impl Patchable for Group {
	fn patch(&self, patch: &Patch) -> Option<Box<Self>> {
		if patch.target() == self.id {
			return match patch {
				Patch::Rename(patch) => Some(Box::new(Group {
					id: self.id,
					name: Rc::new(patch.name.clone()),
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
					name: Rc::clone(&self.name),
					children: Rc::new(children),
					position: Rc::clone(&self.position),
				}));
			}
		}
		return None;
	}
}

impl parser::v0::PartitionTableParse for Group {
	type Output = Group;

	fn parse<'a, 'b>(
		file: &mut parser::v0::Database<'a>,
		row: &parser::v0::PartitionTableRow,
		bytes: &'b [u8],
	) -> IResult<&'b [u8], Self::Output> {
		let children = row
			.children
			.iter()
			.map(|i| {
				let bytes = file
					.read_chunk(*i as usize)
					.expect("Could not retrieve chunk.");
				let (_, node) =
					<DocumentNode as parser::v0::PartitionTableParse>::parse(file, row, &bytes[..])
						.expect("Could not parse node.");
				Rc::new(node)
			})
			.collect::<Vec<_>>();
		Ok((
			bytes,
			Group {
				id: row.id,
				name: Rc::new(String::from(&row.name)),
				position: Rc::new(row.position),
				children: Rc::new(children),
			},
		))
	}

	fn write<'a, W: io::Write + io::Seek>(
		&self,
		file: &mut parser::v0::Database<'a>,
		writer: &mut W,
	) -> io::Result<usize> {
		let offset = writer.seek(io::SeekFrom::Current(0))?;
		let mut size: usize = 0;
		for child in self.children.iter() {
			size += child.write(file, writer)?;
		}
		if let Some(i) = file.lut_rows.get(&self.id) {
			let mut row = file.rows.get_mut(*i).unwrap();
			row.chunk_offset = offset;
			row.chunk_size = 0;
		} else {
			let row = parser::v0::PartitionTableRow {
				id: self.id,
				chunk_type: parser::v0::ChunkType::Group,
				chunk_offset: offset,
				chunk_size: 0,
				position: *self.position,
				size: Extent2::new(0, 0),
				name: String::from(&*self.name),
				children: self
					.children
					.iter()
					.map(|c| *file.lut_rows.get(&c.id()).unwrap() as u32)
					.collect::<Vec<_>>(),
				preview: Vec::new(),
			};
			file.lut_rows.insert(row.id, file.rows.len());
			file.rows.push(row);
		}
		Ok(size)
	}
}
