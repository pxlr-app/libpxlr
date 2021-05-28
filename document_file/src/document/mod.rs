use crate::File;
use crate::{Chunk, ChunkDependencies, NodeParse, NodeWrite, Parse, Write};
use async_std::io;
use async_std::io::prelude::WriteExt;
use async_trait::async_trait;
use document_core::NodeType;
use nom::bytes::complete::take;
use nom::number::complete::le_u32;
use nom::{number::complete::le_u16, IResult};
use std::sync::Arc;

mod canvas;
mod group;
mod note;
mod palette;
mod unloaded;

pub use self::canvas::*;
pub use self::group::*;
pub use self::note::*;
pub use self::palette::*;
pub use self::unloaded::*;

pub trait NodeId {
	fn node_id(&self) -> u16;
}

impl NodeId for NodeType {
	fn node_id(&self) -> u16 {
		match self {
			NodeType::Unloaded(_) => 0,
			NodeType::Group(_) => 1,
			NodeType::Note(_) => 2,
			NodeType::Palette(_) => 3,
			NodeType::CanvasGroup(_) => 4,
		}
	}
}

impl Parse for Arc<NodeType> {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Arc<NodeType>> {
		let (bytes, size) = le_u32(bytes)?;
		let (bytes, buffer) = take(size as usize)(bytes)?;
		let mut buffer = async_std::io::Cursor::new(buffer);
		let file = async_std::task::block_on(File::read(&mut buffer)).unwrap();
		let root = async_std::task::block_on(file.get_root_node(&mut buffer, false)).unwrap();
		Ok((bytes, root))
	}
}

#[async_trait(?Send)]
impl Write for Arc<NodeType> {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		let mut file = File::default();
		let mut buffer = async_std::io::Cursor::new(Vec::new());
		file.set_root_node(self.clone());
		let size = file.append(&mut buffer).await.unwrap();
		writer.write_all(&(size as u32).to_le_bytes()).await?;
		writer.write_all(buffer.get_ref()).await?;
		Ok(size + 4)
	}
}

impl NodeParse for NodeType {
	fn parse<'bytes>(
		version: u8,
		chunk: &Chunk,
		dependencies: ChunkDependencies,
		bytes: &'bytes [u8],
	) -> IResult<&'bytes [u8], Arc<NodeType>> {
		let (bytes, node_type) = le_u16(bytes)?;
		match node_type {
			0u16 => document_core::Unloaded::parse(version, chunk, dependencies, bytes),
			1u16 => document_core::Group::parse(version, chunk, dependencies, bytes),
			2u16 => document_core::Note::parse(version, chunk, dependencies, bytes),
			3u16 => document_core::Palette::parse(version, chunk, dependencies, bytes),
			4u16 => document_core::CanvasGroup::parse(version, chunk, dependencies, bytes),
			_ => unreachable!(),
		}
	}
}

#[async_trait(?Send)]
impl NodeWrite for NodeType {
	async fn write<W: io::Write + std::marker::Unpin>(
		&self,
		writer: &mut W,
	) -> io::Result<(usize, ChunkDependencies)> {
		use async_std::io::prelude::WriteExt;
		let (size, deps) = match self {
			NodeType::Unloaded(node) => {
				writer.write(&0u16.to_le_bytes()).await?;
				node.write(writer).await
			}
			NodeType::Group(node) => {
				writer.write(&1u16.to_le_bytes()).await?;
				node.write(writer).await
			}
			NodeType::Note(node) => {
				writer.write(&2u16.to_le_bytes()).await?;
				node.write(writer).await
			}
			NodeType::Palette(node) => {
				writer.write(&3u16.to_le_bytes()).await?;
				node.write(writer).await
			}
			NodeType::CanvasGroup(node) => {
				writer.write(&4u16.to_le_bytes()).await?;
				node.write(writer).await
			}
		}?;
		Ok((size + 2, deps))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use async_std::task;
	use document_core::Note;

	#[test]
	fn nodetype_parse() {
		let node = Arc::new(NodeType::Note(Note::new("My note", (0, 0), "")));
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(node.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, node2) =
			<Arc<NodeType> as Parse>::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(node2, node);
	}
}
