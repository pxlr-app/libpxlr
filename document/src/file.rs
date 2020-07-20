use crate::prelude::*;
use nom;
use std::{collections::BTreeMap, error::Error, fmt};

#[derive(Debug)]
pub enum FileError {
	Unknown,
	Parse(nom::Err<((), nom::error::ErrorKind)>),
	VersionNotSupported(u8),
	NodeNotFound(Uuid),
}

impl Error for FileError {}

impl fmt::Display for FileError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			FileError::Unknown => write!(f, "Unknown error."),
			FileError::Parse(_) => write!(f, "Could not parse the file."),
			FileError::VersionNotSupported(ver) => write!(f, "Version {} not supported.", ver),
			FileError::NodeNotFound(id) => write!(f, "Node {} not found.", id),
		}
	}
}

impl From<io::Error> for FileError {
	fn from(error: std::io::Error) -> Self {
		match error.kind() {
			_ => FileError::Unknown,
		}
	}
}

impl From<nom::Err<(&[u8], nom::error::ErrorKind)>> for FileError {
	fn from(error: nom::Err<(&[u8], nom::error::ErrorKind)>) -> Self {
		match error {
			nom::Err::Incomplete(e) => FileError::Parse(nom::Err::Incomplete(e)),
			nom::Err::Error(e) => FileError::Parse(nom::Err::Error(((), e.1))),
			nom::Err::Failure(e) => FileError::Parse(nom::Err::Error(((), e.1))),
		}
	}
}

pub struct File {
	header: parser::Header,
	index: parser::v0::Index,
	rows: Vec<parser::v0::IndexRow>,
	uuid_index: BTreeMap<Uuid, usize>,
	cache_node: BTreeMap<Uuid, NodeRef>,
}

impl File {
	pub fn new() -> Self {
		Self::from_parts(
			parser::Header { version: 0 },
			parser::v0::Index {
				hash: Uuid::new_v4(),
				size: 0,
				prev_offset: 0,
			},
			Vec::new(),
		)
	}

	fn from_parts(
		header: parser::Header,
		index: parser::v0::Index,
		rows: Vec<parser::v0::IndexRow>,
	) -> Self {
		let mut uuid_index = BTreeMap::new();
		for (i, row) in rows.iter().enumerate() {
			uuid_index.insert(row.id, i);
		}
		File {
			header,
			index,
			rows,
			uuid_index,
			cache_node: BTreeMap::new(),
		}
	}

	pub fn read<R: io::Read + io::Seek>(reader: &mut R) -> Result<Self, FileError> {
		use crate::parser::Parse;

		let mut buffer = [0u8; 5];
		reader.seek(io::SeekFrom::Start(0))?;
		reader.read_exact(&mut buffer)?;

		let (_, header) = parser::Header::parse(&buffer)?;

		let mut buffer = [0u8; 36];
		reader.seek(io::SeekFrom::End(-36))?;
		reader.read_exact(&mut buffer)?;

		let (_, index) = match header.version {
			0 => parser::v0::Index::parse(&buffer)?,
			_ => return Err(FileError::VersionNotSupported(header.version)),
		};

		let rows: Vec<parser::v0::IndexRow> = if index.size == 0 {
			Vec::new()
		} else {
			let mut buffer = vec![0u8; index.size as usize];
			reader.seek(io::SeekFrom::End(-36i64 - index.size as i64))?;
			reader.read_exact(&mut buffer)?;

			let (_, rows) = match header.version {
				0 => nom::multi::many0(parser::v0::IndexRow::parse)(&buffer)?,
				_ => panic!(FileError::VersionNotSupported(header.version)),
			};
			rows
		};

		Ok(Self::from_parts(header, index, rows))
	}

	pub fn get<R: io::Read + io::Seek>(
		&mut self,
		reader: &mut R,
		id: Uuid,
	) -> Result<NodeRef, FileError> {
		if let Some(node) = self.cache_node.get(&id) {
			Ok(node.clone())
		} else {
			match self.uuid_index.get(&id) {
				None => Err(FileError::NodeNotFound(id)),
				Some(idx) => {
					let row = unsafe { self.rows.get_unchecked(*idx) };
					let mut dependencies: Vec<&parser::v0::IndexRow> = Vec::new();
					let mut queue: Vec<Uuid> = Vec::with_capacity(self.rows.len());
					queue.push(id);
					while let Some(id) = queue.pop() {
						if let Some(idx) = self.uuid_index.get(&id) {
							let row = self.rows.get(*idx).unwrap();
							dependencies.push(row);
							queue.extend_from_slice(&row.dependencies);
							queue.extend_from_slice(&row.items);
						}
					}
					dependencies.reverse();

					for dep in dependencies.drain(..) {
						if !self.cache_node.contains_key(&dep.id) {
							let mut buffer = vec![0u8; dep.chunk_size as usize];
							reader.seek(io::SeekFrom::Start(dep.chunk_offset))?;
							reader.read_exact(&mut buffer)?;

							let items: NodeList = dep
								.items
								.iter()
								.filter_map(|idx| Some(self.cache_node.get(&idx).unwrap().clone()))
								.collect();
							let dependencies: NodeList = dep
								.dependencies
								.iter()
								.filter_map(|idx| Some(self.cache_node.get(&idx).unwrap().clone()))
								.collect();

							// TODO row.chunk_type => reader
						}
					}

					Ok(self.cache_node.get(&row.id).unwrap().clone())
				}
			}
		}
	}
}
