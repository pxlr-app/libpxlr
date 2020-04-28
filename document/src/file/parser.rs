use crate::color;
use crate::file::reader;
use crate::file::writer::*;
use math::{Extent2, Vec2};
use nom::bytes::complete::{tag, take};
use nom::multi::many_m_n;
use nom::number::complete::{le_f32, le_u16, le_u32, le_u64, le_u8};
use nom::IResult;
use std::io::Write;
use uuid::Uuid;

const MAGIC_NUMBER: &'static str = "PXLR";

impl reader::v0::Reader for String {
	fn from_bytes(bytes: &[u8]) -> IResult<&[u8], String> {
		let (bytes, len) = le_u32(bytes)?;
		let (bytes, buffer) = take(len as usize)(bytes)?;
		Ok((bytes, std::str::from_utf8(buffer).unwrap().to_owned()))
	}
}

impl Writer for String {
	fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		let mut b: usize = 0;
		b += writer.write(&(self.len() as u32).to_le_bytes())?;
		b += writer.write(self.as_bytes())?;
		Ok(b)
	}
}

impl reader::v0::Reader for Uuid {
	fn from_bytes(bytes: &[u8]) -> IResult<&[u8], Uuid> {
		let (bytes, buffer) = take(16usize)(bytes)?;
		Ok((bytes, Uuid::from_slice(buffer).unwrap()))
	}
}

impl Writer for Uuid {
	fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		writer.write(self.as_bytes())?;
		Ok(16)
	}
}

impl reader::v0::Reader for Vec2<f32> {
	fn from_bytes(bytes: &[u8]) -> IResult<&[u8], Vec2<f32>> {
		let (bytes, x) = le_f32(bytes)?;
		let (bytes, y) = le_f32(bytes)?;
		Ok((bytes, Vec2::new(x, y)))
	}
}

impl Writer for Vec2<f32> {
	fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		writer.write(&self.x.to_le_bytes())?;
		writer.write(&self.y.to_le_bytes())?;
		Ok(8)
	}
}

impl reader::v0::Reader for Extent2<u32> {
	fn from_bytes(bytes: &[u8]) -> IResult<&[u8], Extent2<u32>> {
		let (bytes, w) = le_u32(bytes)?;
		let (bytes, h) = le_u32(bytes)?;
		Ok((bytes, Extent2::new(w, h)))
	}
}

impl Writer for Extent2<u32> {
	fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		writer.write(&self.w.to_le_bytes())?;
		writer.write(&self.h.to_le_bytes())?;
		Ok(8)
	}
}

#[derive(Debug, PartialEq)]
pub struct Header {
	pub version: u8,
}

impl reader::v0::Reader for Header {
	fn from_bytes(bytes: &[u8]) -> IResult<&[u8], Header> {
		let (bytes, _) = tag(MAGIC_NUMBER)(bytes)?;
		let (bytes, version) = le_u8(bytes)?;
		Ok((bytes, Header { version }))
	}
}

impl Writer for Header {
	fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		writer.write(MAGIC_NUMBER.as_bytes())?;
		writer.write(&self.version.to_le_bytes())?;
		Ok(5)
	}
}

#[derive(Debug, PartialEq)]
pub struct PartitionTable {
	pub hash: Uuid,
	pub size: u32,
}

impl reader::v0::Reader for PartitionTable {
	fn from_bytes(bytes: &[u8]) -> IResult<&[u8], PartitionTable> {
		let (bytes, size) = le_u32(bytes)?;
		let (bytes, hash) = <Uuid as reader::v0::Reader>::from_bytes(bytes)?;
		Ok((bytes, PartitionTable { size, hash }))
	}
}

