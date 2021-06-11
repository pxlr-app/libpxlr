use async_std::io;
use async_trait::async_trait;
use document_file::{Chunk, Index, Parse, Write};
use nom::{number::complete::le_u8, IResult};
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Realm<T> {
	Public,
	Private(T),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Location<T> {
	Local,
	Public(T),
	Private { owner: T, key: T },
}

#[derive(Debug, Clone, PartialEq)]
pub struct CloudIndex {
	pub realm: Realm<Uuid>,
	pub inner_index: Index,
	pub prev_location: Option<Location<Uuid>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CloudChunk {
	pub inner_chunk: Chunk,
	pub location: Location<Uuid>,
}

impl<T> From<(Realm<T>, T)> for Location<T> {
	fn from(tuple: (Realm<T>, T)) -> Self {
		let (realm, key) = tuple;
		match realm {
			Realm::Public => Location::Public(key),
			Realm::Private(owner) => Location::Private { owner, key },
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum LocationError<T> {
	NotSameOwner(T, T),
	PrivateLeak,
}

impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for LocationError<T> {}

impl<T: std::fmt::Display> std::fmt::Display for LocationError<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			LocationError::NotSameOwner(owner_a, owner_b) => write!(
				f,
				"Location is referencing differents owners {}, {}",
				owner_a, owner_b
			),
			LocationError::PrivateLeak => {
				write!(f, "Location will leak private chunk into public realm")
			}
		}
	}
}

// impl<T: PartialEq> std::convert::TryFrom<(Realm<T>, T, Location<T>)> for Location<T> {
// 	type Error = LocationError<T>;

// 	fn try_from(tuple: (Realm<T>, T, Location<T>)) -> Result<Self, Self::Error> {
// 		match tuple {
// 			(_, Location::Local) => unreachable!(),
// 			(_, Location::Public(key)) => Ok(Location::Public(key)),
// 			(Realm::Private(owner_realm), Location::Private { owner, key }) => {
// 				if owner_realm == owner {
// 					Ok(Location::Private { owner, key })
// 				} else {
// 					Err(LocationError::NotSameOwner(owner_realm, owner))
// 				}
// 			}
// 			(Realm::Public, Location::Private { .. }) => Err(LocationError::PrivateLeak),
// 		}
// 	}
// }

impl<T: Default> Default for Realm<T> {
	fn default() -> Self {
		Realm::Public
	}
}

impl<T: Default> Default for Location<T> {
	fn default() -> Self {
		Location::Local
	}
}

impl Default for CloudIndex {
	fn default() -> Self {
		Self {
			realm: Realm::Public,
			inner_index: Index::default(),
			prev_location: None,
		}
	}
}

impl Default for CloudChunk {
	fn default() -> Self {
		Self {
			inner_chunk: Chunk::default(),
			location: Location::Local,
		}
	}
}

impl<T: Parse> Parse for Realm<T> {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Realm<T>> {
		let (bytes, id) = le_u8(bytes)?;
		let (bytes, owner) = T::parse(bytes)?;
		match id {
			0 => Ok((bytes, Realm::Public)),
			1 => Ok((bytes, Realm::Private(owner))),
			n => panic!("Blep {}", n),
		}
	}
}

#[async_trait(?Send)]
impl<T: Write + Default> Write for Realm<T> {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		match self {
			Realm::Public => {
				writer.write_all(&0u8.to_le_bytes()).await?;
				let size = T::default().write(writer).await?;
				Ok(1 + size)
			}
			Realm::Private(owner) => {
				writer.write_all(&1u8.to_le_bytes()).await?;
				let size = owner.write(writer).await?;
				Ok(1 + size)
			}
		}
	}
}

impl<T: Parse> Parse for Location<T> {
	fn parse(bytes: &[u8]) -> IResult<&[u8], Location<T>> {
		let (bytes, id) = le_u8(bytes)?;
		let (bytes, owner) = T::parse(bytes)?;
		let (bytes, key) = T::parse(bytes)?;
		match id {
			0 => Ok((bytes, Location::Local)),
			1 => Ok((bytes, Location::Public(key))),
			2 => Ok((bytes, Location::Private { owner, key })),
			_ => unreachable!(),
		}
	}
}

#[async_trait(?Send)]
impl<T: Write + Default> Write for Location<T> {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		match self {
			Location::Local => {
				writer.write_all(&0u8.to_le_bytes()).await?;
				let mut size = T::default().write(writer).await?;
				size += T::default().write(writer).await?;
				Ok(1 + size)
			}
			Location::Public(key) => {
				writer.write_all(&1u8.to_le_bytes()).await?;
				let mut size = T::default().write(writer).await?;
				size += key.write(writer).await?;
				Ok(1 + size)
			}
			Location::Private { owner, key } => {
				writer.write_all(&2u8.to_le_bytes()).await?;
				let mut size = owner.write(writer).await?;
				size += key.write(writer).await?;
				Ok(1 + size)
			}
		}
	}
}

