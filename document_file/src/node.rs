use crate::{Chunk, ChunkDependencies};
use document_core::NodeType;
use nom::{number::complete::le_u16, IResult};
use std::{io, sync::Arc};
use vek::geom::repr_c::Rect;

pub trait NodeParse {
	fn parse<'bytes>(
		version: u8,
		chunk: &Chunk,
		dependencies: ChunkDependencies,
		bytes: &'bytes [u8],
	) -> IResult<&'bytes [u8], Arc<NodeType>>
	where
		Self: Sized;
}

pub trait NodeWrite {
	fn write<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
	) -> io::Result<(usize, Rect<i32, i32>, ChunkDependencies)>;
}

pub(crate) trait NodeTypeNodeId {
	fn node_id(&self) -> u16;
}

impl NodeTypeNodeId for NodeType {
	fn node_id(&self) -> u16 {
		match self {
			NodeType::Group(_) => 0,
			NodeType::Note(_) => 1,
			NodeType::Palette(_) => 1,
		}
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
			0u16 => document_core::Group::parse(version, chunk, dependencies, bytes),
			1u16 => document_core::Note::parse(version, chunk, dependencies, bytes),
			2u16 => document_core::Palette::parse(version, chunk, dependencies, bytes),
			_ => unreachable!(),
		}
	}
}

impl NodeWrite for NodeType {
	fn write<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
	) -> io::Result<(usize, Rect<i32, i32>, ChunkDependencies)> {
		let (size, rect, deps) = match self {
			NodeType::Group(node) => {
				writer.write_all(&0u16.to_le_bytes())?;
				node.write(writer)
			}
			NodeType::Note(node) => {
				writer.write_all(&1u16.to_le_bytes())?;
				node.write(writer)
			}
			NodeType::Palette(node) => {
				writer.write_all(&2u16.to_le_bytes())?;
				node.write(writer)
			}
		}?;
		Ok((size + 2, rect, deps))
	}
}