impl Writer for PartitionTable {
	fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		writer.write(&self.size.to_le_bytes())?;
		self.hash.write_to(writer)?;
		Ok(5)
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

impl reader::v0::Reader for PartitionTableRow {
	fn from_bytes(bytes: &[u8]) -> IResult<&[u8], PartitionTableRow> {
		let (bytes, id) = <Uuid as reader::v0::Reader>::from_bytes(bytes)?;
		let (bytes, chunk_type) = ChunkType::from_bytes(bytes)?;
		let (bytes, chunk_offset) = le_u64(bytes)?;
		let (bytes, chunk_size) = le_u32(bytes)?;
		let (bytes, position) = Vec2::<f32>::from_bytes(bytes)?;
		let (bytes, size) = Extent2::<u32>::from_bytes(bytes)?;
		let (bytes, child_count) = le_u32(bytes)?;
		let (bytes, preview_size) = le_u32(bytes)?;
		let (bytes, name) = String::from_bytes(bytes)?;
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
}

impl Writer for PartitionTableRow {
	fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		let mut b: usize = 54;
		self.id.write_to(writer)?;
		self.chunk_type.write_to(writer)?;
		writer.write(&self.chunk_offset.to_le_bytes())?;
		writer.write(&self.chunk_size.to_le_bytes())?;
		self.position.write_to(writer)?;
		self.size.write_to(writer)?;
		writer.write(&(self.children.len() as u32).to_le_bytes())?;
		writer.write(&(self.preview.len() as u32).to_le_bytes())?;
		b += self.name.write_to(writer)?;
		for child in self.children.iter() {
			b += writer.write(&child.to_le_bytes())?;
		}
		b += writer.write(&self.preview)?;
		Ok(b)
	}
}

#[derive(Debug, PartialEq)]
pub enum ChunkType {
	Group,
	Note,
	Sprite,
	CanvasI,
	CanvasIXYZ,
	CanvasUV,
	CanvasRGB,
	CanvasRGBA,
	CanvasRGBAXYZ,
}

impl reader::v0::Reader for ChunkType {
	fn from_bytes(bytes: &[u8]) -> IResult<&[u8], ChunkType> {
		let (bytes, index) = le_u16(bytes)?;
		let value = match index {
			0 => ChunkType::Group,
			1 => ChunkType::Note,
			2 => ChunkType::Sprite,
			3 => ChunkType::CanvasI,
			4 => ChunkType::CanvasIXYZ,
			5 => ChunkType::CanvasIXYZ,
			6 => ChunkType::CanvasUV,
			7 => ChunkType::CanvasRGBA,
			8 => ChunkType::CanvasRGBAXYZ,
			_ => panic!("Unknown chunk type"),
		};
		Ok((bytes, value))
	}
}

impl Writer for ChunkType {
	fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		let index: u16 = match self {
			ChunkType::Group => 0,
			ChunkType::Note => 1,
			ChunkType::Sprite => 2,
			ChunkType::CanvasI => 3,
			ChunkType::CanvasIXYZ => 4,
			ChunkType::CanvasIXYZ => 5,
			ChunkType::CanvasUV => 6,
			ChunkType::CanvasRGBA => 7,
			ChunkType::CanvasRGBAXYZ => 8,
			_ => panic!("Unknown chunk type"),
		};
		writer.write(&index.to_le_bytes())?;
		Ok(2)
	}
}

#[derive(Debug, PartialEq)]
pub struct Note {
	pub position: Vec2<f32>,
}

impl reader::v0::Reader for Note {
	fn from_bytes(bytes: &[u8]) -> IResult<&[u8], Note> {
		let (bytes, position) = Vec2::<f32>::from_bytes(bytes)?;
		Ok((bytes, Note { position }))
	}
}

impl Writer for Note {
	fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		self.position.write_to(writer)?;
		Ok(8)
	}
}

#[derive(Debug, PartialEq)]
pub struct Group {
	pub position: Vec2<f32>,
}

impl reader::v0::Reader for Group {
	fn from_bytes(bytes: &[u8]) -> IResult<&[u8], Group> {
		let (bytes, position) = Vec2::<f32>::from_bytes(bytes)?;
		Ok((bytes, Group { position }))
	}
}

impl Writer for Group {
	fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		self.position.write_to(writer)?;
		Ok(8)
	}
}