impl Parse for CloudIndex {
	fn parse(bytes: &[u8]) -> IResult<&[u8], CloudIndex> {
		let (bytes, realm) = <Realm<Uuid>>::parse(bytes)?;
		let (bytes, index) = Index::parse(bytes)?;
		let (bytes, has_location) = le_u8(bytes)?;
		let (bytes, prev_location) = if has_location == 1 {
			let (bytes, location) = Location::parse(bytes)?;
			(bytes, Some(location))
		} else {
			let (bytes, _) = <Location<Uuid>>::parse(bytes)?;
			(bytes, None)
		};

		Ok((
			bytes,
			CloudIndex {
				realm,
				inner_index: index,
				prev_location,
			},
		))
	}
}

#[async_trait(?Send)]
impl Write for CloudIndex {
	async fn write<W: io::Write + std::marker::Unpin>(&self, writer: &mut W) -> io::Result<usize> {
		use async_std::io::prelude::WriteExt;
		let mut size = self.realm.write(writer).await?;
		size += self.inner_index.write(writer).await?;
		if let Some(location) = &self.prev_location {
			writer.write_all(&1u8.to_le_bytes()).await?;
			size += location.write(writer).await?;
		} else {
			writer.write_all(&0u8.to_le_bytes()).await?;
			size += <Location<Uuid>>::default().write(writer).await?;
		}
		Ok(size + 1)
	}
}

impl Parse for CloudChunk {
	fn parse(bytes: &[u8]) -> IResult<&[u8], CloudChunk> {
		let (bytes, chunk) = Chunk::parse(bytes)?;
		let (bytes, location) = Location::parse(bytes)?;
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
		size += self.location.write(writer).await?;
		Ok(size)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use async_std::task;
	use vek::geom::repr_c::Rect;

	#[test]
	fn realm_parse() {
		let realm: Realm<Uuid> = Realm::Public;
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(realm.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(size, 17);

		let (_, realm2) = Realm::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(realm2, realm);

		let realm = Realm::Private(Uuid::new_v4());
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(realm.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(size, 17);

		let (_, realm2) = Realm::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(realm2, realm);
	}

	#[test]
	fn location_parse() {
		let loc: Location<Uuid> = Location::Local;
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(loc.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(size, 33);

		let (_, loc2) = <Location<Uuid>>::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(loc2, loc);

		let loc = Location::Public(Uuid::new_v4());
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(loc.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(size, 33);

		let (_, loc2) = <Location<Uuid>>::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(loc2, loc);

		let loc = Location::Private {
			owner: Uuid::new_v4(),
			key: Uuid::new_v4(),
		};
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(loc.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);
		assert_eq!(size, 33);

		let (_, loc2) = <Location<Uuid>>::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(loc2, loc);
	}

	#[test]
	fn cloudindex_parse() {
		let index = CloudIndex {
			realm: Realm::Public,
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
		assert_eq!(size, 99);

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
			location: Location::Local,
		};
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());

		let size = task::block_on(chunk.write(&mut buffer)).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), size);

		let (_, chunk2) = CloudChunk::parse(&buffer.get_ref()).expect("Could not parse");
		assert_eq!(chunk2, chunk);
	}
}
