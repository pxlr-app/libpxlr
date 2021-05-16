use crate::{Chunk, ChunkDependencies, NodeParse, NodeWrite, Parse, Write};
use async_std::io;
use async_trait::async_trait;
use document_core::{HasContent, NodeType, Note};
use nom::IResult;
use std::sync::Arc;
use vek::vec::repr_c::vec2::Vec2;

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

#[async_trait(?Send)]
impl NodeWrite for Note {
	async fn write<W: io::Write + std::marker::Unpin>(
		&self,
		writer: &mut W,
	) -> io::Result<(usize, ChunkDependencies)> {
		let size = self.content().write(writer).await?;
		Ok((size, ChunkDependencies::default()))
	}
}
