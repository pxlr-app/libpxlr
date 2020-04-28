use crate::file::reader;
use math::{Extent2, Vec2};
use nom::bytes::complete::take;
use nom::multi::many_m_n;
use nom::number::complete::{le_f32, le_u32, le_u64, le_u8};
use uuid::Uuid;

const MAGIC_NUMBER: &'static str = "PXLR";

#[derive(Debug, PartialEq)]
pub struct Header {
	pub version: u8,
}

impl crate::file::reader::v0::Reader for Header {
	fn from_bytes(bytes: &[u8]) -> nom::IResult<&[u8], Header> {
		let (bytes, _) = nom::bytes::complete::tag(MAGIC_NUMBER)(bytes)?;
		let (bytes, version) = nom::number::complete::le_u8(bytes)?;
		Ok((bytes, Header { version }))
	}
}

impl crate::file::writer::WriteTo for Header {
	fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<usize> {
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
	fn from_bytes(bytes: &[u8]) -> nom::IResult<&[u8], PartitionTable> {
		let (bytes, size) = nom::number::complete::le_u32(bytes)?;
		let (bytes, hash) = <Uuid as reader::v0::Reader>::from_bytes(bytes)?;
		Ok((bytes, PartitionTable { size, hash }))
	}
}

impl crate::file::writer::WriteTo for PartitionTable {
	fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		writer.write(&self.size.to_le_bytes())?;
		self.hash.write_to(writer)?;
		Ok(5)
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

impl crate::file::reader::v0::Reader for ChunkType {
	fn from_bytes(bytes: &[u8]) -> nom::IResult<&[u8], ChunkType> {
		let (bytes, index) = nom::number::complete::le_u16(bytes)?;
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

impl crate::file::writer::WriteTo for ChunkType {
	fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<usize> {
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
	fn from_bytes(bytes: &[u8]) -> nom::IResult<&[u8], PartitionTableRow> {
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

impl crate::file::writer::WriteTo for PartitionTableRow {
	fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<usize> {
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

impl crate::file::reader::v0::Reader for String {
	fn from_bytes(bytes: &[u8]) -> nom::IResult<&[u8], String> {
		let (bytes, len) = le_u32(bytes)?;
		let (bytes, buffer) = take(len as usize)(bytes)?;
		Ok((bytes, std::str::from_utf8(buffer).unwrap().to_owned()))
	}
}

impl crate::file::writer::WriteTo for String {
	fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		let mut b: usize = 0;
		b += writer.write(&(self.len() as u32).to_le_bytes())?;
		b += writer.write(self.as_bytes())?;
		Ok(b)
	}
}

impl crate::file::reader::v0::Reader for Uuid {
	fn from_bytes(bytes: &[u8]) -> nom::IResult<&[u8], Uuid> {
		let (bytes, buffer) = take(16usize)(bytes)?;
		Ok((bytes, Uuid::from_slice(buffer).unwrap()))
	}
}

impl crate::file::writer::WriteTo for Uuid {
	fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		writer.write(self.as_bytes())?;
		Ok(16)
	}
}

impl crate::file::reader::v0::Reader for Vec2<f32> {
	fn from_bytes(bytes: &[u8]) -> nom::IResult<&[u8], Vec2<f32>> {
		let (bytes, x) = le_f32(bytes)?;
		let (bytes, y) = le_f32(bytes)?;
		Ok((bytes, Vec2::new(x, y)))
	}
}

impl crate::file::writer::WriteTo for Vec2<f32> {
	fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		writer.write(&self.x.to_le_bytes())?;
		writer.write(&self.y.to_le_bytes())?;
		Ok(8)
	}
}

impl crate::file::reader::v0::Reader for Extent2<u32> {
	fn from_bytes(bytes: &[u8]) -> nom::IResult<&[u8], Extent2<u32>> {
		let (bytes, w) = le_u32(bytes)?;
		let (bytes, h) = le_u32(bytes)?;
		Ok((bytes, Extent2::new(w, h)))
	}
}

impl crate::file::writer::WriteTo for Extent2<u32> {
	fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<usize> {
		writer.write(&self.w.to_le_bytes())?;
		writer.write(&self.h.to_le_bytes())?;
		Ok(8)
	}
}
