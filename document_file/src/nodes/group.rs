use crate::{Chunk, ChunkDependencies, NodeParse, NodeWrite};
use document_core::{Group, HasChildren, NodeType};
use nom::IResult;
use std::{io, sync::Arc};
use vek::{geom::repr_c::Rect, vec::repr_c::vec2::Vec2};

impl NodeParse for Group {
	fn parse<'bytes>(
		_version: u8,
		chunk: &Chunk,
		dependencies: ChunkDependencies,
		bytes: &'bytes [u8],
	) -> IResult<&'bytes [u8], Arc<NodeType>> {
		// TODO dependencies.children filter is_child_valid
		Ok((
			bytes,
			Arc::new(NodeType::Group(unsafe {
				Group::construct(
					chunk.id,
					chunk.name.clone(),
					Vec2::new(chunk.rect.x, chunk.rect.y),
					dependencies.children.clone(),
				)
			})),
		))
	}
}

impl NodeWrite for Group {
	fn write<W: io::Write + io::Seek>(
		&self,
		_writer: &mut W,
	) -> io::Result<(usize, Rect<i32, i32>, ChunkDependencies)> {
		Ok((
			0,
			Rect::default(),
			ChunkDependencies {
				children: self.children().clone(),
				..Default::default()
			},
		))
	}
}
