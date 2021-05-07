use crate::{Chunk, ChunkDependencies, NodeParse, NodeWrite, Parse, Write};
use color::Channel;
use document_core::{CanvasGroup, HasChannel, HasChildren, NodeType};
use nom::IResult;
use std::{io, sync::Arc};
use vek::vec::repr_c::vec2::Vec2;

impl NodeParse for CanvasGroup {
	fn parse<'bytes>(
		_version: u8,
		chunk: &Chunk,
		dependencies: ChunkDependencies,
		bytes: &'bytes [u8],
	) -> IResult<&'bytes [u8], Arc<NodeType>> {
		// TODO dependencies.children filter is_child_valid
		let (bytes, channel) = Channel::parse(bytes)?;
		Ok((
			bytes,
			Arc::new(NodeType::CanvasGroup(unsafe {
				CanvasGroup::construct(
					chunk.id,
					chunk.name.clone(),
					Vec2::new(chunk.rect.x, chunk.rect.y),
					channel,
					dependencies.children.clone(),
				)
			})),
		))
	}
}

impl NodeWrite for CanvasGroup {
	fn write<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
	) -> io::Result<(usize, ChunkDependencies)> {
		let size = self.channel().write(writer)?;
		Ok((
			size,
			ChunkDependencies {
				children: self.children().clone(),
				..Default::default()
			},
		))
	}
}
