use crate::{
	node::{NodeKind, NodeList, NodeRef},
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
	pub root: Uuid,
	pub size: u32,
	pub prev_offset: u64,
}

impl Parse for Index {
	fn parse(bytes: &[u8]) -> Result<&[u8], Index> {
		let (bytes, prev_offset) = le_u64(bytes)?;
		let (bytes, size) = le_u32(bytes)?;
		let (bytes, root) = Uuid::parse(bytes)?;
		let (bytes, hash) = Uuid::parse(bytes)?;
		Ok((
			bytes,
			Index {
				hash,
				root,
				size,
				prev_offset,
			},
		))
	}
}

impl Write for Index {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.prev_offset.to_le_bytes())?; // 8
		writer.write_all(&self.size.to_le_bytes())?; // 4
		self.root.write(writer)?; // 16
		self.hash.write(writer)?; // 16
		Ok(44)
	}
}

#[derive(Debug, Clone)]
pub struct IndexRow {
	pub id: Uuid,
	pub chunk_type: NodeKind,
	pub chunk_offset: u64,
	pub chunk_size: u32,
	pub display: bool,
	pub locked: bool,
	pub folded: bool,
	pub position: Vec2<u32>,
	pub size: Extent2<u32>,
	pub name: String,
	pub children: Vec<Uuid>,
	pub dependencies: Vec<Uuid>,
}

impl IndexRow {
	pub fn new(id: Uuid) -> IndexRow {
		IndexRow {
			id,
			chunk_type: NodeKind::Group,
			chunk_offset: 0,
			chunk_size: 0,
			display: false,
			locked: false,
			folded: false,
			position: Vec2::new(0, 0),
			size: Extent2::new(0, 0),
			name: String::new(),
			children: Vec::new(),
			dependencies: Vec::new(),
		}
	}
}

impl Parse for IndexRow {
	fn parse(bytes: &[u8]) -> Result<&[u8], IndexRow> {
		let (bytes, id) = Uuid::parse(bytes)?;
		let (bytes, chunk_type) = NodeKind::parse(bytes)?;
		let (bytes, chunk_offset) = le_u64(bytes)?;
		let (bytes, chunk_size) = le_u32(bytes)?;
		let (bytes, flag) = le_u8(bytes)?;
		let (bytes, position) = Vec2::<u32>::parse(bytes)?;
		let (bytes, size) = Extent2::<u32>::parse(bytes)?;
		let (bytes, item_count) = le_u32(bytes)?;
		let (bytes, dep_count) = le_u32(bytes)?;
		let (bytes, name) = String::parse(bytes)?;
		let (bytes, children) =
			many_m_n(item_count as usize, item_count as usize, Uuid::parse)(bytes)?;
		let (bytes, dependencies) =
			many_m_n(dep_count as usize, dep_count as usize, Uuid::parse)(bytes)?;
		Ok((
			bytes,
			IndexRow {
				id,
				chunk_type,
				chunk_offset,
				chunk_size,
				display: flag & 1 != 0,
				locked: flag & 2 != 0,
				folded: flag & 4 != 0,
				position,
				size,
				name,
				children,
				dependencies,
			},
		))
	}
}

impl Write for IndexRow {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		let mut b: usize = 55;
		self.id.write(writer)?; // 16
		self.chunk_type.write(writer)?; // 2
		writer.write_all(&self.chunk_offset.to_le_bytes())?; // 8
		writer.write_all(&self.chunk_size.to_le_bytes())?; // 4
		let flag: u8 =
			(self.display as u8) << 0 | (self.locked as u8) << 1 | (self.folded as u8) << 2;
		writer.write_all(&flag.to_le_bytes())?; // 1
		self.position.write(writer)?; // 8
		self.size.write(writer)?; // 8
		writer.write_all(&(self.children.len() as u32).to_le_bytes())?; // 4
		writer.write_all(&(self.dependencies.len() as u32).to_le_bytes())?; // 4
		b += self.name.write(writer)?;
		for item in self.children.iter() {
			b += item.write(writer)?;
		}
		for dep in self.dependencies.iter() {
			b += dep.write(writer)?;
		}
		Ok(b)
	}
}

pub trait ParseNode {
	fn parse_node<'bytes>(
		row: &IndexRow,
		children: NodeList,
		dependencies: NodeList,
		bytes: &'bytes [u8],
	) -> Result<&'bytes [u8], NodeRef>
	where
		Self: Sized;
}

pub type ParseFn<'b> = fn(&IndexRow, NodeList, NodeList, &'b [u8]) -> Result<&'b [u8], NodeRef>;

pub trait WriteNode {
	fn write_node<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
		rows: &mut Vec<IndexRow>,
		dependencies: &mut Vec<NodeRef>,
	) -> io::Result<usize>;
}
