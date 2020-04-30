use crate::parser;
use crate::parser::Parser;
use crate::Node;
use async_std::io;
use async_std::io::prelude::*;
use nom::multi::many0;
use uuid::Uuid;

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

pub struct File {
	pub header: parser::Header,
	pub index: parser::v0::PartitionIndex,
}

impl File {
	pub fn empty(hash: Uuid) -> Self {
		File {
			header: parser::Header { version: 0 },
			index: parser::v0::PartitionIndex::new(
				parser::v0::PartitionTable { hash, size: 0 },
				vec![],
			),
		}
	}

	pub async fn from<S: io::Read + io::Write + io::Seek + std::marker::Unpin>(
		storage: &mut S,
	) -> Result<File, FileStorageError> {
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
			header,
			index: parser::v0::PartitionIndex::new(table, rows),
		})
	}

	pub async fn write<S: io::Read + io::Write + io::Seek + std::marker::Unpin>(
		&mut self,
		storage: &mut S,
		node: &Node,
	) -> io::Result<usize> {
		storage.seek(io::SeekFrom::Start(0)).await?;
		self.header.write(storage).await?;
		let mut size =
			parser::v0::PartitionTableParse::<S>::write(node, &mut self.index, storage).await?;
		let mut table_size: usize = 0;
		for row in self.index.rows.iter() {
			table_size += row.write(storage).await?;
		}
		self.index.table.size = table_size as u32;
		size += self.index.table.write(storage).await?;
		Ok(size + table_size)
	}

	pub async fn get_node<S: io::Read + io::Write + io::Seek + std::marker::Unpin>(
		&mut self,
		storage: &mut S,
		idx: usize,
	) -> io::Result<Node> {
		if idx >= self.index.rows.len() {
			Err(io::ErrorKind::NotFound.into())
		} else {
			let row = self.index.rows.get(idx).unwrap();
			let chunk_offset = row.chunk_offset;
			let chunk_size = row.chunk_size;
			let mut bytes: Vec<u8> = Vec::with_capacity(chunk_size as usize);
			storage.seek(io::SeekFrom::Start(chunk_offset)).await?;
			storage.read(&mut bytes).await?;
			if let Ok((_, node)) = <Node as parser::v0::PartitionTableParse<S>>::parse(
				&self.index,
				row,
				storage,
				&bytes[..],
			)
			.await
			{
				Ok(node)
			} else {
				Err(io::ErrorKind::InvalidData.into())
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::parser;
	use crate::{DocumentNode, Group, Node, Note};
	use async_std::io;
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
		let file = task::block_on(File::from(&mut buffer)).expect("Failed to parse buffer.");
		assert_eq!(file.header.version, 0);
		assert_eq!(
			file.index.table,
			parser::v0::PartitionTable {
				hash: Uuid::parse_str("4b26c471-3098-4cce-9cdb-9e77dbd302ef").unwrap(),
				size: 0
			}
		);
		assert_eq!(file.index.rows.len(), 0);
	}

	#[test]
	fn it_writes_reads_file() {
		let doc = Node::Group(Group::new(
			Some(Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap()),
			"Root",
			Vec2::new(0., 0.),
			// vec![],
			vec![Rc::new(DocumentNode::Note(Note::new(
				Some(Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap()),
				"Foo",
				Vec2::new(0., 0.),
			)))],
		));
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());
		let mut file =
			File::empty(Uuid::parse_str("4b26c471-3098-4cce-9cdb-9e77dbd302ef").unwrap());
		let len = task::block_on(file.write(&mut buffer, &doc)).expect("Failed to write buffer.");
		assert_eq!(len, 132);
		let mut file = task::block_on(File::from(&mut buffer)).expect("Failed to parse buffer.");
		assert_eq!(file.header.version, 0);
		assert_eq!(
			file.index.table,
			parser::v0::PartitionTable {
				hash: Uuid::parse_str("4b26c471-3098-4cce-9cdb-9e77dbd302ef").unwrap(),
				size: 127
			}
		);
		assert_eq!(file.index.rows.len(), 2);
		task::block_on(async {
			if let Ok(Node::Group(group)) = file.get_node(&mut buffer, 1).await {
				assert_eq!(*group.name, "Root");
				assert_eq!(*group.position, Vec2::new(0., 0.));
				assert_eq!(group.children.len(), 1);
				if let DocumentNode::Note(note) = &**group.children.get(0).unwrap() {
					assert_eq!(*note.note, "Foo");
				} else {
					panic!("Could not get child 0");
				}
			} else {
				panic!("Could not get node 0");
			}
		});
	}
}
