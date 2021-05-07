use crate::{Chunk, ChunkDependencies, NodeParse, NodeWrite, Parse, Write};
use color::Rgba;
use document_core::{NodeType, Palette};
use nom::{multi::many_m_n, number::complete::le_u8, IResult};
use std::{io, sync::Arc};
use vek::{geom::repr_c::Rect, vec::repr_c::vec2::Vec2};

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
			Arc::new(NodeType::Palette(Palette {
				id: chunk.id,
				position: Arc::new(Vec2::new(chunk.rect.x, chunk.rect.y)),
				name: Arc::new(chunk.name.clone()),
				colors: Arc::new(colors),
			})),
		))
	}
}

impl NodeWrite for Palette {
	fn write<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
	) -> io::Result<(usize, Rect<i32, i32>, ChunkDependencies)> {
		let mut size = 1;
		writer.write_all(&(self.colors.len() as u8).to_le_bytes())?;
		for color in self.colors.iter() {
			size += color.write(writer)?;
		}
		Ok((size, Rect::default(), ChunkDependencies::default()))
	}
}
