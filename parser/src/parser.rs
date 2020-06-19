use async_trait::async_trait;
use futures::io;
use math::{Extent2, Vec2};
use nom::bytes::complete::{tag, take};
use nom::number::complete::{le_f32, le_u32, le_u8};
use nom::IResult;
use uuid::Uuid;

const MAGIC_NUMBER: &'static str = "PXLR";

#[async_trait]
pub trait IParser {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self>
	where
		Self: Sized;
}

#[async_trait]
pub trait IWriter {
	async fn write<S>(&self, storage: &mut S) -> io::Result<usize>
	where
		S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
	pub version: u8,
}

#[async_trait]
impl IParser for Header {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, _) = tag(MAGIC_NUMBER)(bytes)?;
		let (bytes, version) = le_u8(bytes)?;
		Ok((bytes, Header { version }))
	}
}

#[async_trait]
impl IWriter for Header {
	async fn write<S>(&self, storage: &mut S) -> io::Result<usize>
	where
		S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
	{
		storage.write(MAGIC_NUMBER.as_bytes()).await?;
		storage.write(&self.version.to_le_bytes()).await?;
		Ok(5)
	}
}

pub mod v0 {
	use async_trait::async_trait;
	use document::Node;
	use futures::io;
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

		pub fn reindex_rows(&mut self) {
			self.index_uuid.clear();
			for (i, row) in self.rows.iter().enumerate() {
				self.index_uuid.insert(row.id, i);
			}
		}
	}

	#[async_trait]
	pub trait IParser {
		type Output: std::marker::Send;

		async fn parse<'b>(
			row: &PartitionTableRow,
			children: &mut Vec<Node>,
			bytes: &'b [u8],
		) -> IResult<&'b [u8], Self::Output>
		where
			Self::Output: Sized;
	}

	#[async_trait]
	pub trait IWriter {
		async fn write<S>(
			&self,
			index: &mut PartitionIndex,
			storage: &mut S,
			offset: u64,
		) -> io::Result<usize>
		where
			S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin;
	}

	#[derive(Debug, Clone, PartialEq)]
	pub struct PartitionTable {
		pub hash: Uuid,
		pub size: u32,
		pub root_child: u32,
	}

	#[async_trait]
	impl super::IParser for PartitionTable {
		fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
			let (bytes, root_child) = le_u32(bytes)?;
			let (bytes, size) = le_u32(bytes)?;
			let (bytes, hash) = Uuid::parse(bytes)?;
			Ok((
				bytes,
				PartitionTable {
					size,
					hash,
					root_child,
				},
			))
		}
	}

	#[async_trait]
	impl super::IWriter for PartitionTable {
		async fn write<S>(&self, storage: &mut S) -> io::Result<usize>
		where
			S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
		{
			storage.write_all(&self.root_child.to_le_bytes()).await?;
			storage.write_all(&self.size.to_le_bytes()).await?;
			self.hash.write(storage).await?;
			Ok(24)
		}
	}

	#[derive(Debug, Clone, PartialEq)]
	pub enum ChunkType {
		Unknown,
		Group,
		Note,
		LayerGroup,
		CanvasGrey,
		CanvasRGBA,
		CanvasUV,
	}

	#[async_trait]
	impl super::IParser for ChunkType {
		fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
			let (bytes, index) = le_u16(bytes)?;
			let value = match index {
				0 => ChunkType::Unknown,
				1 => ChunkType::Group,
				2 => ChunkType::Note,
				3 => ChunkType::LayerGroup,
				4 => ChunkType::CanvasGrey,
				5 => ChunkType::CanvasRGBA,
				6 => ChunkType::CanvasUV,
				_ => panic!("Unknown chunk type"),
			};
			Ok((bytes, value))
		}
	}

	#[async_trait]
	impl super::IWriter for ChunkType {
		async fn write<S>(&self, storage: &mut S) -> io::Result<usize>
		where
			S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
		{
			let index: u16 = match self {
				ChunkType::Unknown => 0,
				ChunkType::Group => 1,
				ChunkType::Note => 2,
				ChunkType::LayerGroup => 3,
				ChunkType::CanvasGrey => 4,
				ChunkType::CanvasRGBA => 5,
				ChunkType::CanvasUV => 6,
			};
			storage.write_all(&index.to_le_bytes()).await?;
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

	#[async_trait]
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
	}

	#[async_trait]
	impl super::IWriter for PartitionTableRow {
		async fn write<S>(&self, storage: &mut S) -> io::Result<usize>
		where
			S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
		{
			let mut b: usize = 55;
			self.id.write(storage).await?;
			self.chunk_type.write(storage).await?;
			storage.write_all(&self.chunk_offset.to_le_bytes()).await?;
			storage.write_all(&self.chunk_size.to_le_bytes()).await?;
			let flag: u8 = (self.is_root as u8) << 0
				| (self.is_visible as u8) << 1
				| (self.is_locked as u8) << 2
				| (self.is_folded as u8) << 3;
			storage.write_all(&flag.to_le_bytes()).await?;
			self.position.write(storage).await?;
			self.size.write(storage).await?;
			storage
				.write_all(&(self.children.len() as u32).to_le_bytes())
				.await?;
			storage
				.write_all(&(self.preview.len() as u32).to_le_bytes())
				.await?;
			b += self.name.write(storage).await?;
			for child in self.children.iter() {
				storage.write_all(&child.to_le_bytes()).await?;
				b += 4;
			}
			storage.write_all(&self.preview).await?;
			b += self.preview.len();
			Ok(b)
		}
	}
}

