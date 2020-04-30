use async_std::io;
use async_std::io::prelude::*;
use async_trait::async_trait;
use math::{Extent2, Vec2};
use nom::bytes::complete::{tag, take};
use nom::number::complete::{le_f32, le_u32, le_u8};
use nom::IResult;
use uuid::Uuid;

const MAGIC_NUMBER: &'static str = "PXLR";

#[async_trait(?Send)]
pub trait Parser {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self>
	where
		Self: Sized;

	async fn write<S: io::Read + io::Write + io::Seek + std::marker::Unpin>(
		&self,
		storage: &mut S,
	) -> io::Result<usize>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
	pub version: u8,
}

#[async_trait(?Send)]
impl Parser for Header {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, _) = tag(MAGIC_NUMBER)(bytes)?;
		let (bytes, version) = le_u8(bytes)?;
		Ok((bytes, Header { version }))
	}

	async fn write<S: io::Read + io::Write + io::Seek + std::marker::Unpin>(
		&self,
		storage: &mut S,
	) -> io::Result<usize> {
		storage.write(MAGIC_NUMBER.as_bytes()).await?;
		storage.write(&self.version.to_le_bytes()).await?;
		Ok(5)
	}
}

pub mod v0 {
	use async_std::io;
	use async_std::io::prelude::*;
	use async_trait::async_trait;
	use math::{Extent2, Vec2};
	use nom::multi::many_m_n;
	use nom::number::complete::{le_u16, le_u32, le_u64, le_u8};
	use nom::IResult;
	use std::collections::HashMap;
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

		// pub fn read_chunk(&mut self, index: usize) -> io::Result<Vec<u8>> {
		// 	let row = self.rows.get(index).expect("Row not found.");
		// 	let offset = row.chunk_offset;
		// 	let size = row.chunk_size as usize;
		// 	self.reader.seek(io::SeekFrom::Start(offset))?;
		// 	let mut bytes: Vec<u8> = Vec::with_capacity(size);
		// 	self.reader.read(&mut bytes)?;
		// 	Ok(bytes)
		// }

		// pub fn read_root(&mut self) -> io::Result<Node> {
		// 	let bytes = self.read_chunk(0)?;
		// 	let root_chunk = self.rows.get(0).expect("No root chunk.");
		// 	let (_, node) =
		// 		<Node as self::PartitionTableParse>::parse(self, root_chunk, &bytes[..])
		// 			.expect("Expected node.");
		// 	Ok(node)
		// }
	}

	#[async_trait(?Send)]
	pub trait PartitionTableParse<S>
	where
		S: io::Read + io::Write + io::Seek + std::marker::Unpin,
	{
		type Output;

		async fn parse<'b>(
			index: &PartitionIndex,
			row: &PartitionTableRow,
			storage: &mut S,
			bytes: &'b [u8],
		) -> IResult<&'b [u8], Self::Output>
		where
			Self::Output: Sized;

		async fn write(&self, index: &mut PartitionIndex, storage: &mut S) -> io::Result<usize>;
	}

	#[async_trait(?Send)]
	impl<T, S> PartitionTableParse<S> for Vec<T>
	where
		S: io::Read + io::Write + io::Seek + std::marker::Unpin,
		T: PartitionTableParse<S>,
	{
		type Output = Vec<T::Output>;

		async fn parse<'b>(
			index: &PartitionIndex,
			row: &PartitionTableRow,
			storage: &mut S,
			bytes: &'b [u8],
		) -> IResult<&'b [u8], Self::Output> {
			let mut items: Vec<T::Output> = Vec::new();
			let mut remainder: &'b [u8] = bytes;
			loop {
				if let Ok((b, item)) =
					<T as PartitionTableParse<S>>::parse(index, row, storage, remainder).await
				{
					remainder = b;
					items.push(item);
				} else {
					break;
				}
			}
			Ok((remainder, items))
		}

		async fn write(&self, index: &mut PartitionIndex, storage: &mut S) -> io::Result<usize> {
			let mut b: usize = 0;
			for item in self.iter() {
				b += item.write(index, storage).await?;
			}
			Ok(b)
		}
	}

	#[derive(Debug, Clone, PartialEq)]
	pub struct PartitionTable {
		pub hash: Uuid,
		pub size: u32,
	}

	#[async_trait(?Send)]
	impl super::Parser for PartitionTable {
		fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
			let (bytes, size) = le_u32(bytes)?;
			let (bytes, hash) = Uuid::parse(bytes)?;
			Ok((bytes, PartitionTable { size, hash }))
		}

		async fn write<S: io::Read + io::Write + io::Seek + std::marker::Unpin>(
			&self,
			storage: &mut S,
		) -> io::Result<usize> {
			storage.write(&self.size.to_le_bytes()).await?;
			self.hash.write(storage).await?;
			Ok(5)
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

	#[async_trait(?Send)]
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

		async fn write<S: io::Read + io::Write + io::Seek + std::marker::Unpin>(
			&self,
			storage: &mut S,
		) -> io::Result<usize> {
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
			storage.write(&index.to_le_bytes()).await?;
			Ok(2)
		}
	}

	#[derive(Debug, Clone, PartialEq)]
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

	#[async_trait(?Send)]
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

		async fn write<S: io::Read + io::Write + io::Seek + std::marker::Unpin>(
			&self,
			storage: &mut S,
		) -> io::Result<usize> {
			let mut b: usize = 54;
			self.id.write(storage).await?;
			self.chunk_type.write(storage).await?;
			storage.write(&self.chunk_offset.to_le_bytes()).await?;
			storage.write(&self.chunk_size.to_le_bytes()).await?;
			self.position.write(storage).await?;
			self.size.write(storage).await?;
			storage
				.write(&(self.children.len() as u32).to_le_bytes())
				.await?;
			storage
				.write(&(self.preview.len() as u32).to_le_bytes())
				.await?;
			b += self.name.write(storage).await?;
			for child in self.children.iter() {
				b += storage.write(&child.to_le_bytes()).await?;
			}
			b += storage.write(&self.preview).await?;
			Ok(b)
		}
	}
}

