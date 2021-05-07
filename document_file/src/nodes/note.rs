use crate::{Chunk, ChunkDependencies, NodeParse, NodeWrite, Parse, Write};
use document_core::{HasContent, NodeType, Note};
use nom::IResult;
use std::{io, sync::Arc};
use vek::{geom::repr_c::Rect, vec::repr_c::vec2::Vec2};

impl NodeParse for Note {
	fn parse<'bytes>(
		_version: u8,
		chunk: &Chunk,
		_dependencies: ChunkDependencies,
		bytes: &'bytes [u8],
	) -> IResult<&'bytes [u8], Arc<NodeType>> {
		let (bytes, content) = String::parse(&bytes)?;
		Ok((
			bytes,
			Arc::new(NodeType::Note(unsafe {
				Note::construct(
					chunk.id,
					chunk.name.clone(),
					Vec2::new(chunk.rect.x, chunk.rect.y),
					content,
				)
			})),
		))
	}
}

impl NodeWrite for Note {
	fn write<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
	) -> io::Result<(usize, Rect<i32, i32>, ChunkDependencies)> {
		let size = self.content().write(writer)?;
		Ok((size, Rect::default(), ChunkDependencies::default()))
	}
}
