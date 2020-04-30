use crate::parser::v0::PartitionTableParse;
use crate::Node;
use math::{Extent2, Vec2};
use nom::bytes::complete::{tag, take};
use nom::multi::many0;
use nom::number::complete::{le_f32, le_u32, le_u8};
use nom::IResult;
use std::io;
use uuid::Uuid;

const MAGIC_NUMBER: &'static str = "PXLR";

pub trait Parser {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self>
	where
		Self: Sized;

	fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<usize>;
}

#[derive(Debug, PartialEq)]
pub struct Header {
	pub version: u8,
}

impl Parser for Header {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, _) = tag(MAGIC_NUMBER)(bytes)?;
		let (bytes, version) = le_u8(bytes)?;
		Ok((bytes, Header { version }))
	}

	fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
		writer.write(MAGIC_NUMBER.as_bytes())?;
		writer.write(&self.version.to_le_bytes())?;
		Ok(5)
	}
}

pub mod v0 {
	use crate::Node;
	use math::{Extent2, Vec2};
	use nom::multi::many0;
	use nom::multi::many_m_n;
	use nom::number::complete::{le_u16, le_u32, le_u64, le_u8};
	use nom::IResult;
	use std::collections::HashMap;
	use std::io;
	use uuid::Uuid;

	pub trait ReadSeek: io::Read + io::Seek {}

