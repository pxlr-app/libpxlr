use std::sync::Arc;

use async_std::io;
use async_trait::async_trait;
use document_core::NodeType;
use nom::IResult;

use crate::{Chunk, ChunkDependencies};

pub trait Parse {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Self>
	where
		Self: Sized;
}

#[async_trait(?Send)]
pub trait Write {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize>;
}

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

#[async_trait(?Send)]
pub trait NodeWrite {
	async fn write<W: io::Write + std::marker::Unpin>(
		&self,
		writer: &mut W,
	) -> io::Result<(usize, ChunkDependencies)>;
}
