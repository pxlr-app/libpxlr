use crate::{Parse, Write};
use async_std::io;
use async_trait::async_trait;
use document_core::NodeType;
use nom::{
	bytes::complete::tag,
	multi::many_m_n,
	number::complete::{le_u16, le_u32, le_u64},
	IResult,
};
use std::sync::Arc;
use uuid::Uuid;
use vek::geom::repr_c::Rect;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Footer {
	pub version: u8,
}

pub const MAGIC_NUMBER: &'static str = "PXLR";

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Index {
	pub hash: Uuid,
	pub root: Uuid,
	pub message_size: u32,
	pub chunks_size: u32,
	pub prev_offset: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
	pub date: u64,
	pub author: String,
	pub message: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Chunk {
	pub id: Uuid,
	pub node_type: u16,
	pub offset: u64,
	pub size: u32,
	pub rect: Rect<i32, i32>,
	pub name: String,
	pub children: Vec<Uuid>,
	pub dependencies: Vec<Uuid>,
}

#[derive(Debug, Clone, Default)]
pub struct ChunkDependencies {
	pub children: Vec<Arc<NodeType>>,
	pub dependencies: Vec<Arc<NodeType>>,
}

impl Default for Index {
	fn default() -> Self {
		Index {
			hash: Uuid::new_v4(),
			root: Uuid::default(),
			message_size: 0,
			chunks_size: 0,
			prev_offset: 0,
		}
	}
}

impl Default for Message {
	fn default() -> Self {
		Message {
			date: chrono::offset::Utc::now().timestamp() as u64,
			author: "".into(),
			message: "".into(),
		}
	}
}

impl Default for Chunk {
	fn default() -> Self {
		Chunk {
			id: Uuid::new_v4(),
			node_type: 0,
			offset: 0,
			size: 0,
			rect: Rect::new(0, 0, 0, 0),
			name: "Chunk".into(),
			children: vec![],
			dependencies: vec![],
		}
	}
}

impl Parse for Footer {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Footer> {
		let (bytes, version) = nom::number::complete::le_u8(bytes)?;
		let (bytes, _) = tag(MAGIC_NUMBER)(bytes)?;
		Ok((bytes, Footer { version }))
	}
}

#[async_trait(?Send)]
impl Write for Footer {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		writer.write(&self.version.to_le_bytes()).await?;
		writer.write(MAGIC_NUMBER.as_bytes()).await?;
		Ok(5)
	}
}

impl Parse for Index {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Index> {
		let (bytes, prev_offset) = le_u64(bytes)?;
		let (bytes, message_size) = le_u32(bytes)?;
		let (bytes, chunks_size) = le_u32(bytes)?;
		let (bytes, root) = Uuid::parse(bytes)?;
		let (bytes, hash) = Uuid::parse(bytes)?;
		Ok((
			bytes,
			Index {
				hash,
				root,
				message_size,
				chunks_size,
				prev_offset,
			},
		))
	}
}

#[async_trait(?Send)]
impl Write for Index {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		writer.write(&self.prev_offset.to_le_bytes()).await?;
		writer.write(&self.message_size.to_le_bytes()).await?;
		writer.write(&self.chunks_size.to_le_bytes()).await?;
		self.root.write(writer).await?;
		self.hash.write(writer).await?;
		Ok(48)
	}
}

impl Parse for Message {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Message> {
		let (bytes, date) = le_u64(bytes)?;
		let (bytes, author) = String::parse(bytes)?;
		let (bytes, message) = String::parse(bytes)?;
		Ok((
			bytes,
			Message {
				date,
				author,
				message,
			},
		))
	}
}

#[async_trait(?Send)]
impl Write for Message {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		let mut size = 8;
		writer.write(&self.date.to_le_bytes()).await?;
		size += self.author.write(writer).await?;
		size += self.message.write(writer).await?;
		Ok(size)
	}
}

impl Parse for Chunk {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Chunk> {
		let (bytes, id) = Uuid::parse(bytes)?;
		let (bytes, node) = le_u16(bytes)?;
		let (bytes, offset) = le_u64(bytes)?;
		let (bytes, size) = le_u32(bytes)?;
		let (bytes, rect) = Rect::<i32, i32>::parse(bytes)?;
		let (bytes, child_count) = le_u32(bytes)?;
		let (bytes, dep_count) = le_u32(bytes)?;
		let (bytes, name) = String::parse(bytes)?;
		let (bytes, children) =
			many_m_n(child_count as usize, child_count as usize, Uuid::parse)(bytes)?;
		let (bytes, dependencies) =
			many_m_n(dep_count as usize, dep_count as usize, Uuid::parse)(bytes)?;
		Ok((
			bytes,
			Chunk {
				id,
				node_type: node,
				offset,
				size,
				rect,
				name,
				children,
				dependencies,
			},
		))
	}
}

#[async_trait(?Send)]
impl Write for Chunk {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		let mut b: usize = 54;
		self.id.write(writer).await?;
		writer.write(&self.node_type.to_le_bytes()).await?;
		writer.write(&self.offset.to_le_bytes()).await?;
		writer.write(&self.size.to_le_bytes()).await?;
		self.rect.write(writer).await?;
		writer
			.write(&(self.children.len() as u32).to_le_bytes())
			.await?;
		writer
			.write(&(self.dependencies.len() as u32).to_le_bytes())
			.await?;
		b += self.name.write(writer).await?;
		for item in self.children.iter() {
			b += item.write(writer).await?;
		}
		for dep in self.dependencies.iter() {
			b += dep.write(writer).await?;
		}
		Ok(b)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use async_std::task;

	#[test]
	fn footer_parse() {
		let footer = Footer { version: 1 };
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(footer.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, footer2) = Footer::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(footer2, footer);
	}

	#[test]
	fn index_parse() {
		let index = Index {
			hash: Uuid::parse_str("68204970-a53a-4eb5-bee4-93e3fd19e8de").unwrap(),
			root: Uuid::parse_str("4a89c955-54fe-4a48-b367-378a8a47ab34").unwrap(),
			chunks_size: 1,
			message_size: 2,
			prev_offset: 3,
		};
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(index.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, index2) = Index::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(index2, index);
	}

	#[test]
	fn message_parse() {
		let message = Message {
			author: "Michael Grenier <michael@grenier.dev>".into(),
			message: "My first message".into(),
			..Message::default()
		};
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(message.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, message2) = Message::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(message2, message);
	}

	#[test]
	fn chunk_parse() {
		let chunk = Chunk {
			id: Uuid::parse_str("ac16bacf-9a95-413e-b2f4-fcf94274ad62").unwrap(),
			node_type: 1,
			offset: 2,
			size: 3,
			rect: Rect::new(4, 5, 6, 7),
			name: "Chunk".into(),
			children: vec![
				Uuid::parse_str("291666d7-e9e2-4401-8e7b-c3177a2f8536").unwrap(),
				Uuid::parse_str("5aed490e-e4f0-4a18-94ed-01472f8d52a7").unwrap(),
			],
			dependencies: vec![Uuid::parse_str("b1e02af1-468b-4a94-b80f-7050874b39ef").unwrap()],
		};
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(chunk.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, chunk2) = Chunk::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(chunk2, chunk);
	}
}