macro_rules! define_canvas {
	($name:ident, $color:path) => {
		#[derive(Debug, PartialEq)]
		pub struct $name {
			pub size: Extent2<u32>,
			pub data: Vec<$color>,
		}

		impl reader::v0::Reader for $name {
			fn from_bytes(bytes: &[u8]) -> IResult<&[u8], $name> {
				let (bytes, size) = Extent2::<u32>::from_bytes(bytes)?;
				let (bytes, data) = many_m_n(
					(size.w as usize) * (size.h as usize),
					(size.w as usize) * (size.h as usize),
					<$color as reader::v0::Reader>::from_bytes,
				)(bytes)?;
				Ok((bytes, $name { size, data }))
			}
		}
		impl Writer for $name {
			fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<usize> {
				let mut b: usize = 8;
				self.size.write_to(writer)?;
				for color in self.data.iter() {
					b += color.write_to(writer)?;
				}
				Ok(b)
			}
		}
	};
}

define_canvas!(CanvasI, color::I);
define_canvas!(CanvasIXYZ, color::IXYZ);
define_canvas!(CanvasUV, color::UV);
define_canvas!(CanvasRGB, color::RGB);
define_canvas!(CanvasRGBA, color::RGBA);
define_canvas!(CanvasRGBAXYZ, color::RGBAXYZ);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::file::reader;
	use std::io::{BufWriter, Cursor};
	// use crate::color;
	// use math::{Extent2, Vec2};

	#[test]
	fn it_parse_header() {
		let header = Header { version: 1 };
		let mut writer = BufWriter::new(Cursor::new(Vec::new()));
		let len = header.write_to(&mut writer).unwrap();
		assert_eq!(len, 5);

		let buffer = writer.buffer();
		let (buffer, header2) = <Header as reader::v0::Reader>::from_bytes(buffer).unwrap();
		assert_eq!(header.version, header2.version);
		assert_eq!(buffer, []);
	}

	#[test]
	fn it_parse_partition_table_row() {
		let row = PartitionTableRow {
			id: Uuid::new_v4(),
			chunk_type: ChunkType::Note,
			chunk_offset: 0,
			chunk_size: 8,
			position: Vec2::new(10., 10.),
			size: Extent2::new(0, 0),
			name: "Foo".into(),
			children: vec![],
			preview: vec![],
		};
		let mut writer = BufWriter::new(Cursor::new(Vec::new()));
		let len = row.write_to(&mut writer).unwrap();
		assert_eq!(len, 61);

		let buffer = writer.buffer();
		let (buffer, row2) = <PartitionTableRow as reader::v0::Reader>::from_bytes(buffer).unwrap();
		assert_eq!(row.id, row2.id);
		assert_eq!(row.chunk_type, row2.chunk_type);
		assert_eq!(row.chunk_offset, row2.chunk_offset);
		assert_eq!(row.chunk_size, row2.chunk_size);
		assert_eq!(row.position, row2.position);
		assert_eq!(row.size, row2.size);
		assert_eq!(row.name, row2.name);
		assert_eq!(row.children, row2.children);
		assert_eq!(row.preview, row2.preview);
		assert_eq!(buffer, []);
	}

	#[test]
	fn it_parse_partition_table_rows() {
		let rows = vec![
			PartitionTableRow {
				id: Uuid::new_v4(),
				chunk_type: ChunkType::Note,
				chunk_offset: 0,
				chunk_size: 8,
				position: Vec2::new(10., 10.),
				size: Extent2::new(0, 0),
				name: "Foo".into(),
				children: vec![],
				preview: vec![],
			},
			PartitionTableRow {
				id: Uuid::new_v4(),
				chunk_type: ChunkType::Note,
				chunk_offset: 0,
				chunk_size: 8,
				position: Vec2::new(10., 20.),
				size: Extent2::new(0, 0),
				name: "Bar".into(),
				children: vec![],
				preview: vec![],
			},
		];
		let mut writer = BufWriter::new(Cursor::new(Vec::new()));
		let len = rows.write_to(&mut writer).unwrap();
		assert_eq!(len, 122);

		let buffer = writer.buffer();
		let (buffer, rows2) =
			<Vec<PartitionTableRow> as reader::v0::Reader>::from_bytes(buffer).unwrap();
		assert_eq!(rows, rows2);
		assert_eq!(buffer, []);
	}
}
