use crate::{Chunk, ChunkDependencies};
use document_core::NodeType;
use nom::IResult;
use std::{io, sync::Arc};
use vek::geom::repr_c::Rect;
mod canvas;
mod core;
mod group;
mod note;
mod palette;

pub use self::canvas::*;
pub use self::core::*;
pub use self::group::*;
pub use self::note::*;
pub use self::palette::*;

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

pub trait NodeWrite {
	fn write<W: io::Write + io::Seek>(
		&self,
		writer: &mut W,
	) -> io::Result<(usize, Rect<i32, i32>, ChunkDependencies)>;
}
