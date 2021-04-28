use crate::{Chunk, ChunkDependencies, NodeParse, NodeWrite};
use document_core::{Group, NodeType};
use nom::IResult;
use std::{io, sync::Arc};
use vek::{geom::repr_c::Rect, vec::repr_c::vec2::Vec2};

impl NodeParse for Group {
	fn parse<'bytes>(
		chunk: &Chunk,
		dependencies: ChunkDependencies,
		bytes: &'bytes [u8],
	) -> IResult<&'bytes [u8], Arc<NodeType>> {
		// TODO dependencies.children filter is_child_valid
		Ok((
			bytes,
			Arc::new(NodeType::Group(Group {
				id: chunk.id,
				position: Arc::new(Vec2::new(chunk.rect.x, chunk.rect.y)),
				name: Arc::new(chunk.name.clone()),
				children: dependencies.children.clone(),
			})),
		))
	}
}

impl NodeWrite for Group {
	fn write<W: io::Write + io::Seek>(
		&self,
		_writer: &mut W,
	) -> io::Result<(usize, Rect<u32, u32>, ChunkDependencies)> {
		Ok((
			0,
			Rect::default(),
			ChunkDependencies {
				children: self.children.clone(),
				..Default::default()
			},
		))
	}
}
