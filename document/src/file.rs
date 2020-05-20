use crate::parser;
use crate::parser::IParser;
use crate::Node;
use async_std::io;
use async_std::io::prelude::*;
use collections::bitvec;
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
				parser::v0::PartitionTable {
					hash,
					size: 0,
					root_child: 0,
				},
				vec![],
			),
		}
	}

	pub async fn from<S>(storage: &mut S) -> Result<File, FileStorageError>
	where
		S: parser::ReadAt + std::marker::Send + std::marker::Unpin,
	{
		let mut buffer = [0u8; 5];
		storage.read_at(io::SeekFrom::Start(0), &mut buffer).await?;
		let (_, header) = parser::Header::parse(&buffer)?;

		let mut buffer = [0u8; 24];
		storage.read_at(io::SeekFrom::End(-24), &mut buffer).await?;

		let (_, table) = match header.version {
			0 => <parser::v0::PartitionTable as IParser>::parse(&buffer),
			_ => panic!(FileStorageError::VersionNotSupported),
		}?;

		let rows: Vec<parser::v0::PartitionTableRow> = if table.size == 0 {
			vec![]
		} else {
			let mut buffer = vec![0u8; table.size as usize];
			storage
				.read_at(io::SeekFrom::End(-24 - (table.size as i64)), &mut buffer)
				.await?;

			let (_, rows) = match header.version {
				0 => many0(<parser::v0::PartitionTableRow as IParser>::parse)(&buffer),
				_ => panic!(FileStorageError::VersionNotSupported),
			}?;
			rows
		};

		Ok(File {
			header,
			index: parser::v0::PartitionIndex::new(table, rows),
		})
	}

	async fn write_node<S>(
		&mut self,
		storage: &mut S,
		node: &Node,
		offset: u64,
	) -> io::Result<usize>
	where
		S: io::Write + std::marker::Send + std::marker::Unpin,
	{
		let size = parser::v0::IParser::write(node, &mut self.index, storage, offset).await?;
		Ok(size)
	}

	async fn write_partition<S>(&mut self, storage: &mut S) -> io::Result<usize>
	where
		S: io::Write + std::marker::Send + std::marker::Unpin,
	{
		let mut size: usize = 0;
		let mut row_dependencies = bitvec![0; self.index.rows.len()];
		let root_idx = self.index.table.root_child as usize;
		let root_uuid = self
			.index
			.rows
			.get(self.index.table.root_child as usize)
			.unwrap()
			.id;
		let mut row_to_visit: Vec<usize> = Vec::with_capacity(self.index.rows.len());
		row_to_visit.push(root_idx);
		while let Some(i) = row_to_visit.pop() {
			row_dependencies.set(i, true);
			if let Some(row) = self.index.rows.get(i) {
				for child in row.children.iter() {
					row_to_visit.push(*child as usize);
				}
			}
		}
		self.index.rows = self
			.index
			.rows
			.drain(..)
			.enumerate()
			.filter(|(c, _)| row_dependencies[*c])
			.map(|(_, row)| row)
			.collect::<Vec<_>>();
		self.index.reindex_rows();
		for row in self.index.rows.iter() {
			size += row.write(storage).await?;
		}
		self.index.table.size = size as u32;
		self.index.table.root_child = *self.index.index_uuid.get(&root_uuid).unwrap() as u32;
		size += self.index.table.write(storage).await?;
		Ok(size)
	}

	pub async fn write<S>(&mut self, storage: &mut S, root: &Node) -> io::Result<usize>
	where
		S: io::Write + io::Seek + std::marker::Send + std::marker::Unpin,
	{
		storage.seek(io::SeekFrom::Start(0)).await?;
		let mut size: usize = 0;
		size += self.header.write(storage).await?;
		size += self.write_node(storage, root, 0u64).await?;
		self.index.table.root_child = (self.index.rows.len() as u32) - 1;
		size += self.write_partition(storage).await?;
		Ok(size)
	}

	pub async fn update<S>(&mut self, storage: &mut S, node: &Node) -> io::Result<usize>
	where
		S: io::Write + io::Seek + std::marker::Send + std::marker::Unpin,
	{
		let offset = storage.seek(io::SeekFrom::End(0)).await?;
		let mut size: usize = 0;
		size += self.write_node(storage, node, offset).await?;
		size += self.write_partition(storage).await?;
		Ok(size)
	}

	pub async fn update_metadata_only<S>(
		&mut self,
		storage: &mut S,
		node: &Node,
	) -> io::Result<usize>
	where
		S: io::Write + std::marker::Send + std::marker::Unpin,
	{
		if let Some(row_idx) = self.index.index_uuid.get(&node.id()).copied() {
			let (chunk_offset, chunk_size) = {
				let row = self.index.rows.get(row_idx).unwrap();
				(row.chunk_offset, row.chunk_size)
			};
			let mut size: usize = 0;
			size += self.write_node(&mut io::sink(), node, 0u64).await?;
			let mut row = self.index.rows.get_mut(row_idx).unwrap();
			row.chunk_offset = chunk_offset;
			row.chunk_size = chunk_size;
			size += self.write_partition(storage).await?;
			Ok(size)
		} else {
			Err(io::ErrorKind::NotFound.into())
		}
	}

	pub async fn get_root_node<S>(&mut self, storage: &mut S) -> io::Result<Node>
	where
		S: parser::ReadAt + std::marker::Send + std::marker::Unpin,
	{
		if (self.index.table.root_child as usize) > self.index.rows.len() {
			Err(io::ErrorKind::NotFound.into())
		} else {
			let row_id = self
				.index
				.rows
				.get(self.index.table.root_child as usize)
				.unwrap()
				.id;
			self.get_node_by_uuid(storage, row_id).await
		}
	}

	pub async fn get_node_by_uuid<S>(&mut self, storage: &mut S, id: Uuid) -> io::Result<Node>
	where
		S: parser::ReadAt + std::marker::Send + std::marker::Unpin,
	{
		if !self.index.index_uuid.contains_key(&id) {
			Err(io::ErrorKind::NotFound.into())
		} else {
			let idx = self.index.index_uuid.get(&id).unwrap();
			let row = self.index.rows.get(*idx).unwrap();
			let chunk_offset = row.chunk_offset;
			let chunk_size = row.chunk_size;
			let mut bytes: Vec<u8> = Vec::with_capacity(chunk_size as usize);
			let mut children: Vec<Node> = Vec::new();
			storage
				.read_at(io::SeekFrom::Start(chunk_offset), &mut bytes)
				.await?;
			if let Ok((_, node)) = <Node as parser::v0::IParser>::parse(
				&self.index,
				row,
				storage,
				&bytes[..],
				&mut children,
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
	use async_std::fs;
	use async_std::io;
	use async_std::task;
	use math::Vec2;
	use std::sync::Arc;
	use uuid::Uuid;

	#[test]
	fn it_reads_empty_file() {
		task::block_on(async {
			let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(vec![
				0x50, 0x58, 0x4C, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4B,
				0x26, 0xC4, 0x71, 0x30, 0x98, 0x4C, 0xCE, 0x9C, 0xDB, 0x9E, 0x77, 0xDB, 0xD3, 0x02,
				0xEF,
			]);
			let file = File::from(&mut buffer)
				.await
				.expect("Failed to parse buffer.");
			assert_eq!(file.header.version, 0);
			assert_eq!(
				file.index.table,
				parser::v0::PartitionTable {
					hash: Uuid::parse_str("4b26c471-3098-4cce-9cdb-9e77dbd302ef").unwrap(),
					size: 0,
					root_child: 0,
				}
			);
			assert_eq!(file.index.rows.len(), 0);
		});
	}

	#[test]
	fn it_writes_reads_file() {
		task::block_on(async {
			let doc = Node::Group(Group::new(
				Some(Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap()),
				"Root",
				Vec2::new(0., 0.),
				// vec![],
				vec![Arc::new(DocumentNode::Note(Note::new(
					Some(Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap()),
					"Foo",
					Vec2::new(0., 0.),
				)))],
			));
			let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());
			let mut file =
				File::empty(Uuid::parse_str("4b26c471-3098-4cce-9cdb-9e77dbd302ef").unwrap());
			let len = file
				.write(&mut buffer, &doc)
				.await
				.expect("Failed to write buffer.");
			assert_eq!(len, 158);
			assert_eq!(buffer.get_ref().len(), 158);
			let mut file = File::from(&mut buffer)
				.await
				.expect("Failed to parse buffer.");
			assert_eq!(file.header.version, 0);
			assert_eq!(
				file.index.table,
				parser::v0::PartitionTable {
					hash: Uuid::parse_str("4b26c471-3098-4cce-9cdb-9e77dbd302ef").unwrap(),
					size: 129,
					root_child: 1,
				}
			);
			assert_eq!(file.index.rows.len(), 2);
			if let Ok(Node::Group(group)) = file.get_root_node(&mut buffer).await {
				assert_eq!(
					group.id,
					Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap()
				);
				assert_eq!(*group.name, "Root");
				assert_eq!(*group.position, Vec2::new(0., 0.));
				assert_eq!(group.children.len(), 1);
				if let DocumentNode::Note(note) = &**group.children.get(0).unwrap() {
					assert_eq!(*note.note, "Foo");
				} else {
					panic!("Could not get child 0");
				}
			} else {
				panic!("Could not get root node");
			}
		});
	}

	#[test]
	fn it_updates_reads_file() {
		task::block_on(async {
			let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());
			// Init file
			{
				let doc = Node::Group(Group::new(
					Some(Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap()),
					"Root",
					Vec2::new(0., 0.),
					// vec![],
					vec![Arc::new(DocumentNode::Note(Note::new(
						Some(Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap()),
						"Foo",
						Vec2::new(0., 0.),
					)))],
				));
				let mut file =
					File::empty(Uuid::parse_str("4b26c471-3098-4cce-9cdb-9e77dbd302ef").unwrap());
				let len = file
					.write(&mut buffer, &doc)
					.await
					.expect("Failed to write buffer.");
				assert_eq!(len, 158);
				assert_eq!(buffer.get_ref().len(), 158);
			}

			// Stip note from group and append to current file
			{
				let mut file = File::from(&mut buffer)
					.await
					.expect("Failed to parse buffer.");
				let doc = Node::Group(Group::new(
					Some(Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap()),
					"Root",
					Vec2::new(0., 0.),
					vec![],
				));
				let len = file
					.update(&mut buffer, &doc)
					.await
					.expect("Failed to write buffer.");
				assert_eq!(len, 87);
				assert_eq!(buffer.get_ref().len(), 245);
				println!("{:?}", file.index);
			}

			// Assert that note is gone
			{
				let mut file = File::from(&mut buffer)
					.await
					.expect("Failed to parse buffer.");
				assert_eq!(file.header.version, 0);
				assert_eq!(file.index.rows.len(), 1);
				if let Ok(Node::Group(group)) = file.get_root_node(&mut buffer).await {
					assert_eq!(*group.name, "Root");
					assert_eq!(*group.position, Vec2::new(0., 0.));
					assert_eq!(group.children.len(), 0);
				} else {
					panic!("Could not get node fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b");
				}
			}
		});
	}

	#[test]
	fn it_updates_metadata_only_reads_file() {
		task::block_on(async {
			let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());
			// Init file
			{
				let doc = Node::Group(Group::new(
					Some(Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap()),
					"Root",
					Vec2::new(0., 0.),
					// vec![],
					vec![Arc::new(DocumentNode::Note(Note::new(
						Some(Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap()),
						"Foo",
						Vec2::new(0., 0.),
					)))],
				));
				let mut file =
					File::empty(Uuid::parse_str("4b26c471-3098-4cce-9cdb-9e77dbd302ef").unwrap());
				let len = file
					.write(&mut buffer, &doc)
					.await
					.expect("Failed to write buffer.");
				assert_eq!(len, 158);
				assert_eq!(buffer.get_ref().len(), 158);
			}

			// Stip note from group and append to current file
			{
				let mut file = File::from(&mut buffer)
					.await
					.expect("Failed to parse buffer.");
				let doc = Node::Group(Group::new(
					Some(Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap()),
					"Root",
					Vec2::new(0., 0.),
					vec![],
				));
				let len = file
					.update_metadata_only(&mut buffer, &doc)
					.await
					.expect("Failed to write buffer.");
				assert_eq!(len, 87);
				assert_eq!(buffer.get_ref().len(), 221);
			}

			// Assert that note is gone
			{
				let mut file = File::from(&mut buffer)
					.await
					.expect("Failed to parse buffer.");
				assert_eq!(file.header.version, 0);
				assert_eq!(file.index.rows.len(), 1);
				if let Ok(Node::Group(group)) = file.get_root_node(&mut buffer).await {
					assert_eq!(*group.name, "Root");
					assert_eq!(*group.position, Vec2::new(0., 0.));
					assert_eq!(group.children.len(), 0);
				} else {
					panic!("Could not get node fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b");
				}
			}
		});
	}

	#[test]
	fn it_dumps_to_disk() {
		task::block_on(async {
			let mut buffer = fs::OpenOptions::new()
				.truncate(true)
				.create(true)
				.write(true)
				.open("it_dump_to_disk.bin")
				.await
				.expect("Could not open file.");
			let doc = Node::Group(Group::new(
				Some(Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap()),
				"Root",
				Vec2::new(0., 0.),
				// vec![],
				vec![Arc::new(DocumentNode::Note(Note::new(
					Some(Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap()),
					"Foo",
					Vec2::new(0., 0.),
				)))],
			));
			let mut file =
				File::empty(Uuid::parse_str("4b26c471-3098-4cce-9cdb-9e77dbd302ef").unwrap());
			let len = file
				.write(&mut buffer, &doc)
				.await
				.expect("Failed to write buffer.");
			assert_eq!(len, 158);

			buffer.sync_all().await.expect("Sync to disk.");

			let metadata = buffer.metadata().await.expect("Could not get metadata.");
			assert_eq!(metadata.len(), 158);

			fs::remove_file("it_dump_to_disk.bin")
				.await
				.expect("Could not remove file.");
		});
	}
}
