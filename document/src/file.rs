use crate::parser;
use crate::parser::Parser;
use async_std::io;
use async_std::io::prelude::*;
use nom::multi::many0;

#[derive(Debug, Clone, PartialEq)]
pub enum FileStorageError {
	Unknown,
	VersionNotSupported,
	NodeNotSupported,
	ParseError(nom::Err<((), nom::error::ErrorKind)>),
}

impl std::fmt::Display for FileStorageError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			FileStorageError::Unknown => write!(f, "Unknown error."),
			FileStorageError::VersionNotSupported => write!(f, "Version not supported."),
			FileStorageError::NodeNotSupported => write!(f, "Node not supported."),
			FileStorageError::ParseError(_) => write!(f, "Could not parse the file."),
		}
	}
}

impl From<io::Error> for FileStorageError {
	fn from(error: std::io::Error) -> Self {
		match error.kind() {
			_ => FileStorageError::Unknown,
		}
	}
}

impl From<nom::Err<(&[u8], nom::error::ErrorKind)>> for FileStorageError {
	fn from(error: nom::Err<(&[u8], nom::error::ErrorKind)>) -> Self {
		match error {
			nom::Err::Incomplete(e) => FileStorageError::ParseError(nom::Err::Incomplete(e)),
			nom::Err::Error(e) => FileStorageError::ParseError(nom::Err::Error(((), e.1))),
			nom::Err::Failure(e) => FileStorageError::ParseError(nom::Err::Error(((), e.1))),
		}
	}
}

pub struct File<'s, S>
where
	S: io::Read + io::Write + io::Seek + std::marker::Unpin,
{
	pub storage: &'s S,
	pub header: parser::Header,
	pub index: parser::v0::PartitionIndex,
}