#[async_trait]
impl IParser for String {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, len) = le_u32(bytes)?;
		let (bytes, buffer) = take(len as usize)(bytes)?;
		Ok((bytes, std::str::from_utf8(buffer).unwrap().to_owned()))
	}
}

#[async_trait]
impl IWriter for String {
	async fn write<S>(&self, storage: &mut S) -> io::Result<usize>
	where
		S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
	{
		let mut b: usize = 4;
		storage
			.write_all(&(self.len() as u32).to_le_bytes())
			.await?;
		let buf = self.as_bytes();
		storage.write_all(buf).await?;
		b += buf.len();
		Ok(b)
	}
}

#[async_trait]
impl IParser for Uuid {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, buffer) = take(16usize)(bytes)?;
		Ok((bytes, Uuid::from_slice(buffer).unwrap()))
	}
}

#[async_trait]
impl IWriter for Uuid {
	async fn write<S>(&self, storage: &mut S) -> io::Result<usize>
	where
		S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
	{
		storage.write_all(self.as_bytes()).await?;
		Ok(16)
	}
}

#[async_trait]
impl IParser for Vec2<f32> {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, x) = le_f32(bytes)?;
		let (bytes, y) = le_f32(bytes)?;
		Ok((bytes, Vec2::new(x, y)))
	}
}

#[async_trait]
impl IWriter for Vec2<f32> {
	async fn write<S>(&self, storage: &mut S) -> io::Result<usize>
	where
		S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
	{
		storage.write_all(&self.x.to_le_bytes()).await?;
		storage.write_all(&self.y.to_le_bytes()).await?;
		Ok(8)
	}
}

#[async_trait]
impl IParser for Extent2<u32> {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
		let (bytes, w) = le_u32(bytes)?;
		let (bytes, h) = le_u32(bytes)?;
		Ok((bytes, Extent2::new(w, h)))
	}
}

#[async_trait]
impl IWriter for Extent2<u32> {
	async fn write<S>(&self, storage: &mut S) -> io::Result<usize>
	where
		S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
	{
		storage.write_all(&self.w.to_le_bytes()).await?;
		storage.write_all(&self.h.to_le_bytes()).await?;
		Ok(8)
	}
}
