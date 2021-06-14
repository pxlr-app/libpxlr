mod backend;
mod parser;

pub use self::backend::*;
pub use self::parser::*;
use async_recursion::async_recursion;
use async_std::io;
use async_std::io::prelude::WriteExt;
use document_core::{HasBounds, Node, NodeType, Unloaded};
use document_file::{
	Chunk, ChunkDependencies, FileError, Footer, Message, NodeId, NodeParse, NodeWrite, Parse,
	Write,
};
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

impl<B: Backend + PartialEq + Clone> CloudFile<B> {
	pub fn location(&self) -> &Location<Uuid> {
		&self.index.location
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

	pub async fn read(backend: B, location: Location<Uuid>) -> Result<Self, FileError> {
		use async_std::io::prelude::ReadExt;
		use async_std::io::prelude::SeekExt;

		let mut reader = backend.get_reader(&location).await?;

		let mut buffer = [0u8; 5];
		reader.seek(async_std::io::SeekFrom::End(-5)).await?;
		reader.read_exact(&mut buffer).await?;
		let (_, footer) = Footer::parse(&buffer)?;

		match footer.version {
			0 => {
				let mut buffer = [0u8; 112];
				reader.seek(async_std::io::SeekFrom::End(-5 - 112)).await?;
				reader.read_exact(&mut buffer).await?;
				let (_, index) = CloudIndex::parse(&buffer)?;

				let mut buffer = vec![0u8; index.inner_index.message_size as usize];
				reader
					.seek(async_std::io::SeekFrom::End(
						-5 - 112 - index.inner_index.message_size as i64,
					))
					.await?;
				reader.read_exact(&mut buffer).await?;
				let (_, message) = Message::parse(&buffer)?;

				let mut chunk_map = HashMap::default();
				if index.inner_index.chunks_size > 0 {
					let mut buffer = vec![0u8; index.inner_index.chunks_size as usize];
					reader
						.seek(async_std::io::SeekFrom::End(
							-5 - 112
								- index.inner_index.message_size as i64
								- index.inner_index.chunks_size as i64,
						))
						.await?;
					reader.read_exact(&mut buffer).await?;

					let (_, mut chunks) = nom::multi::many1(CloudChunk::parse)(&buffer)?;

					for chunk in chunks.drain(..) {
						chunk_map.insert(chunk.inner_chunk.id, chunk);
					}
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
			_ => Err(FileError::UnsupportedVersion(footer.version as u8)),
		}
	}

	/// Retrieve root node
	pub async fn get_root_node(&self, shallow: bool) -> Result<Arc<NodeType>, FileError> {
		self.get_node_by_id(self.index.inner_index.root, shallow)
			.await
	}

	/// Gather chunk by walking dependency tree from node
	fn get_chunk_dependencies(&self, id: Uuid) -> Vec<&CloudChunk> {
		let mut deps = vec![];
		let mut queue: Vec<Uuid> = Vec::with_capacity(self.chunks.len());
		queue.push(id);
		while let Some(id) = queue.pop() {
			if let Some(chunk) = self.chunks.get(&id) {
				deps.push(chunk);
				queue.extend_from_slice(&chunk.inner_chunk.children);
				queue.extend_from_slice(&chunk.inner_chunk.dependencies);
			}
		}
		deps
	}

	/// Retrieve a node by it's ID
	pub async fn get_node_by_id(
		&self,
		id: Uuid,
		shallow: bool,
	) -> Result<Arc<NodeType>, FileError> {
		use async_std::io::prelude::ReadExt;
		use async_std::io::prelude::SeekExt;

		if !self.chunks.contains_key(&id) {
			Err(FileError::NodeNotFound(id))
		} else {
			let mut nodes: HashMap<Uuid, Arc<NodeType>> = HashMap::default();
			let mut deps = self.get_chunk_dependencies(id);
			deps.reverse();

			for chunk in deps.drain(..) {
				if !nodes.contains_key(&chunk.inner_chunk.id) {
					let mut reader = self
						.backend
						.get_reader(&chunk.location.or(Some(self.index.location)).unwrap())
						.await?;

					let mut buffer = vec![0u8; chunk.inner_chunk.size as usize];
					reader
						.seek(async_std::io::SeekFrom::Start(chunk.inner_chunk.offset))
						.await?;
					reader.read_exact(&mut buffer).await?;

					let (children, dependencies) = if shallow {
						let children: Vec<_> = chunk
							.inner_chunk
							.children
							.iter()
							.map(|id| {
								self.chunks
									.get(&id)
									.map(|chunk| {
										Arc::new(NodeType::Unloaded(unsafe {
											Unloaded::construct(
												chunk.inner_chunk.id,
												chunk.inner_chunk.name.clone(),
												chunk.inner_chunk.rect.clone(),
											)
										}))
									})
									.unwrap()
							})
							.collect();
						let dependencies: Vec<_> = chunk
							.inner_chunk
							.dependencies
							.iter()
							.map(|id| {
								self.chunks
									.get(&id)
									.map(|chunk| {
										Arc::new(NodeType::Unloaded(unsafe {
											Unloaded::construct(
												chunk.inner_chunk.id,
												chunk.inner_chunk.name.clone(),
												chunk.inner_chunk.rect.clone(),
											)
										}))
									})
									.unwrap()
							})
							.collect();
						(children, dependencies)
					} else {
						let children: Vec<_> = chunk
							.inner_chunk
							.children
							.iter()
							.map(|id| nodes.get(&id).unwrap().clone())
							.collect();
						let dependencies: Vec<_> = chunk
							.inner_chunk
							.dependencies
							.iter()
							.map(|id| nodes.get(&id).unwrap().clone())
							.collect();
						(children, dependencies)
					};

					let (_, node) = NodeType::parse(
						self.footer.version,
						&chunk.inner_chunk,
						ChunkDependencies {
							children,
							dependencies,
						},
						&buffer,
					)?;
					nodes.insert(*node.id(), node);
				}
			}

			let node = nodes.get(&id).unwrap().clone();
			Ok(node)
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
				location: None,
				inner_chunk: Chunk {
					id: *node.id(),
					node_type: 0,
					offset: 0,
					name: "".to_string(),
					..Default::default()
				},
			},
		};
		chunk.location = Some(location);
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
		location: Location<Uuid>,
		author: impl Into<String>,
		message: impl Into<String>,
	) -> Result<usize, FileError> {
		let new_id = Uuid::new_v4();

		for (_, chunk) in self.chunks.iter_mut() {
			chunk.location = chunk.location.take().or(Some(location));
		}

		let mut written = 0;

		let mut writer = self.backend.get_writer(&location).await?;
		let dirty_nodes: Vec<DirtyNode> = self.dirty_nodes.drain().map(|(_, node)| node).collect();
		for doc_node in dirty_nodes.iter() {
			written += self
				.write_node(
					&mut writer,
					doc_node.content,
					doc_node.shallow,
					doc_node.node.clone(),
					written as u64,
					location,
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

		self.index.prev_location = if self.index.location == Location::default() {
			None
		} else {
			Some(self.index.location)
		};
		self.index.location = location;
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
	pub async fn read_previous(&self) -> Result<Self, FileError> {
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

		let location = Location(Uuid::new_v4(), Uuid::new_v4());
		let written = task::block_on(doc.write(location, "Test", "My message"))
			.expect("Could not write file");
		assert!(written > 0);

		let doc2 =
			task::block_on(CloudFile::read(vault.clone(), location)).expect("Could not read file");
		assert_eq!(doc, doc2);

		let root2 = task::block_on(doc2.get_root_node(false)).expect("Could not get root");
		assert_eq!(*root2, *root);
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

		let note_a = Arc::new(NodeType::Note(unsafe {
			Note::construct(
				Uuid::parse_str("1b1fb655-1110-4e25-9c88-0d49d21456ad").unwrap(),
				"Note A".into(),
				(0, 0).into(),
				"AAAAAAAAAA".into(),
			)
		}));
		let note_id = *note_a.id();
		let note_b = unsafe {
			Note::construct(
				Uuid::parse_str("cfc592b5-9bc1-49b4-b2d9-40ae0cb97c6a").unwrap(),
				"Note B".into(),
				(0, 0).into(),
				"BBBBBBBBBB".into(),
			)
		};
		let group = unsafe {
			Group::construct(
				Uuid::parse_str("4e3da9cc-2661-4725-b70e-15425a0d1ac2").unwrap(),
				"Group".into(),
				(0, 0).into(),
				vec![note_a.clone(), Arc::new(NodeType::Note(note_b))],
			)
		};
		let root = Arc::new(NodeType::Group(group));
		doc0.set_root_node(root.clone());

		let loc0 = Location(
			Uuid::parse_str("9da7f444-6bf0-4723-b4dd-5b695f30d42e").unwrap(),
			Uuid::parse_str("8369e12b-e3e6-4c3a-87f2-c77073a3f3ae").unwrap(),
		);
		let written =
			task::block_on(doc0.write(loc0, "Test", "My message")).expect("Could not write file");
		assert!(written > 0);

		let rename = RenameCommand::new(note_id, "Note AA");
		let root2 = Arc::new(rename.execute(&*root).expect("Could not rename"));
		assert_ne!(*root2, *root);

		let note_a2 = find(&root2, &note_id).expect("Could not find note_id");

		let mut doc1 = doc0.clone();
		doc1.update_node(note_a2.clone(), false);

		let loc1 = Location(
			Uuid::parse_str("1ab52e6b-109b-40e8-bb4b-ece03bf7c3f7").unwrap(),
			Uuid::parse_str("c542dcee-68e5-4875-a055-14e1b96ef4f1").unwrap(),
		);
		let written2 = task::block_on(doc1.write(loc1, "Test", "B")).expect("Could not write");
		assert_eq!(written2, 474);

		let doc2 =
			task::block_on(CloudFile::read(vault.clone(), loc1)).expect("Could not read file");

		let root2 = task::block_on(doc2.get_root_node(false)).expect("Could not get root");
		let note_a3 = find(&root2, &note_id).expect("Could not find note_id");
		assert_eq!(note_a3.name(), "Note AA");

		let doc3 =
			task::block_on(CloudFile::read(vault.clone(), loc0)).expect("Could not read file");
		assert_eq!(doc3, doc0);
	}
}