impl<'s, S> File<'s, S>
where
	S: io::Read + io::Write + io::Seek + std::marker::Unpin,
{
	pub async fn from(storage: &'s mut S) -> Result<File<'s, S>, FileStorageError> {
		let mut buffer = [0u8; 5];
		storage.seek(io::SeekFrom::Start(0)).await?;
		storage.read(&mut buffer).await?;
		let (_, header) = parser::Header::parse(&buffer)?;

		let mut buffer = [0u8; 20];
		storage.seek(io::SeekFrom::End(-20)).await?;
		storage.read(&mut buffer).await?;

		let (_, table) = match header.version {
			0 => <parser::v0::PartitionTable as Parser>::parse(&buffer),
			_ => panic!(FileStorageError::VersionNotSupported),
		}?;

		let rows: Vec<parser::v0::PartitionTableRow> = if table.size == 0 {
			vec![]
		} else {
			let mut buffer = vec![0u8; table.size as usize];
			storage
				.seek(io::SeekFrom::Current(-20 - (table.size as i64)))
				.await?;
			storage.read(&mut buffer).await?;

			let (_, rows) = match header.version {
				0 => many0(<parser::v0::PartitionTableRow as Parser>::parse)(&buffer),
				_ => panic!(FileStorageError::VersionNotSupported),
			}?;
			rows
		};

		Ok(File {
			storage,
			header,
			index: parser::v0::PartitionIndex::new(table, rows),
		})
	}

	// pub async fn get_chunk_from_row(
	// 	&mut self,
	// 	row: &parser::v0::PartitionTableRow,
	// ) -> Result<Node, FileStorageError> {
	// 	let mut bytes: Vec<u8> = Vec::with_capacity(row.chunk_size as usize);
	// 	self.storage
	// 		.seek(io::SeekFrom::Start(row.chunk_offset))
	// 		.await?;
	// 	self.storage.read(&mut bytes).await?;
	// 	let (_, node) = <Node as parser::v0::PartitionTableParse>::parse(
	// 		&self.index,
	// 		row,
	// 		storage,
	// 		&bytes[..],
	// 	)
	// 	.await?;
	// 	node
	// }
}

// impl<'a> File<'a> {
// 	pub fn append<W: io::Write + io::Seek>(
// 		&mut self,
// 		node: &Node,
// 		writer: &mut W,
// 	) -> io::Result<usize> {
// 		writer.seek(io::SeekFrom::End(0))?;
// 		let mut size = node.write(&mut self.database, writer)?;
// 		let mut table_size: usize = 0;
// 		for row in self.database.rows.iter() {
// 			table_size += row.write(writer)?;
// 		}
// 		self.database.table.size = table_size as u32;
// 		size += self.database.table.write(writer)?;
// 		Ok(size + table_size)
// 	}
// }

#[cfg(test)]
mod tests {
	use super::*;
	use crate::parser;
	use crate::{DocumentNode, Group, Node, Note};
	use async_std::io;
	use async_std::io::prelude::*;
	use async_std::task;
	use math::Vec2;
	use std::rc::Rc;
	use uuid::Uuid;

	#[test]
	fn it_reads_empty_file() {
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(vec![
			0x50, 0x58, 0x4C, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4B, 0x26, 0xC4, 0x71, 0x30,
			0x98, 0x4C, 0xCE, 0x9C, 0xDB, 0x9E, 0x77, 0xDB, 0xD3, 0x02, 0xEF,
		]);
		let file = task::block_on(File::from(&mut buffer)).expect("Failed to parse buffer");
		assert_eq!(file.header.version, 0);
		assert_eq!(
			file.index.table,
			parser::v0::PartitionTable {
				hash: Uuid::parse_str("4b26c471-3098-4cce-9cdb-9e77dbd302ef").unwrap(),
				size: 0
			}
		);
	}

	// #[test]
	// fn it_writes() {
	// 	let mut buffer = Cursor::new(vec![
	// 		0x50, 0x58, 0x4C, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4B, 0x26, 0xC4, 0x71, 0x30,
	// 		0x98, 0x4C, 0xCE, 0x9C, 0xDB, 0x9E, 0x77, 0xDB, 0xD3, 0x02, 0xEF,
	// 	]);

	// 	println!("len:{} : {:X?}", buffer.get_ref().len(), buffer.get_ref());

	// 	let mut file = File::from(&mut buffer).expect("Failed to parse buffer");
	// 	assert_eq!(file.header.version, 0);
	// 	assert_eq!(
	// 		file.database.table,
	// 		v0::PartitionTable {
	// 			hash: Uuid::parse_str("4b26c471-3098-4cce-9cdb-9e77dbd302ef").unwrap(),
	// 			size: 0
	// 		}
	// 	);

	// 	let doc = Group::new(
	// 		Some(Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap()),
	// 		"Root",
	// 		Vec2::new(0., 0.),
	// 		vec![Rc::new(DocumentNode::Note(Note::new(
	// 			Some(Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap()),
	// 			"Foo",
	// 			Vec2::new(0., 0.),
	// 		)))],
	// 	);

	// 	let mut buffer = Cursor::new(vec![
	// 		0x50, 0x58, 0x4C, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4B, 0x26, 0xC4, 0x71, 0x30,
	// 		0x98, 0x4C, 0xCE, 0x9C, 0xDB, 0x9E, 0x77, 0xDB, 0xD3, 0x02, 0xEF,
	// 	]);
	// 	file.append(&Node::Group(doc), &mut buffer).unwrap();

	// 	println!("len:{} : {:X?}", buffer.get_ref().len(), buffer.get_ref());

	// 	let file = File::from(&mut buffer).expect("Failed to parse buffer");
	// 	assert_eq!(
	// 		file.database.table,
	// 		v0::PartitionTable {
	// 			hash: Uuid::parse_str("4b26c471-3098-4cce-9cdb-9e77dbd302ef").unwrap(),
	// 			size: 127
	// 		}
	// 	);
	// }
}
