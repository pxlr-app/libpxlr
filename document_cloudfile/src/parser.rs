use async_std::io;
use async_trait::async_trait;
use document_file::{Chunk, Index, Parse, Write};
use nom::IResult;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Location<T>(pub T, pub T);

#[derive(Debug, Clone, PartialEq)]
pub struct CloudIndex {
	pub location: Location<Uuid>,
	pub inner_index: Index,
	pub prev_location: Option<Location<Uuid>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CloudChunk {
	pub inner_chunk: Chunk,
	pub location: Option<Location<Uuid>>,
}

impl<T: Default> Default for Location<T> {
	fn default() -> Self {
		Location(T::default(), T::default())
	}
}

impl Default for CloudIndex {
	fn default() -> Self {
		Self {
			location: Location::default(),
			inner_index: Index::default(),
			prev_location: None,
		}
	}
}

impl Default for CloudChunk {
	fn default() -> Self {
		Self {
			inner_chunk: Chunk::default(),
			location: None,
		}
	}
}

impl<T: Parse> Parse for Location<T> {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Location<T>> {
		let (bytes, owner) = T::parse(bytes)?;
		let (bytes, key) = T::parse(bytes)?;
		Ok((bytes, Location(owner, key)))
	}
}

#[async_trait(?Send)]
impl<T: Write + Default> Write for Location<T> {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		let mut size = self.0.write(writer).await?;
		size += self.1.write(writer).await?;
		Ok(size)
	}
}

impl Parse for CloudIndex {
	fn parse(bytes: &[u8]) -> IResult<&[u8], CloudIndex> {
		let (bytes, location) = <Location<Uuid>>::parse(bytes)?;
		let (bytes, index) = Index::parse(bytes)?;
		let (bytes, prev_location) = Location::parse(bytes)?;
		let prev_location = if prev_location == <Location<Uuid>>::default() {
			None
		} else {
			Some(prev_location)
		};

		Ok((
			bytes,
			CloudIndex {
				location,
				inner_index: index,
				prev_location,
			},
		))
	}
}

#[async_trait(?Send)]
impl Write for CloudIndex {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		let mut size = self.location.write(writer).await?;
		size += self.inner_index.write(writer).await?;
		if let Some(location) = &self.prev_location {
			size += location.write(writer).await?;
		} else {
			size += <Location<Uuid>>::default().write(writer).await?;
		}
		Ok(size)
	}
}

impl Parse for CloudChunk {
	fn parse(bytes: &[u8]) -> IResult<&[u8], CloudChunk> {
		let (bytes, chunk) = Chunk::parse(bytes)?;
		let (bytes, location) = Location::parse(bytes)?;
		let location = if location == <Location<Uuid>>::default() {
			None
		} else {
			Some(location)
		};
		Ok((
			bytes,
			CloudChunk {
				inner_chunk: chunk,
				location,
			},
		))
	}
}

#[async_trait(?Send)]
impl Write for CloudChunk {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		let mut size: usize = 0;
		size += self.inner_chunk.write(writer).await?;
		if let Some(location) = &self.location {
			size += location.write(writer).await?;
		} else {
			size += <Location<Uuid>>::default().write(writer).await?;
		}
		Ok(size)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use async_std::task;
	use vek::geom::repr_c::Rect;

	#[test]
	fn location_parse() {
		let loc = <Location<Uuid>>::default();
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(loc.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(size, 32);

		let (_, loc2) = <Location<Uuid>>::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(loc2, loc);

		let loc = Location(Uuid::new_v4(), Uuid::new_v4());
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(loc.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(size, 32);
	}

	#[test]
	fn cloudindex_parse() {
		let index = CloudIndex {
			location: Location::default(),
			inner_index: Index {
				hash: Uuid::new_v4(),
				root: Uuid::new_v4(),
				chunks_size: 1,
				message_size: 2,
				prev_offset: 3,
			},
			prev_location: None,
		};
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(index.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(size, 112);

		let (_, index2) = CloudIndex::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(index2, index);
	}

	#[test]
	fn cloudchunk_parse() {
		let chunk = CloudChunk {
			inner_chunk: Chunk {
				id: Uuid::new_v4(),
				node_type: 1,
				offset: 2,
				size: 3,
				rect: Rect::new(4, 5, 6, 7),
				name: "Chunk".into(),
				children: vec![Uuid::new_v4(), Uuid::new_v4()],
				dependencies: vec![Uuid::new_v4()],
			},
			location: None,
		};
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(chunk.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, chunk2) = CloudChunk::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(chunk2, chunk);
	}
}
