// use crate::file::parser::{
// 	parse_header, parse_partition_table, parse_partition_table_rows, Header, PartitionTable,
// 	PartitionTableRow,
// };
// use crate::Node;
// use std::collections::HashMap;
// use std::io::prelude::*;
// use std::io::SeekFrom;
// use uuid::Uuid;

// #[derive(Debug)]
// pub enum Error {
// 	ParseError,
// 	Incomplete,
// 	NotFound,
// 	Other,
// }

// impl std::fmt::Display for Error {
// 	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
// 		match *self {
// 			Error::ParseError => write!(f, "Could not parse the file."),
// 			Error::Incomplete => write!(f, "Incomplete file."),
// 			Error::NotFound => write!(f, "File not found."),
// 			Error::Other => write!(f, "Unexpected error."),
// 		}
// 	}
// }

// impl From<std::io::Error> for Error {
// 	fn from(error: std::io::Error) -> Self {
// 		match error.kind() {
// 			std::io::ErrorKind::NotFound => Error::NotFound,
// 			_ => Error::Other,
// 		}
// 	}
// }

// impl From<nom::Err<(&[u8], nom::error::ErrorKind)>> for Error {
// 	fn from(error: nom::Err<(&[u8], nom::error::ErrorKind)>) -> Self {
// 		match error {
// 			nom::Err::Incomplete(_) => Error::Incomplete,
// 			_ => Error::ParseError,
// 		}
// 	}
// }

// pub struct Reader<'file, F>
// where
// 	F: Read + Seek,
// {
// 	file: &'file F,
// 	pub header: Header,
// 	pub table: PartitionTable,
// 	chunks: HashMap<Uuid, PartitionTableRow>,
// }

// impl<'reader, R> Reader<'reader, R>
// where
// 	R: Read + Seek,
// {
// 	pub async fn from<'f>(file: &'f mut R) -> Result<Reader<'f, R>, Error> {
// 		let mut buffer = [0u8; 5];
// 		file.seek(SeekFrom::Start(0))?;
// 		file.read(&mut buffer)?;

// 		let (_, header) = parse_header(&buffer)?;

// 		let mut buffer = [0u8; 20];
// 		file.seek(SeekFrom::End(-20))?;
// 		file.read(&mut buffer)?;

// 		let (_, table) = parse_partition_table(&buffer)?;

// 		let rows: Vec<PartitionTableRow> = if table.size == 0 {
// 			vec![]
// 		} else {
// 			let mut buffer = vec![0u8; table.size as usize];
// 			file.seek(SeekFrom::Current(-(table.size as i64)))?;
// 			file.read(&mut buffer)?;

// 			let (_, rows) = parse_partition_table_rows(&buffer, table.size as usize)?;
// 			rows
// 		};

// 		let mut chunks: HashMap<Uuid, PartitionTableRow> = HashMap::new();

// 		for row in rows.into_iter() {
// 			chunks.insert(row.id, row);
// 		}

// 		Ok(Reader {
// 			file,
// 			header,
// 			table,
// 			chunks,
// 		})
// 	}

// 	pub async fn read(&self, id: Uuid) -> Result<Node, Error> {
// 		Err(Error::NotFound)
// 	}
// }

// pub struct Writer<'writer, W>
// where
// 	W: Write + Seek,
// {
// 	file: &'writer W,
// }

// impl<'writer, W> Writer<'writer, W>
// where
// 	W: Write + Seek,
// {
// 	pub async fn from<'f>(file: &'f mut W) -> Result<Writer<'f, W>, Error> {
// 		Ok(Writer { file })
// 	}

// 	pub async fn write(&self, node: &Node) -> Result<(), Error> {
// 		Ok(())
// 	}

// 	// pub async fn append<'reader, R>(
// 	// 	&self,
// 	// 	reader: Reader<'reader, R>,
// 	// 	node: Node,
// 	// ) -> Result<(), Error>
// 	// where
// 	// 	R: Read + Seek,
// 	// {
// 	// 	Ok(())
// 	// }
// }

// #[cfg(test)]
// mod tests {
// 	use super::*;
// 	use crate::{DocumentNode, Group, Node, Note};
// 	use async_std::task;
// 	use math::Vec2;
// 	use std::io::Cursor;
// 	use std::rc::Rc;
// 	use uuid::Uuid;

// 	#[test]
// 	fn it_reads_file() {
// 		let mut file = Cursor::new(vec![
// 			0x50, 0x58, 0x4C, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4B, 0x26, 0xC4, 0x71, 0x30,
// 			0x98, 0x4C, 0xCE, 0x9C, 0xDB, 0x9E, 0x77, 0xDB, 0xD3, 0x02, 0xEF,
// 		]);
// 		let reader = task::block_on(Reader::from(&mut file)).unwrap();
// 		println!("Version {}", reader.header.version);
// 		println!("Table hash {}", reader.table.hash);
// 		println!("Table rows {}", reader.chunks.len());
// 	}

// 	#[test]
// 	fn it_writes_file() {
// 		let mut file: Cursor<Vec<u8>> = Cursor::new(Vec::new());
// 		let doc = Group::new(
// 			Some(Uuid::parse_str("fc2c9e3e-2cd7-4375-a6fe-49403cc9f82b").unwrap()),
// 			"Root",
// 			Vec2::new(0., 0.),
// 			vec![Rc::new(DocumentNode::Note(Note::new(
// 				Some(Uuid::parse_str("1c3deaf3-3c7f-444d-9e05-9ddbcc2b9391").unwrap()),
// 				"Foo",
// 				Vec2::new(0., 0.),
// 			)))],
// 		);

// 		let writer = task::block_on(Writer::from(&mut file)).unwrap();
// 		task::block_on(writer.write(&Node::Group(doc))).unwrap();

// 		println!("{:X?}", file.bytes());
// 	}

// 	use nom::number::complete::{le_f32, le_u16, le_u32, le_u64, le_u8};

// 	#[test]
// 	fn it_blep() {
// 		// let (_, b) = le_u32(b"\x01\x00\x00\x00").unwrap();
// 		println!("u32 to bytes {:X?}", 20u32.to_le_bytes());
// 	}
// }
