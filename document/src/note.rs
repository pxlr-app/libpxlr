use crate::document::Document;
use crate::parser;
use crate::patch::*;
use async_std::io;
use async_std::io::prelude::*;
use async_trait::async_trait;
use math::{Extent2, Vec2};
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
	pub id: Uuid,
	pub note: Rc<String>,
	pub position: Rc<Vec2<f32>>,
}

impl Note {
	pub fn new(id: Option<Uuid>, note: &str, position: Vec2<f32>) -> Note {
		Note {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			note: Rc::new(note.to_owned()),
			position: Rc::new(position),
		}
	}
}

impl Document for Note {
	fn position(&self) -> Vec2<f32> {
		*(self.position).clone()
	}
}

impl<'a> Renamable<'a> for Note {
	fn rename(&self, new_name: &'a str) -> Result<(Patch, Patch), RenameError> {
		if *self.note == new_name {
			Err(RenameError::SameName)
		} else {
			Ok((
				Patch::Rename(RenamePatch {
					target: self.id,
					name: new_name.to_owned(),
				}),
				Patch::Rename(RenamePatch {
					target: self.id,
					name: (*self.note).to_owned(),
				}),
			))
		}
	}
}

impl Patchable for Note {
	fn patch(&self, patch: &Patch) -> Option<Box<Self>> {
		if patch.target() == self.id {
			return match patch {
				Patch::Rename(rename) => Some(Box::new(Note {
					id: self.id,
					note: Rc::new(rename.name.clone()),
					position: self.position.clone(),
				})),
				_ => None,
			};
		}
		return None;
	}
}

#[async_trait(?Send)]
impl<S> parser::v0::PartitionTableParse<S> for Note
where
	S: io::Read + io::Write + io::Seek + std::marker::Unpin,
{
	type Output = Note;

	async fn parse<'b>(
		_index: &parser::v0::PartitionIndex,
		row: &parser::v0::PartitionTableRow,
		_storage: &mut S,
		bytes: &'b [u8],
	) -> IResult<&'b [u8], Self::Output> {
		Ok((
			bytes,
			Note {
				id: row.id,
				note: Rc::new(String::from(&row.name)),
				position: Rc::new(row.position),
			},
		))
	}

	async fn write(
		&self,
		index: &mut parser::v0::PartitionIndex,
		storage: &mut S,
	) -> io::Result<usize> {
		let offset = storage.seek(io::SeekFrom::Current(0)).await?;
		if let Some(i) = index.index_uuid.get(&self.id) {
			let mut row = index.rows.get_mut(*i).unwrap();
			row.chunk_offset = offset as u64;
			row.chunk_size = 0;
		} else {
			let row = parser::v0::PartitionTableRow {
				id: self.id,
				chunk_type: parser::v0::ChunkType::Note,
				chunk_offset: offset as u64,
				chunk_size: 0,
				position: *self.position,
				size: Extent2::new(0, 0),
				name: String::from(&*self.note),
				children: Vec::new(),
				preview: Vec::new(),
			};
			index.index_uuid.insert(row.id, index.rows.len());
			index.rows.push(row);
		}
		Ok(0)
	}
}
