use crate::{
	node::{Node, NodeList, NodeRef},
	parser::{Parse, Result, Write},
};
use math::{Extent2, Vec2};
use nom::{
	multi::many_m_n,
	number::complete::{le_u32, le_u64, le_u8},
};
use std::io;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Index {
	pub hash: Uuid,
	pub size: u32,
	pub prev_offset: u64,
}

impl Parse for Index {
	fn parse(bytes: &[u8]) -> Result<&[u8], Index> {
		let (bytes, prev_offset) = le_u64(bytes)?;
		let (bytes, size) = le_u32(bytes)?;
		let (bytes, hash) = Uuid::parse(bytes)?;
		Ok((
			bytes,
			Index {
				hash,
				size,
				prev_offset,
			},
		))
	}
}

impl Write for Index {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.prev_offset.to_le_bytes())?; // 8
		writer.write_all(&self.size.to_le_bytes())?; // 12
		self.hash.write(writer)?; // 28
		Ok(28)
	}
}

#[derive(Debug, Clone)]
pub struct IndexRow {
	pub id: Uuid,
	pub chunk_type: String,
	pub chunk_offset: u64,
	pub chunk_size: u32,
	pub visible: bool,
	pub locked: bool,
	pub folded: bool,
	pub position: Vec2<u32>,
	pub size: Extent2<u32>,
	pub name: String,
	pub items: Vec<Uuid>,
	pub dependencies: Vec<Uuid>,
	pub preview: Vec<u8>,
}

impl IndexRow {
	pub fn new(id: Uuid) -> IndexRow {
		IndexRow {
			id,
			chunk_type: "".into(),
			chunk_offset: 0,
			chunk_size: 0,
			visible: false,
			locked: false,
			folded: false,
			position: Vec2::new(0, 0),
			size: Extent2::new(0, 0),
			name: String::new(),
			items: Vec::new(),
			dependencies: Vec::new(),
			preview: Vec::new(),
		}
	}
}

impl Parse for IndexRow {
	fn parse(bytes: &[u8]) -> Result<&[u8], IndexRow> {
		let (bytes, id) = Uuid::parse(bytes)?;
		let (bytes, chunk_type) = String::parse(bytes)?;
		let (bytes, chunk_offset) = le_u64(bytes)?;
		let (bytes, chunk_size) = le_u32(bytes)?;
		let (bytes, flag) = le_u8(bytes)?;
		let (bytes, position) = Vec2::<u32>::parse(bytes)?;
		let (bytes, size) = Extent2::<u32>::parse(bytes)?;
		let (bytes, item_count) = le_u32(bytes)?;
		let (bytes, dep_count) = le_u32(bytes)?;
		let (bytes, preview_size) = le_u32(bytes)?;
		let (bytes, name) = String::parse(bytes)?;
		let (bytes, items) =
			many_m_n(item_count as usize, item_count as usize, Uuid::parse)(bytes)?;
		let (bytes, dependencies) =
			many_m_n(dep_count as usize, dep_count as usize, Uuid::parse)(bytes)?;
		let (bytes, preview) =
			many_m_n(preview_size as usize, preview_size as usize, le_u8)(bytes)?;
		Ok((
			bytes,
			IndexRow {
				id,
				chunk_type,
				chunk_offset,
				chunk_size,
				visible: flag & 1 != 0,
				locked: flag & 2 != 0,
				folded: flag & 4 != 0,
				position,
				size,
				name,
				items,
				dependencies,
				preview,
			},
		))
	}
}

impl Write for IndexRow {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		let mut b: usize = 59;
		self.id.write(writer)?; // 16
		self.chunk_type.write(writer)?; // 18
		writer.write_all(&self.chunk_offset.to_le_bytes())?; // 26
		writer.write_all(&self.chunk_size.to_le_bytes())?; // 30
		let flag: u8 =
			(self.visible as u8) << 0 | (self.locked as u8) << 1 | (self.folded as u8) << 2;
		writer.write_all(&flag.to_le_bytes())?; // 31
		self.position.write(writer)?; // 39
		self.size.write(writer)?; // 47
		writer.write_all(&(self.items.len() as u32).to_le_bytes())?; // 51
		writer.write_all(&(self.dependencies.len() as u32).to_le_bytes())?; // 55
		writer.write_all(&(self.preview.len() as u32).to_le_bytes())?; // 59
		b += self.name.write(writer)?;
		for item in self.items.iter() {
			b += item.write(writer)?;
		}
		for dep in self.dependencies.iter() {
			b += dep.write(writer)?;
		}
		writer.write_all(&self.preview)?;
		b += self.preview.len();
		Ok(b)
	}
}

pub trait ParseNode {
	fn parse_node<'bytes>(
		row: &IndexRow,
		items: Vec<NodeRef>,
		dependencies: NodeList,
		bytes: &'bytes [u8],
	) -> Result<&'bytes [u8], Self>
	where
		Self: Sized;
}

pub trait WriteNode {
	fn write_node<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
		rows: &mut Vec<IndexRow>,
		dependencies: &mut Vec<NodeRef>,
	) -> io::Result<usize>;
}
