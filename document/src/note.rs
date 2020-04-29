use crate::document::Document;
use crate::parser;
use crate::patch::*;
use math::Vec2;
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

impl parser::v0::PartitionTableParse for Note {
	type Output = Note;

	fn parse<'a, 'b>(
		_file: &mut parser::v0::Database<'a>,
		row: &parser::v0::PartitionTableRow,
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
}
