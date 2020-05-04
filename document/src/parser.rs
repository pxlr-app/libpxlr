use math::{Extent2, Vec2};
use nom::bytes::complete::{tag, take};
use nom::number::complete::{le_f32, le_u32, le_u8};
use nom::IResult;
use std::io;
use uuid::Uuid;

const MAGIC_NUMBER: &'static str = "PXLR";

pub trait IParser {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self>
	where
		Self: Sized;

	fn write<S>(&self, storage: &mut S) -> io::Result<usize>
	where
		S: io::Write + io::Seek;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
	pub version: u8,
}

impl IParser for Header {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, _) = tag(MAGIC_NUMBER)(bytes)?;
		let (bytes, version) = le_u8(bytes)?;
		Ok((bytes, Header { version }))
	}

	fn write<S>(&self, storage: &mut S) -> io::Result<usize>
	where
		S: io::Write + io::Seek,
	{
		storage.write(MAGIC_NUMBER.as_bytes())?;
		storage.write(&self.version.to_le_bytes())?;
		Ok(5)
	}
}

pub mod v0 {
	use math::{Extent2, Vec2};
	use nom::multi::many_m_n;
	use nom::number::complete::{le_u16, le_u32, le_u64, le_u8};
	use nom::IResult;
	use std::collections::HashMap;
	use std::io;
	use uuid::Uuid;

	#[derive(Debug, Clone, PartialEq)]
	pub struct PartitionIndex {
		pub table: PartitionTable,
		pub rows: Vec<PartitionTableRow>,
		pub index_uuid: HashMap<Uuid, usize>,
	}

	impl PartitionIndex {
		pub fn new(table: PartitionTable, rows: Vec<PartitionTableRow>) -> Self {
			let mut index_uuid: HashMap<Uuid, usize> = HashMap::new();

			for (i, row) in rows.iter().enumerate() {
				index_uuid.insert(row.id, i);
			}

			PartitionIndex {
				table,
				rows,
				index_uuid,
			}
		}
	}

	pub trait IParser {
		type Output;

		fn parse<'b, S>(
			index: &PartitionIndex,
			row: &PartitionTableRow,
			storage: &mut S,
			bytes: &'b [u8],
		) -> IResult<&'b [u8], Self::Output>
		where
			S: io::Read + io::Seek,
			Self::Output: Sized;

