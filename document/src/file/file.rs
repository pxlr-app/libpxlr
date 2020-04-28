use crate::file;
use crate::file::part;
use crate::file::writer::{WriteTo, Writer};
use crate::Node;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};
use uuid::Uuid;

#[derive(Debug)]
pub enum Error {
	Unknown,
	VersionNotSupported,
	NodeNotSupported,
	ParseError(nom::Err<((), nom::error::ErrorKind)>),
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			Error::Unknown => write!(f, "Unknown error."),
			Error::VersionNotSupported => write!(f, "Version not supported."),
			Error::NodeNotSupported => write!(f, "Node not supported."),
			Error::ParseError(_) => write!(f, "Could not parse the file."),
		}
	}
}

impl From<std::io::Error> for Error {
	fn from(error: std::io::Error) -> Self {
		match error.kind() {
			_ => Error::Unknown,
		}
	}
}

impl From<nom::Err<(&[u8], nom::error::ErrorKind)>> for Error {
	fn from(error: nom::Err<(&[u8], nom::error::ErrorKind)>) -> Self {
		match error {
			nom::Err::Incomplete(e) => Error::ParseError(nom::Err::Incomplete(e)),
			nom::Err::Error(e) => Error::ParseError(nom::Err::Error(((), e.1))),
			nom::Err::Failure(e) => Error::ParseError(nom::Err::Error(((), e.1))),
		}
	}
}

#[derive(Debug)]
pub struct File {
	pub header: part::Header,
	pub table: part::PartitionTable,
	pub rows: Vec<part::PartitionTableRow>,
	pub chunks: HashMap<Uuid, usize>,
}

impl File {
	pub async fn from<R: Read + Seek>(reader: &mut R) -> Result<File, file::Error> {
		let mut buffer = [0u8; 5];
		reader.seek(SeekFrom::Start(0))?;
		reader.read(&mut buffer)?;

		let (_, header) = <part::Header as file::reader::v0::Reader>::from_bytes(&buffer)?;

		let mut buffer = [0u8; 20];
		reader.seek(SeekFrom::End(-20))?;
		reader.read(&mut buffer)?;

		let (_, table) = match header.version {
			0 => <part::PartitionTable as file::reader::v0::Reader>::from_bytes(&buffer),
			_ => panic!(file::Error::VersionNotSupported),
		}?;

		let rows: Vec<part::PartitionTableRow> = if table.size == 0 {
			vec![]
		} else {
			let mut buffer = vec![0u8; table.size as usize];
			reader.seek(SeekFrom::Current(-20 - (table.size as i64)))?;
			reader.read(&mut buffer)?;

			let (_, rows) = match header.version {
				0 => {
					<Vec<part::PartitionTableRow> as file::reader::v0::Reader>::from_bytes(&buffer)
				}
				_ => panic!(file::Error::VersionNotSupported),
			}?;
			rows
		};

		let mut chunks: HashMap<Uuid, usize> = HashMap::new();

		for (i, row) in rows.iter().enumerate() {
			chunks.insert(row.id, i);
		}

		Ok(File {
			header,
			table,
			rows,
			chunks,
		})
	}

	pub async fn read<R: Read + Seek>(&self, _file: &mut R, _id: Uuid) -> Option<Node> {
		None
	}

	pub async fn append<W: Write + Seek>(
		&mut self,
		node: &Node,
		writer: &mut W,
	) -> Result<usize, file::Error> {
		writer.seek(SeekFrom::End(0))?;
		let mut size = node.write(self, writer)?;
		let table_size = self.rows.write_to(writer)?;
		self.table.size = table_size as u32;
		size += table_size;
		size += self.table.write_to(writer)?;
		Ok(size)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{DocumentNode, Group, Node, Note};
	use async_std::task;
	use math::Vec2;
	use std::io::Cursor;
	use std::rc::Rc;
	use uuid::Uuid;

	#[test]
	fn it_reads_empty_file() {
		let mut buffer = Cursor::new(vec![
			0x50, 0x58, 0x4C, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4B, 0x26, 0xC4, 0x71, 0x30,
			0x98, 0x4C, 0xCE, 0x9C, 0xDB, 0x9E, 0x77, 0xDB, 0xD3, 0x02, 0xEF,
		]);
		let file = task::block_on(File::from(&mut buffer)).unwrap();
		println!("Version {}", file.header.version);
		println!("Table hash={}, size={}", file.table.hash, file.table.size);
		println!("Table rows {:?}", file.rows);
	}

	#[test]
	fn it_writes_file() {
		let mut buffer = Cursor::new(vec![
			0x50, 0x58, 0x4C, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4B, 0x26, 0xC4, 0x71, 0x30,
			0x98, 0x4C, 0xCE, 0x9C, 0xDB, 0x9E, 0x77, 0xDB, 0xD3, 0x02, 0xEF,
		]);
		println!("len:{} : {:X?}", buffer.get_ref().len(), buffer.get_ref());
		let mut file = task::block_on(File::from(&mut buffer)).unwrap();

		let doc = Group::new(
			Some(Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap()),
			"Root",
			Vec2::new(0., 0.),
			vec![Rc::new(DocumentNode::Note(Note::new(
				Some(Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap()),
				"Foo",
				Vec2::new(0., 0.),
			)))],
		);

		let mut buffer: Cursor<Vec<u8>> = Cursor::new(buffer.into_inner());
		task::block_on(file.append(&Node::Group(doc), &mut buffer)).unwrap();

		println!("len:{} : {:X?}", buffer.get_ref().len(), buffer.get_ref());

		let mut file = task::block_on(File::from(&mut buffer)).unwrap();
		println!("Version {}", file.header.version);
		println!("Table hash={}, size={}", file.table.hash, file.table.size);
		println!("Table rows {:?}", file.rows);
	}
}
