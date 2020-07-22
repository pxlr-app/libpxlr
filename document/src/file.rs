use crate::prelude::*;
use nom;
use std::{collections::BTreeMap, error::Error, fmt};

#[derive(Debug)]
pub enum FileError {
	Unknown,
	Parse(nom::Err<((), nom::error::ErrorKind)>),
	VersionNotSupported(u8),
	NodeNotFound(Uuid),
	NoPreviousVersionFound,
}

impl Error for FileError {}

impl fmt::Display for FileError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			FileError::Unknown => write!(f, "Unknown error."),
			FileError::Parse(_) => write!(f, "Could not parse the file."),
			FileError::VersionNotSupported(ver) => write!(f, "Version {} not supported.", ver),
			FileError::NodeNotFound(id) => write!(f, "Node {} not found.", id),
			FileError::NoPreviousVersionFound => write!(f, "No previous version found."),
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

#[derive(Debug, Clone)]
pub struct File {
	pub(crate) header: parser::Header,
	pub(crate) index: parser::v0::Index,
	pub(crate) rows: Vec<parser::v0::IndexRow>,
	pub(crate) uuid_index: BTreeMap<Uuid, usize>,
	pub(crate) cache_node: BTreeMap<Uuid, NodeRef>,
}

impl File {
	pub fn new() -> Self {
		Self::from_parts(
			parser::Header { version: 0 },
			parser::v0::Index {
				hash: Uuid::new_v4(),
				root: Uuid::default(),
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
		let end = reader.seek(io::SeekFrom::End(0))?;
		File::read_at(reader, end)
	}

	pub fn read_previous<R: io::Read + io::Seek>(&self, reader: &mut R) -> Result<Self, FileError> {
		if self.index.prev_offset == 0 {
			Err(FileError::NoPreviousVersionFound)
		} else {
			File::read_at(reader, self.index.prev_offset)
		}
	}

	pub fn read_at<R: io::Read + io::Seek>(
		reader: &mut R,
		position: u64,
	) -> Result<Self, FileError> {
		use crate::parser::Parse;

		let mut buffer = [0u8; 5];
		reader.seek(io::SeekFrom::Start(0))?;
		reader.read_exact(&mut buffer)?;

		let (_, header) = parser::Header::parse(&buffer)?;

		let mut buffer = [0u8; 44];
		reader.seek(io::SeekFrom::Start(position - 44))?;
		reader.read_exact(&mut buffer)?;

		let (_, index) = match header.version {
			0 => parser::v0::Index::parse(&buffer)?,
			_ => return Err(FileError::VersionNotSupported(header.version)),
		};

		let rows: Vec<parser::v0::IndexRow> = if index.size == 0 {
			Vec::new()
		} else {
			let mut buffer = vec![0u8; index.size as usize];
			reader.seek(io::SeekFrom::Start(position - 44 - index.size as u64))?;
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
							queue.extend_from_slice(&row.children);
						}
					}
					dependencies.reverse();

					for dep in dependencies.drain(..) {
						if !self.cache_node.contains_key(&dep.id) {
							let mut buffer = vec![0u8; dep.chunk_size as usize];
							reader.seek(io::SeekFrom::Start(dep.chunk_offset))?;
							reader.read_exact(&mut buffer)?;

							let children: NodeList = dep
								.children
								.iter()
								.filter_map(|idx| Some(self.cache_node.get(&idx).unwrap().clone()))
								.collect();
							let dependencies: NodeList = dep
								.dependencies
								.iter()
								.filter_map(|idx| Some(self.cache_node.get(&idx).unwrap().clone()))
								.collect();

							if let Ok((_, node)) = match self.header.version {
								0 => <NodeType as parser::v0::ParseNode>::parse_node(
									dep,
									children,
									dependencies,
									&buffer,
								),
								v => return Err(FileError::VersionNotSupported(v)),
							} {
								self.cache_node.insert(dep.id, node);
							}
						}
					}

					if let Some(node) = self.cache_node.get(&row.id) {
						Ok(node.clone())
					} else {
						Err(FileError::NodeNotFound(id))
					}
				}
			}
		}
	}

	pub fn write<W: io::Write + io::Seek>(
		&mut self,
		writer: &mut W,
		node: &NodeType,
	) -> io::Result<usize> {
		use parser::Write;
		writer.seek(io::SeekFrom::Start(0))?;
		let mut size = self.header.write(writer)?;
		let mut rows: Vec<parser::v0::IndexRow> = Vec::new();
		size += self.write_node(writer, &mut rows, node)?;
		self.rows = rows;
		self.index.root = node.as_node().id();
		self.index.prev_offset = 0;
		size += self.write_index(writer)?;
		Ok(size)
	}

	fn write_node<W: io::Write + io::Seek>(
		&mut self,
		writer: &mut W,
		rows: &mut Vec<parser::v0::IndexRow>,
		node: &NodeType,
	) -> io::Result<usize> {
		use parser::v0::WriteNode;
		let mut dependencies: Vec<NodeRef> = Vec::new();
		let mut size = node.write_node(writer, rows, &mut dependencies)?;
		while let Some(dep) = dependencies.pop() {
			size += dep.write_node(writer, rows, &mut dependencies)?;
		}
		Ok(size)
	}

	fn write_index<W: io::Write + io::Seek>(&mut self, writer: &mut W) -> io::Result<usize> {
		use parser::Write;
		let mut size: usize = 0;
		self.uuid_index.clear();
		for (i, row) in self.rows.iter().enumerate() {
			self.uuid_index.insert(row.id, i);
			size += row.write(writer)?;
		}
		self.index.size = size as u32;
		size += self.index.write(writer)?;
		Ok(size)
	}

	pub fn update<W: io::Write + io::Seek>(
		&mut self,
		writer: &mut W,
		node: &NodeType,
	) -> io::Result<usize> {
		self.index.prev_offset = writer.seek(io::SeekFrom::End(0))?;
		let mut rows: Vec<parser::v0::IndexRow> = Vec::new();
		let mut size = self.write_node(writer, &mut rows, node)?;
		for row in rows.drain(..) {
			if let Some(old_row) = self.rows.iter_mut().find(|r| r.id == row.id) {
				*old_row = row;
			} else {
				self.rows.push(row);
			}
		}
		size += self.write_index(writer)?;
		Ok(size)
	}
}