		fn write<S>(&self, index: &mut PartitionIndex, storage: &mut S) -> io::Result<usize>
		where
			S: io::Write + io::Seek;
	}

	impl<T> IParser for Vec<T>
	where
		T: IParser,
	{
		type Output = Vec<T::Output>;

		fn parse<'b, S>(
			index: &PartitionIndex,
			row: &PartitionTableRow,
			storage: &mut S,
			bytes: &'b [u8],
		) -> IResult<&'b [u8], Self::Output>
		where
			S: io::Read + io::Seek,
		{
			let mut items: Vec<T::Output> = Vec::new();
			let mut remainder: &'b [u8] = bytes;
			loop {
				if let Ok((b, item)) = <T as IParser>::parse(index, row, storage, remainder) {
					remainder = b;
					items.push(item);
				} else {
					break;
				}
			}
			Ok((remainder, items))
		}

		fn write<S>(&self, index: &mut PartitionIndex, storage: &mut S) -> io::Result<usize>
		where
			S: io::Write + io::Seek,
		{
			let mut b: usize = 0;
			for item in self.iter() {
				b += item.write(index, storage)?;
			}
			Ok(b)
		}
	}

	#[derive(Debug, Clone, PartialEq)]
	pub struct PartitionTable {
		pub hash: Uuid,
		pub size: u32,
	}

	impl super::IParser for PartitionTable {
		fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
			let (bytes, size) = le_u32(bytes)?;
			let (bytes, hash) = Uuid::parse(bytes)?;
			Ok((bytes, PartitionTable { size, hash }))
		}

		fn write<S>(&self, storage: &mut S) -> io::Result<usize>
		where
			S: io::Write + io::Seek,
		{
			storage.write(&self.size.to_le_bytes())?;
			self.hash.write(storage)?;
			Ok(20)
		}
	}

	#[derive(Debug, Clone, PartialEq)]
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

	impl super::IParser for ChunkType {
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

		fn write<S>(&self, storage: &mut S) -> io::Result<usize>
		where
			S: io::Write + io::Seek,
		{
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
			storage.write(&index.to_le_bytes())?;
			Ok(2)
		}
	}

	#[derive(Debug, Clone, PartialEq)]
	pub struct PartitionTableRow {
		pub id: Uuid,
		pub chunk_type: ChunkType,
		pub chunk_offset: u64,
		pub chunk_size: u32,
		pub is_root: bool,
		pub is_visible: bool,
		pub is_locked: bool,
		pub is_folded: bool,
		pub position: Vec2<f32>,
		pub size: Extent2<u32>,
		pub name: String,
		pub children: Vec<u32>,
		pub preview: Vec<u8>,
	}

	impl super::IParser for PartitionTableRow {
		fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
			let (bytes, id) = Uuid::parse(bytes)?;
			let (bytes, chunk_type) = <ChunkType as super::IParser>::parse(bytes)?;
			let (bytes, chunk_offset) = le_u64(bytes)?;
			let (bytes, chunk_size) = le_u32(bytes)?;
			let (bytes, flag) = le_u8(bytes)?;
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
					is_root: flag | 1 > 0,
					is_visible: flag | 2 > 0,
					is_locked: flag | 4 > 0,
					is_folded: flag | 8 > 0,
					position,
					size,
					name,
					children,
					preview,
				},
			))
		}

		fn write<S>(&self, storage: &mut S) -> io::Result<usize>
		where
			S: io::Write + io::Seek,
		{
			let mut b: usize = 55;
			self.id.write(storage)?;
			self.chunk_type.write(storage)?;
			storage.write(&self.chunk_offset.to_le_bytes())?;
			storage.write(&self.chunk_size.to_le_bytes())?;
			let flag: u8 = (self.is_root as u8) << 0
				| (self.is_visible as u8) << 1
				| (self.is_locked as u8) << 2
				| (self.is_folded as u8) << 3;
			storage.write(&flag.to_le_bytes())?;
			self.position.write(storage)?;
			self.size.write(storage)?;
			storage.write(&(self.children.len() as u32).to_le_bytes())?;
			storage.write(&(self.preview.len() as u32).to_le_bytes())?;
			b += self.name.write(storage)?;
			for child in self.children.iter() {
				b += storage.write(&child.to_le_bytes())?;
			}
			b += storage.write(&self.preview)?;
			Ok(b)
		}
	}
}

impl IParser for String {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, len) = le_u32(bytes)?;
		let (bytes, buffer) = take(len as usize)(bytes)?;
		Ok((bytes, std::str::from_utf8(buffer).unwrap().to_owned()))
	}

	fn write<S>(&self, storage: &mut S) -> io::Result<usize>
	where
		S: io::Write + io::Seek,
	{
		let mut b: usize = 0;
		b += storage.write(&(self.len() as u32).to_le_bytes())?;
		b += storage.write(self.as_bytes())?;
		Ok(b)
	}
}

impl IParser for Uuid {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, buffer) = take(16usize)(bytes)?;
		Ok((bytes, Uuid::from_slice(buffer).unwrap()))
	}

	fn write<S>(&self, storage: &mut S) -> io::Result<usize>
	where
		S: io::Write + io::Seek,
	{
		storage.write(self.as_bytes())?;
		Ok(16)
	}
}

impl IParser for Vec2<f32> {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, x) = le_f32(bytes)?;
		let (bytes, y) = le_f32(bytes)?;
		Ok((bytes, Vec2::new(x, y)))
	}

	fn write<S>(&self, storage: &mut S) -> io::Result<usize>
	where
		S: io::Write + io::Seek,
	{
		storage.write(&self.x.to_le_bytes())?;
		storage.write(&self.y.to_le_bytes())?;
		Ok(8)
	}
}

impl IParser for Extent2<u32> {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, w) = le_u32(bytes)?;
		let (bytes, h) = le_u32(bytes)?;
		Ok((bytes, Extent2::new(w, h)))
	}

	fn write<S>(&self, storage: &mut S) -> io::Result<usize>
	where
		S: io::Write + io::Seek,
	{
		storage.write(&self.w.to_le_bytes())?;
		storage.write(&self.h.to_le_bytes())?;
		Ok(8)
	}
}