	pub struct Database<'a> {
		reader: &'a mut dyn ReadSeek,
		pub table: PartitionTable,
		pub rows: Vec<PartitionTableRow>,
		pub lut_rows: HashMap<Uuid, usize>,
	}

	impl<'a> Database<'a> {
		pub fn new(
			reader: &'a mut dyn ReadSeek,
			table: PartitionTable,
			rows: Vec<PartitionTableRow>,
		) -> Self {
			let mut lut_rows: HashMap<Uuid, usize> = HashMap::new();

			for (i, row) in rows.iter().enumerate() {
				lut_rows.insert(row.id, i);
			}

			Database {
				reader,
				table,
				rows,
				lut_rows,
			}
		}

		pub fn read_chunk(&mut self, index: usize) -> io::Result<Vec<u8>> {
			let row = self.rows.get(index).expect("Row not found.");
			self.reader.seek(io::SeekFrom::Start(row.chunk_offset))?;
			let mut bytes: Vec<u8> = Vec::with_capacity(row.chunk_size as usize);
			self.reader.read(&mut bytes)?;
			Ok(bytes)
		}

		pub fn read_root(&mut self) -> io::Result<Node> {
			let bytes = self.read_chunk(0)?;
			let root_chunk = self.rows.get(0).expect("No root chunk.");
			let (_, node) =
				<Node as self::PartitionTableParse>::parse(self, root_chunk, &bytes[..])
					.expect("Expected node.");
			Ok(node)
		}
	}

	pub trait PartitionTableParse {
		type Output;

		fn parse<'a, 'b>(
			file: &mut Database<'a>,
			row: &PartitionTableRow,
			bytes: &'b [u8],
		) -> IResult<&'b [u8], Self::Output>
		where
			Self::Output: Sized;

		fn write<'a, W: io::Write + io::Seek>(
			&self,
			file: &mut Database<'a>,
			writer: &mut W,
		) -> io::Result<usize>;
	}

	impl<T> PartitionTableParse for Vec<T>
	where
		T: PartitionTableParse,
	{
		type Output = Vec<T::Output>;

		fn parse<'a, 'b>(
			file: &mut Database<'a>,
			row: &PartitionTableRow,
			bytes: &'b [u8],
		) -> IResult<&'b [u8], Self::Output> {
			// TODO After https://github.com/Geal/nom/issues/1132 we might use simple form
			//      many0(|bytes| <T as PartitionTableParse>::parse(file, row, bytes))(bytes)
			let mut items: Vec<T::Output> = Vec::new();
			let mut remainder: &'b [u8] = bytes;
			loop {
				if let Ok((b, item)) = <T as PartitionTableParse>::parse(file, row, remainder) {
					remainder = b;
					items.push(item);
				} else {
					break;
				}
			}
			Ok((remainder, items))
		}

		fn write<'a, W: io::Write + io::Seek>(
			&self,
			file: &mut Database<'a>,
			writer: &mut W,
		) -> io::Result<usize> {
			let mut b: usize = 0;
			for item in self.iter() {
				b += item.write(file, writer)?;
			}
			Ok(b)
		}
	}

	#[derive(Debug, PartialEq)]
	pub struct PartitionTable {
		pub hash: Uuid,
		pub size: u32,
	}

	impl super::Parser for PartitionTable {
		fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
			let (bytes, size) = le_u32(bytes)?;
			let (bytes, hash) = Uuid::parse(bytes)?;
			Ok((bytes, PartitionTable { size, hash }))
		}

		fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
			writer.write(&self.size.to_le_bytes())?;
			self.hash.write(writer)?;
			Ok(5)
		}
	}

	#[derive(Debug, PartialEq)]
	pub enum ChunkType {
		Group,
		Note,
		Sprite,
		LayerGroup,
		CanvasI,
		CanvasIXYZ,
		CanvasUV,
		CanvasRGB,
		CanvasRGBA,
		CanvasRGBAXYZ,
	}

	impl super::Parser for ChunkType {
		fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
			let (bytes, index) = le_u16(bytes)?;
			let value = match index {
				0 => ChunkType::Group,
				1 => ChunkType::Note,
				2 => ChunkType::Sprite,
				3 => ChunkType::LayerGroup,
				4 => ChunkType::CanvasI,
				5 => ChunkType::CanvasIXYZ,
				6 => ChunkType::CanvasUV,
				7 => ChunkType::CanvasRGB,
				8 => ChunkType::CanvasRGBA,
				9 => ChunkType::CanvasRGBAXYZ,
				_ => panic!("Unknown chunk type"),
			};
			Ok((bytes, value))
		}

		fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
			let index: u16 = match self {
				ChunkType::Group => 0,
				ChunkType::Note => 1,
				ChunkType::Sprite => 2,
				ChunkType::LayerGroup => 3,
				ChunkType::CanvasI => 4,
				ChunkType::CanvasIXYZ => 5,
				ChunkType::CanvasUV => 6,
				ChunkType::CanvasRGB => 7,
				ChunkType::CanvasRGBA => 8,
				ChunkType::CanvasRGBAXYZ => 9,
				_ => panic!("Unknown chunk type"),
			};
			writer.write(&index.to_le_bytes())?;
			Ok(2)
		}
	}

	#[derive(Debug, PartialEq)]
	pub struct PartitionTableRow {
		pub id: Uuid,
		pub chunk_type: ChunkType,
		pub chunk_offset: u64,
		pub chunk_size: u32,
		pub position: Vec2<f32>,
		pub size: Extent2<u32>,
		pub name: String,
		pub children: Vec<u32>,
		pub preview: Vec<u8>,
	}

	impl super::Parser for PartitionTableRow {
		fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
			let (bytes, id) = Uuid::parse(bytes)?;
			let (bytes, chunk_type) = <ChunkType as super::Parser>::parse(bytes)?;
			let (bytes, chunk_offset) = le_u64(bytes)?;
			let (bytes, chunk_size) = le_u32(bytes)?;
			let (bytes, position) = Vec2::<f32>::parse(bytes)?;
			let (bytes, size) = Extent2::<u32>::parse(bytes)?;
			let (bytes, child_count) = le_u32(bytes)?;
			let (bytes, preview_size) = le_u32(bytes)?;
			let (bytes, name) = String::parse(bytes)?;
			let (bytes, children) =
				many_m_n(child_count as usize, child_count as usize, le_u32)(bytes)?;
			let (bytes, preview) =
				many_m_n(preview_size as usize, preview_size as usize, le_u8)(bytes)?;
			Ok((
				bytes,
				PartitionTableRow {
					id,
					chunk_type,
					chunk_offset,
					chunk_size,
					position,
					size,
					name,
					children,
					preview,
				},
			))
		}

		fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
			let mut b: usize = 54;
			self.id.write(writer)?;
			self.chunk_type.write(writer)?;
			writer.write(&self.chunk_offset.to_le_bytes())?;
			writer.write(&self.chunk_size.to_le_bytes())?;
			self.position.write(writer)?;
			self.size.write(writer)?;
			writer.write(&(self.children.len() as u32).to_le_bytes())?;
			writer.write(&(self.preview.len() as u32).to_le_bytes())?;
			b += self.name.write(writer)?;
			for child in self.children.iter() {
				b += writer.write(&child.to_le_bytes())?;
			}
			b += writer.write(&self.preview)?;
			Ok(b)
		}
	}
}

