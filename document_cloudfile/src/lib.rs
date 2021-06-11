mod backend;
mod parser;

pub use self::backend::*;
pub use self::parser::*;
use async_recursion::async_recursion;
use async_std::io;
use async_std::io::prelude::WriteExt;
use document_core::HasBounds;
use document_core::{Node, NodeType};
use document_file::NodeWrite;
use document_file::{Chunk, FileError, Footer, Message, NodeId, Parse, Write};
use std::{collections::HashMap, convert::TryInto, sync::Arc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
struct DirtyNode {
	content: bool,
	shallow: bool,
	node: Arc<NodeType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CloudFile<B: Backend + PartialEq> {
	backend: B,
	footer: Footer,
	index: CloudIndex,
	message: Message,
	chunks: HashMap<Uuid, CloudChunk>,
	dirty_nodes: HashMap<Uuid, DirtyNode>,
}

#[derive(Debug)]
pub enum CloudFileError {
	FileError(FileError),
	LocationError(LocationError<Uuid>),
}

impl std::error::Error for CloudFileError {}

impl std::fmt::Display for CloudFileError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			CloudFileError::FileError(err) => err.fmt(f),
			CloudFileError::LocationError(err) => err.fmt(f),
		}
	}
}

impl From<async_std::io::Error> for CloudFileError {
	fn from(error: async_std::io::Error) -> Self {
		CloudFileError::FileError(FileError::IO(error))
	}
}

impl From<FileError> for CloudFileError {
	fn from(error: FileError) -> Self {
		CloudFileError::FileError(error)
	}
}

impl From<LocationError<Uuid>> for CloudFileError {
	fn from(error: LocationError<Uuid>) -> Self {
		CloudFileError::LocationError(error)
	}
}

impl From<nom::Err<nom::error::Error<&[u8]>>> for CloudFileError {
	fn from(error: nom::Err<nom::error::Error<&[u8]>>) -> Self {
		match error {
			nom::Err::Incomplete(needed) => {
				CloudFileError::FileError(FileError::Parse(nom::Err::Incomplete(needed)))
			}
			nom::Err::Failure(_) => {
				CloudFileError::FileError(FileError::Parse(nom::Err::Failure(())))
			}
			nom::Err::Error(_) => CloudFileError::FileError(FileError::Parse(nom::Err::Error(()))),
		}
	}
}

impl<B: Backend + PartialEq + Clone> CloudFile<B> {
	pub fn location(&self) -> Location<Uuid> {
		(self.index.realm, self.index.inner_index.hash).into()
	}

	pub fn backend(&self) -> &B {
		&self.backend
	}

	pub fn footer(&self) -> &Footer {
		&self.footer
	}

	pub fn index(&self) -> &CloudIndex {
		&self.index
	}

	pub fn message(&self) -> &Message {
		&self.message
	}

	pub fn chunks(&self) -> &HashMap<Uuid, CloudChunk> {
		&self.chunks
	}

	pub async fn new(backend: B) -> io::Result<Self> {
		Ok(CloudFile {
			backend,
			footer: Footer::default(),
			index: CloudIndex::default(),
			message: Message::default(),
			chunks: HashMap::default(),
			dirty_nodes: HashMap::default(),
		})
	}

	pub async fn read(mut backend: B, location: Location<Uuid>) -> Result<Self, CloudFileError> {
		use async_std::io::prelude::ReadExt;
		use async_std::io::prelude::SeekExt;

		let mut reader = backend.get_reader(&location).await?;

		let mut buffer = [0u8; 5];
		reader.seek(async_std::io::SeekFrom::End(-5)).await?;
		reader.read_exact(&mut buffer).await?;
		let (_, footer) = Footer::parse(&buffer)?;

		match footer.version {
			0 => {
				let mut buffer = [0u8; 99];
				reader.seek(async_std::io::SeekFrom::End(-5 - 99)).await?;
				reader.read_exact(&mut buffer).await?;
				let (_, index) = CloudIndex::parse(&buffer)?;

				let mut buffer = vec![0u8; index.inner_index.message_size as usize];
				reader
					.seek(async_std::io::SeekFrom::End(
						-5 - 99 - index.inner_index.message_size as i64,
					))
					.await?;
				reader.read_exact(&mut buffer).await?;
				let (_, message) = Message::parse(&buffer)?;

				let mut buffer = vec![0u8; index.inner_index.chunks_size as usize];
				reader
					.seek(async_std::io::SeekFrom::End(
						-5 - 99
							- index.inner_index.message_size as i64
							- index.inner_index.chunks_size as i64,
					))
					.await?;
				reader.read_exact(&mut buffer).await?;

				let (_, mut chunks) = nom::multi::many0(CloudChunk::parse)(&buffer)?;
				let mut chunk_map = HashMap::default();

				for chunk in chunks.drain(..) {
					chunk_map.insert(chunk.inner_chunk.id, chunk);
				}

				// TODO assert chunks tree is sound (all children/deps ID exists)

				Ok(CloudFile {
					backend,
					footer,
					index,
					chunks: chunk_map,
					message,
					dirty_nodes: HashMap::default(),
				})
			}
			_ => Err(CloudFileError::FileError(FileError::UnsupportedVersion(
				footer.version as u8,
			))),
		}
	}

