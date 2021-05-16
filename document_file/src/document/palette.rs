use crate::{Chunk, ChunkDependencies, NodeParse, NodeWrite, Parse, Write};
use async_std::io;
use async_trait::async_trait;
use color::Rgba;
use document_core::{HasColors, NodeType, Palette};
use nom::{multi::many_m_n, number::complete::le_u8, IResult};
use std::sync::Arc;
use vek::vec::repr_c::vec2::Vec2;

impl NodeParse for Palette {
	fn parse<'bytes>(
		_version: u8,
		chunk: &Chunk,
		_dependencies: ChunkDependencies,
		bytes: &'bytes [u8],
	) -> IResult<&'bytes [u8], Arc<NodeType>> {
		let (bytes, len) = le_u8(bytes)?;
		let (bytes, colors) = many_m_n(len as usize, len as usize, Rgba::parse)(bytes)?;
		Ok((
			bytes,
			Arc::new(NodeType::Palette(unsafe {
				Palette::construct(
					chunk.id,
					chunk.name.clone(),
					Vec2::new(chunk.rect.x, chunk.rect.y),
					colors,
				)
			})),
		))
	}
}

#[async_trait(?Send)]
impl NodeWrite for Palette {
	async fn write<W: io::Write + std::marker::Unpin>(
		&self,
		writer: &mut W,
	) -> io::Result<(usize, ChunkDependencies)> {
		use async_std::io::prelude::WriteExt;
		let mut size = 1;
		writer
			.write(&(self.colors().len() as u8).to_le_bytes())
			.await?;
		for color in self.colors().iter() {
			size += color.write(writer).await?;
		}
		Ok((size, ChunkDependencies::default()))
	}
}