#[async_trait(?Send)]
impl Parser for String {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, len) = le_u32(bytes)?;
		let (bytes, buffer) = take(len as usize)(bytes)?;
		Ok((bytes, std::str::from_utf8(buffer).unwrap().to_owned()))
	}

	async fn write<S: io::Read + io::Write + io::Seek + std::marker::Unpin>(
		&self,
		storage: &mut S,
	) -> io::Result<usize> {
		let mut b: usize = 0;
		b += storage.write(&(self.len() as u32).to_le_bytes()).await?;
		b += storage.write(self.as_bytes()).await?;
		Ok(b)
	}
}

#[async_trait(?Send)]
impl Parser for Uuid {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, buffer) = take(16usize)(bytes)?;
		Ok((bytes, Uuid::from_slice(buffer).unwrap()))
	}

	async fn write<S: io::Read + io::Write + io::Seek + std::marker::Unpin>(
		&self,
		storage: &mut S,
	) -> io::Result<usize> {
		storage.write(self.as_bytes()).await?;
		Ok(16)
	}
}

#[async_trait(?Send)]
impl Parser for Vec2<f32> {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, x) = le_f32(bytes)?;
		let (bytes, y) = le_f32(bytes)?;
		Ok((bytes, Vec2::new(x, y)))
	}

	async fn write<S: io::Read + io::Write + io::Seek + std::marker::Unpin>(
		&self,
		storage: &mut S,
	) -> io::Result<usize> {
		storage.write(&self.x.to_le_bytes()).await?;
		storage.write(&self.y.to_le_bytes()).await?;
		Ok(8)
	}
}

#[async_trait(?Send)]
impl Parser for Extent2<u32> {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, w) = le_u32(bytes)?;
		let (bytes, h) = le_u32(bytes)?;
		Ok((bytes, Extent2::new(w, h)))
	}

	async fn write<S: io::Read + io::Write + io::Seek + std::marker::Unpin>(
		&self,
		storage: &mut S,
	) -> io::Result<usize> {
		storage.write(&self.w.to_le_bytes()).await?;
		storage.write(&self.h.to_le_bytes()).await?;
		Ok(8)
	}
}