impl Parser for String {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, len) = le_u32(bytes)?;
		let (bytes, buffer) = take(len as usize)(bytes)?;
		Ok((bytes, std::str::from_utf8(buffer).unwrap().to_owned()))
	}

	fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
		let mut b: usize = 0;
		b += writer.write(&(self.len() as u32).to_le_bytes())?;
		b += writer.write(self.as_bytes())?;
		Ok(b)
	}
}

impl Parser for Uuid {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, buffer) = take(16usize)(bytes)?;
		Ok((bytes, Uuid::from_slice(buffer).unwrap()))
	}

	fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
		writer.write(self.as_bytes())?;
		Ok(16)
	}
}

impl Parser for Vec2<f32> {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, x) = le_f32(bytes)?;
		let (bytes, y) = le_f32(bytes)?;
		Ok((bytes, Vec2::new(x, y)))
	}

	fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
		writer.write(&self.x.to_le_bytes())?;
		writer.write(&self.y.to_le_bytes())?;
		Ok(8)
	}
}

impl Parser for Extent2<u32> {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, w) = le_u32(bytes)?;
		let (bytes, h) = le_u32(bytes)?;
		Ok((bytes, Extent2::new(w, h)))
	}

	fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
		writer.write(&self.w.to_le_bytes())?;
		writer.write(&self.h.to_le_bytes())?;
		Ok(8)
	}
}

#[derive(Debug)]
pub enum ParseError {
	Unknown,
	VersionNotSupported,
	NodeNotSupported,
	ParseError(nom::Err<((), nom::error::ErrorKind)>),
}

impl std::fmt::Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			ParseError::Unknown => write!(f, "Unknown error."),
			ParseError::VersionNotSupported => write!(f, "Version not supported."),
			ParseError::NodeNotSupported => write!(f, "Node not supported."),
			ParseError::ParseError(_) => write!(f, "Could not parse the file."),
		}
	}
}

impl From<std::io::Error> for ParseError {
	fn from(error: std::io::Error) -> Self {
		match error.kind() {
			_ => ParseError::Unknown,
		}
	}
}

impl From<nom::Err<(&[u8], nom::error::ErrorKind)>> for ParseError {
	fn from(error: nom::Err<(&[u8], nom::error::ErrorKind)>) -> Self {
		match error {
			nom::Err::Incomplete(e) => ParseError::ParseError(nom::Err::Incomplete(e)),
			nom::Err::Error(e) => ParseError::ParseError(nom::Err::Error(((), e.1))),
			nom::Err::Failure(e) => ParseError::ParseError(nom::Err::Error(((), e.1))),
		}
	}
}

pub struct File<'a> {
	pub header: Header,
	pub database: self::v0::Database<'a>,
}

