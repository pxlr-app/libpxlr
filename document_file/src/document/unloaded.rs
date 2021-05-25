use crate::{Chunk, ChunkDependencies, NodeParse, NodeWrite};
use async_std::io;
use async_trait::async_trait;
use document_core::{NodeType, Unloaded};
use nom::IResult;
use std::sync::Arc;

impl NodeParse for Unloaded {
	fn parse<'bytes>(
		_version: u8,
		_chunk: &Chunk,
		_dependencies: ChunkDependencies,
		_bytes: &'bytes [u8],
	) -> IResult<&'bytes [u8], Arc<NodeType>> {
		unreachable!()
	}
}

#[async_trait(?Send)]
impl NodeWrite for Unloaded {
	async fn write<W: io::Write + std::marker::Unpin>(
		&self,
		_writer: &mut W,
	) -> io::Result<(usize, ChunkDependencies)> {
		unreachable!()
	}
}
