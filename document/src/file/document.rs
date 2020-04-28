use crate::file;
use crate::file::part;
use math::{Extent2, Vec2};
use std::io::SeekFrom;

#[derive(Debug, PartialEq)]
pub struct Group {
	pub position: Vec2<f32>,
}

impl crate::file::reader::v0::Reader for Group {
	fn from_bytes(bytes: &[u8]) -> nom::IResult<&[u8], Group> {
		let (bytes, position) = Vec2::<f32>::from_bytes(bytes)?;
		Ok((bytes, Group { position }))
	}
}

impl crate::file::writer::Writer for crate::Group {
	fn write<W: std::io::Write + std::io::Seek>(
		&self,
		file: &mut file::File,
		writer: &mut W,
	) -> std::io::Result<usize> {
		let offset = writer.seek(SeekFrom::Current(0))?;
		let mut size: usize = 0;
		for child in self.children.iter() {
			size += child.write(file, writer)?;
			println!("Group::write size={}", size);
		}
		if let Some(i) = file.chunks.get(&self.id) {
			let mut row = file.rows.get_mut(*i).unwrap();
			row.chunk_offset = offset;
			row.chunk_size = 0;
		} else {
			let row = part::PartitionTableRow {
				id: self.id,
				chunk_type: part::ChunkType::Group,
				chunk_offset: offset,
				chunk_size: 0,
				position: *self.position,
				size: Extent2::new(0, 0),
				name: String::from(&*self.name),
				children: self
					.children
					.iter()
					.map(|c| *file.chunks.get(&c.id()).unwrap() as u32)
					.collect::<Vec<_>>(),
				preview: Vec::new(),
			};
			file.chunks.insert(row.id, file.rows.len());
			file.rows.push(row);
		}
		Ok(size)
	}
}

#[derive(Debug, PartialEq)]
pub struct Note {
	pub position: Vec2<f32>,
}

impl crate::file::reader::v0::Reader for Note {
	fn from_bytes(bytes: &[u8]) -> nom::IResult<&[u8], Note> {
		let (bytes, position) = Vec2::<f32>::from_bytes(bytes)?;
		Ok((bytes, Note { position }))
	}
}

impl crate::file::writer::Writer for crate::Note {
	fn write<W: std::io::Write + std::io::Seek>(
		&self,
		file: &mut file::File,
		writer: &mut W,
	) -> std::io::Result<usize> {
		let offset = writer.seek(SeekFrom::Current(0))?;
		println!("Note::write offset={}", offset);
		if let Some(i) = file.chunks.get(&self.id) {
			let mut row = file.rows.get_mut(*i).unwrap();
			row.chunk_offset = offset;
			row.chunk_size = 0;
		} else {
			let row = part::PartitionTableRow {
				id: self.id,
				chunk_type: part::ChunkType::Note,
				chunk_offset: offset,
				chunk_size: 0,
				position: *self.position,
				size: Extent2::new(0, 0),
				name: String::from(&*self.note),
				children: Vec::new(),
				preview: Vec::new(),
			};
			file.chunks.insert(row.id, file.rows.len());
			file.rows.push(row);
		}
		Ok(0)
	}
}
