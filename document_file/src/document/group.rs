use crate::{Chunk, ChunkDependencies, NodeParse, NodeWrite};
use async_std::io;
use async_trait::async_trait;
use document_core::{Group, HasChildren, NodeType};
use nom::IResult;
use std::sync::Arc;
use vek::vec::repr_c::vec2::Vec2;

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

#[async_trait(?Send)]
impl NodeWrite for Group {
	async fn write<W: io::Write + std::marker::Unpin>(
		&self,
		_writer: &mut W,
	) -> io::Result<(usize, ChunkDependencies)> {
		Ok((
			0,
			ChunkDependencies {
				children: self.children().clone(),
				..Default::default()
			},
		))
	}
}