	/// Mark node as dirty (everything) and use it as root node
	pub fn set_root_node(&mut self, node: Arc<NodeType>) {
		self.index.inner_index.root = *node.id();
		self.update_node(node, false);
	}

	/// Mark node as dirty
	fn mark_dirty_node(&mut self, content: bool, shallow: bool, node: Arc<NodeType>) {
		let id = *node.id();
		let mut doc_node = match self.dirty_nodes.remove(&id) {
			Some(node) => node,
			None => DirtyNode {
				node: node.clone(),
				content,
				shallow,
			},
		};
		doc_node.content |= content;
		doc_node.shallow |= shallow;
		doc_node.node = node.clone();
		self.dirty_nodes.insert(id, doc_node);
	}

	/// Mark node and children (shallow?) as dirty (content, meta)
	pub fn update_node(&mut self, node: Arc<NodeType>, shallow: bool) {
		self.mark_dirty_node(true, shallow, node.clone());
	}

	/// Mark node and children (shallow?) as dirty (meta only)
	pub fn touch_node(&mut self, node: Arc<NodeType>, shallow: bool) {
		self.mark_dirty_node(false, shallow, node.clone());
	}

	/// Write node at the end of file and return chunk dependencies
	#[async_recursion(?Send)]
	async fn write_node<W: async_std::io::Write + std::marker::Unpin>(
		&mut self,
		writer: &mut W,
		content: bool,
		shallow: bool,
		node: Arc<NodeType>,
		offset: u64,
		location: Location<Uuid>,
	) -> async_std::io::Result<usize> {
		if let NodeType::Unloaded(_) = *node {
			return Ok(0);
		}

		let mut written = 0;
		let mut chunk = match self.chunks.remove(node.id()) {
			Some(chunk) => chunk,
			None => CloudChunk {
				location: Location::Local,
				inner_chunk: Chunk {
					id: *node.id(),
					node_type: 0,
					offset: 0,
					name: "".to_string(),
					..Default::default()
				},
			},
		};
		chunk.location = location;
		chunk.inner_chunk.node_type = node.node_id();
		chunk.inner_chunk.offset = offset;
		chunk.inner_chunk.name = node.name().to_string();

		if let Ok(node_bounds) = TryInto::<&dyn HasBounds>::try_into(&*node) {
			chunk.inner_chunk.rect = node_bounds.bounds();
		}

		let (node_size, node_deps) = if content {
			NodeWrite::write(&*node, writer).await
		} else {
			NodeWrite::write(&*node, &mut document_file::io::Void).await
		}?;

		chunk.inner_chunk.children = node_deps.children.iter().map(|dep| *dep.id()).collect();
		chunk.inner_chunk.dependencies =
			node_deps.dependencies.iter().map(|dep| *dep.id()).collect();

		if content {
			chunk.inner_chunk.size = node_size as u32;
			written += node_size;
		}

		self.chunks.insert(*node.id(), chunk);

		if !shallow {
			for child in node_deps.children.iter() {
				written += self
					.write_node(
						writer,
						content,
						shallow,
						child.clone(),
						written as u64,
						location,
					)
					.await?;
			}
			for dep in node_deps.dependencies.iter() {
				written += self
					.write_node(
						writer,
						content,
						shallow,
						dep.clone(),
						written as u64,
						location,
					)
					.await?;
			}
		}

		Ok(written)
	}