impl<'a> File<'a> {
	pub fn from<R: self::v0::ReadSeek>(reader: &'a mut R) -> Result<File<'a>, ParseError> {
		let mut buffer = [0u8; 5];
		reader.seek(io::SeekFrom::Start(0))?;
		reader.read(&mut buffer)?;

		let (_, header) = Header::parse(&buffer)?;

		let mut buffer = [0u8; 20];
		reader.seek(io::SeekFrom::End(-20))?;
		reader.read(&mut buffer)?;

		let (_, table) = match header.version {
			0 => <self::v0::PartitionTable as Parser>::parse(&buffer),
			_ => panic!(ParseError::VersionNotSupported),
		}?;

		let rows: Vec<self::v0::PartitionTableRow> = if table.size == 0 {
			vec![]
		} else {
			let mut buffer = vec![0u8; table.size as usize];
			reader.seek(io::SeekFrom::Current(-20 - (table.size as i64)))?;
			reader.read(&mut buffer)?;

			let (_, rows) = match header.version {
				0 => many0(<self::v0::PartitionTableRow as Parser>::parse)(&buffer),
				_ => panic!(ParseError::VersionNotSupported),
			}?;
			rows
		};

		Ok(File {
			header,
			database: self::v0::Database::new(reader, table, rows),
		})
	}

	pub fn append<W: io::Write + io::Seek>(
		&mut self,
		node: &Node,
		writer: &mut W,
	) -> io::Result<usize> {
		writer.seek(io::SeekFrom::End(0))?;
		let mut size = node.write(&mut self.database, writer)?;
		let mut table_size: usize = 0;
		for row in self.database.rows.iter() {
			table_size += row.write(writer)?;
		}
		self.database.table.size = table_size as u32;
		size += self.database.table.write(writer)?;
		Ok(size + table_size)
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

	impl v0::ReadSeek for Cursor<Vec<u8>> {}

	#[test]
	fn it_reads_empty_file() {
		let mut buffer = Cursor::new(vec![
			0x50, 0x58, 0x4C, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4B, 0x26, 0xC4, 0x71, 0x30,
			0x98, 0x4C, 0xCE, 0x9C, 0xDB, 0x9E, 0x77, 0xDB, 0xD3, 0x02, 0xEF,
		]);
		let file = File::from(&mut buffer).expect("Failed to parse buffer");
		assert_eq!(file.header.version, 0);
		assert_eq!(
			file.database.table,
			v0::PartitionTable {
				hash: Uuid::parse_str("4b26c471-3098-4cce-9cdb-9e77dbd302ef").unwrap(),
				size: 0
			}
		);
	}

	#[test]
	fn it_writes() {
		let mut buffer = Cursor::new(vec![
			0x50, 0x58, 0x4C, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4B, 0x26, 0xC4, 0x71, 0x30,
			0x98, 0x4C, 0xCE, 0x9C, 0xDB, 0x9E, 0x77, 0xDB, 0xD3, 0x02, 0xEF,
		]);

		println!("len:{} : {:X?}", buffer.get_ref().len(), buffer.get_ref());

		let mut file = File::from(&mut buffer).expect("Failed to parse buffer");
		assert_eq!(file.header.version, 0);
		assert_eq!(
			file.database.table,
			v0::PartitionTable {
				hash: Uuid::parse_str("4b26c471-3098-4cce-9cdb-9e77dbd302ef").unwrap(),
				size: 0
			}
		);

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

		let mut buffer = Cursor::new(vec![
			0x50, 0x58, 0x4C, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4B, 0x26, 0xC4, 0x71, 0x30,
			0x98, 0x4C, 0xCE, 0x9C, 0xDB, 0x9E, 0x77, 0xDB, 0xD3, 0x02, 0xEF,
		]);
		file.append(&Node::Group(doc), &mut buffer).unwrap();

		println!("len:{} : {:X?}", buffer.get_ref().len(), buffer.get_ref());

		let file = File::from(&mut buffer).expect("Failed to parse buffer");
		assert_eq!(
			file.database.table,
			v0::PartitionTable {
				hash: Uuid::parse_str("4b26c471-3098-4cce-9cdb-9e77dbd302ef").unwrap(),
				size: 127
			}
		);
	}
}