	/// Write new object to backend
	pub async fn write(
		&mut self,
		realm: Realm<Uuid>,
		author: impl Into<String>,
		message: impl Into<String>,
	) -> Result<usize, CloudFileError> {
		let new_id = Uuid::new_v4();

		for (_, chunk) in self.chunks.iter_mut() {
			chunk.location = match (&realm, &chunk.location) {
				(&Realm::Public, &Location::Local) => Location::Public(new_id),
				(&Realm::Private(owner), &Location::Local) => {
					Location::Private { owner, key: new_id }
				}
				(_, &Location::Public(key)) => Location::Public(key),
				(&Realm::Public, &Location::Private { .. }) => {
					return Err(LocationError::PrivateLeak.into())
				}
				(&Realm::Private(owner_a), &Location::Private { owner, key }) => {
					if owner_a == owner {
						Location::Private { owner, key }
					} else {
						return Err(LocationError::NotSameOwner(owner_a, owner).into());
					}
				}
			};
		}

		let new_loc: Location<Uuid> = (realm, new_id).into();
		let mut written = 0;

		let mut writer = self.backend.get_writer(&new_loc).await?;
		let dirty_nodes: Vec<DirtyNode> = self.dirty_nodes.drain().map(|(_, node)| node).collect();
		for doc_node in dirty_nodes.iter() {
			written += self
				.write_node(
					&mut writer,
					doc_node.content,
					doc_node.shallow,
					doc_node.node.clone(),
					written as u64,
					new_loc,
				)
				.await?;
		}

		let mut chunks_size = 0;
		for (_, chunk) in self.chunks.iter() {
			chunks_size += chunk.write(&mut writer).await?;
		}
		written += chunks_size;

		self.message.date = chrono::offset::Utc::now().timestamp() as u64;
		self.message.author = author.into();
		self.message.message = message.into();
		let message_size = self.message.write(&mut writer).await?;
		written += message_size;

		self.index.prev_location = Some((self.index.realm, self.index.inner_index.hash).into());
		self.index.realm = realm;
		self.index.inner_index.hash = new_id;
		self.index.inner_index.prev_offset = 0;
		self.index.inner_index.message_size = message_size as u32;
		self.index.inner_index.chunks_size = chunks_size as u32;

		written += self.index.write(&mut writer).await?;
		written += (Footer { version: 0 }).write(&mut writer).await?;

		writer.flush().await?;

		Ok(written)
	}

	/// Retrieve a new document pointing to previous File
	pub async fn read_previous(&self) -> Result<Self, CloudFileError> {
		match self.index.prev_location {
			Some(prev_location) => Self::read(self.backend.clone(), prev_location).await,
			None => Err(FileError::NoPreviousVersion.into()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use async_std::task;
	use document_command::*;
	use document_core::*;
	use tempdir::TempDir;

	#[test]
	fn write_and_read_simple_doc() {
		let root = TempDir::new("").expect("Could not get tempdir");
		let vault = LocalVaultBackend::new(root.path());

		// let root = std::env::temp_dir().join("write_and_read_simple_doc");
		// std::fs::create_dir_all(&root).expect("Could not create tmp dir");
		// let vault = LocalVaultBackend::new(root);

		let mut doc =
			task::block_on(CloudFile::new(vault.clone())).expect("Could not create new file");

		let note = Note::new("My note", (0, 0), "");
		let root = Arc::new(NodeType::Note(note));
		doc.set_root_node(root.clone());

		let written = task::block_on(doc.write(Realm::Public, "Test", "My message"))
			.expect("Could not write file");
		assert!(written > 0);

		let loc = doc.location();
		let doc2 =
			task::block_on(CloudFile::read(vault.clone(), loc)).expect("Could not read file");
		assert_eq!(doc, doc2);
	}

	#[test]
	fn rename_read_previous_doc() {
		let root = TempDir::new("").expect("Could not get tempdir");
		let vault = LocalVaultBackend::new(root.path());

		// let root = std::env::temp_dir().join("rename_read_previous_doc");
		// std::fs::create_dir_all(&root).expect("Could not create tmp dir");
		// let vault = LocalVaultBackend::new(root);

		let mut doc0 =
			task::block_on(CloudFile::new(vault.clone())).expect("Could not create new file");

		let note_a = Arc::new(NodeType::Note(Note::new("Note A", (0, 0), "AAAAAAAAAA")));
		let note_id = *note_a.id();
		let note_b = Note::new("Note B", (0, 0), "BBBBBBBBBB");
		let group = Group::new(
			"Group",
			(0, 0),
			vec![note_a.clone(), Arc::new(NodeType::Note(note_b))],
		);
		let root = Arc::new(NodeType::Group(group));
		doc0.set_root_node(root.clone());

		let written = task::block_on(doc0.write(Realm::Public, "Test", "My message"))
			.expect("Could not write file");
		assert!(written > 0);
		let loc = doc0.location();

		let rename = RenameCommand::new(note_id, "Note AA");
		let root2 = Arc::new(rename.execute(&*root).expect("Could not rename"));
		assert_ne!(*root2, *root);

		let mut doc1 = doc0.clone();
		doc1.update_node(note_a.clone(), false);

		let written2 =
			task::block_on(doc1.write(Realm::Public, "Test", "B")).expect("Could not write");
		assert!(written2 > 0);
		let loc2 = doc1.location();

		assert_ne!(loc2, loc);

		let doc2 =
			task::block_on(CloudFile::read(vault.clone(), loc2)).expect("Could not read file");
		assert_eq!(doc2, doc1);

		let doc3 =
			task::block_on(CloudFile::read(vault.clone(), loc)).expect("Could not read file");
		assert_eq!(doc3, doc0);
	}
}
