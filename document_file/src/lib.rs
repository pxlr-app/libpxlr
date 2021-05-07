use document_core::{HasBounds, Node, NodeType};
use std::{collections::HashMap, convert::TryInto, sync::Arc};
use uuid::Uuid;
pub mod io;
mod meta;
mod nodes;
mod parser;
mod primitives;

pub use self::meta::*;
pub use self::nodes::*;
pub use self::parser::*;
pub use self::primitives::*;

#[derive(Debug)]
struct DocumentNode {
	content: bool,
	shallow: bool,
	node: Arc<NodeType>,
}

pub struct Document {
	pub footer: Footer,
	pub index: Index,
	pub chunks: HashMap<Uuid, Chunk>,
	pub(crate) dirty_nodes: HashMap<Uuid, DocumentNode>,
}

impl Default for Document {
	fn default() -> Self {
		Document {
			footer: Footer::default(),
			index: Index::default(),
			chunks: HashMap::default(),
			dirty_nodes: HashMap::default(),
		}
	}
}

#[derive(Debug)]
pub enum DocumentError {
	IO(std::io::Error),
	Parse(nom::Err<()>),
	UnsupportedVersion(u8),
	NodeNotFound(Uuid),
	NoPreviousVersion,
}

impl std::error::Error for DocumentError {}

impl std::fmt::Display for DocumentError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			DocumentError::Parse(err) => write!(f, "Parse error: {}", err),
			DocumentError::IO(err) => write!(f, "File IO error: {}", err),
			DocumentError::UnsupportedVersion(ver) => write!(f, "Unsupported version: {}", ver),
			DocumentError::NodeNotFound(id) => write!(f, "Node {} not found", id),
			DocumentError::NoPreviousVersion => write!(f, "No previous version"),
		}
	}
}

impl From<std::io::Error> for DocumentError {
	fn from(error: std::io::Error) -> Self {
		DocumentError::IO(error)
	}
}

impl From<nom::Err<nom::error::Error<&[u8]>>> for DocumentError {
	fn from(error: nom::Err<nom::error::Error<&[u8]>>) -> Self {
		match error {
			nom::Err::Incomplete(needed) => DocumentError::Parse(nom::Err::Incomplete(needed)),
			nom::Err::Failure(_) => DocumentError::Parse(nom::Err::Failure(())),
			nom::Err::Error(_) => DocumentError::Parse(nom::Err::Error(())),
		}
	}
}

impl Document {
	/// Read from file
	pub fn read<R: std::io::Read + std::io::Seek>(reader: &mut R) -> Result<Self, DocumentError> {
		let end = reader.seek(std::io::SeekFrom::End(0))?;
		Document::read_at(reader, end)
	}

	/// Read from file at specific offset
	fn read_at<R: std::io::Read + std::io::Seek>(
		reader: &mut R,
		offset: u64,
	) -> Result<Self, DocumentError> {
		let mut buffer = [0u8; 5];
		reader.seek(std::io::SeekFrom::Start(offset - 5))?;
		reader.read_exact(&mut buffer)?;
		let (_, footer) = Footer::parse(&buffer)?;

		match footer.version {
			0 => {
				let mut buffer = [0u8; 44];
				reader.seek(std::io::SeekFrom::Start(offset - 5 - 44))?;
				reader.read_exact(&mut buffer)?;
				let (_, index) = Index::parse(&buffer)?;

				let mut buffer = vec![0u8; index.size as usize];
				reader.seek(std::io::SeekFrom::Start(
					offset - 5 - 44 - index.size as u64,
				))?;
				reader.read_exact(&mut buffer)?;

				let (_, mut chunks) = nom::multi::many0(Chunk::parse)(&buffer)?;
				let mut chunk_map = HashMap::default();

				for chunk in chunks.drain(..) {
					chunk_map.insert(chunk.id, chunk);
				}

				// TODO assert chunks tree is sound (all children/deps ID exists)

				Ok(Document {
					footer,
					index,
					chunks: chunk_map,
					dirty_nodes: HashMap::default(),
				})
			}
			_ => Err(DocumentError::UnsupportedVersion(footer.version as u8)),
		}
	}

	/// Retrieve root node
	pub fn get_root_node<R: std::io::Read + std::io::Seek>(
		&self,
		reader: &mut R,
	) -> Result<Arc<NodeType>, DocumentError> {
		self.get_node_by_id(reader, self.index.root)
	}

	/// Gather chunk by walking dependency tree from node
	fn get_chunk_dependencies(&self, id: Uuid) -> Vec<&Chunk> {
		let mut deps = vec![];
		let mut queue: Vec<Uuid> = Vec::with_capacity(self.chunks.len());
		queue.push(id);
		while let Some(id) = queue.pop() {
			if let Some(chunk) = self.chunks.get(&id) {
				deps.push(chunk);
				queue.extend_from_slice(&chunk.children);
				queue.extend_from_slice(&chunk.dependencies);
			}
		}
		deps
	}

	/// Retrieve a node by it's ID
	pub fn get_node_by_id<R: std::io::Read + std::io::Seek>(
		&self,
		reader: &mut R,
		id: Uuid,
	) -> Result<Arc<NodeType>, DocumentError> {
		if !self.chunks.contains_key(&id) {
			Err(DocumentError::NodeNotFound(id))
		} else {
			let mut nodes: HashMap<Uuid, Arc<NodeType>> = HashMap::default();
			let mut deps = self.get_chunk_dependencies(id);
			deps.reverse();

			for chunk in deps.drain(..) {
				if !nodes.contains_key(&chunk.id) {
					let mut buffer = vec![0u8; chunk.size as usize];
					reader.seek(std::io::SeekFrom::Start(chunk.offset))?;
					reader.read_exact(&mut buffer)?;

					let children: Vec<_> = chunk
						.children
						.iter()
						.map(|id| nodes.get(&id).unwrap().clone())
						.collect();
					let dependencies: Vec<_> = chunk
						.dependencies
						.iter()
						.map(|id| nodes.get(&id).unwrap().clone())
						.collect();

					let (_, node) = NodeType::parse(
						self.footer.version,
						&chunk,
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
		self.index.root = *node.id();
		self.update_node(node, false);
	}

	/// Mark node as dirty
	fn mark_dirty_node(&mut self, content: bool, shallow: bool, node: Arc<NodeType>) {
		let id = *node.id();
		let mut doc_node = match self.dirty_nodes.remove(&id) {
			Some(node) => node,
			None => DocumentNode {
				node: node.clone(),
				content,
				shallow,
			},
		};
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
	fn write_node<W: std::io::Write + std::io::Seek>(
		&mut self,
		writer: &mut W,
		content: bool,
		shallow: bool,
		node: Arc<NodeType>,
	) -> std::io::Result<usize> {
		let mut size = 0;
		let mut chunk = match self.chunks.remove(node.id()) {
			Some(chunk) => chunk,
			None => Chunk {
				id: *node.id(),
				node_type: 0,
				offset: 0,
				name: "".to_string(),
				..Default::default()
			},
		};
		chunk.node_type = node.node_id();
		chunk.offset = writer.seek(std::io::SeekFrom::Current(0))?;
		chunk.name = node.name().to_string();

		if let Ok(node_bounds) = TryInto::<&dyn HasBounds>::try_into(&*node) {
			chunk.rect = node_bounds.bounds();
		}

		let (node_size, node_rect, node_deps) = if content {
			node.write(writer)
		} else {
			node.write(&mut crate::io::Void)
		}?;

		chunk.size = node_size as u32;
		chunk.rect = node_rect;

		if content {
			size += node_size;
		}
		if !shallow {
			for child in node_deps.children.iter() {
				size += self.write_node(writer, content, shallow, child.clone())?;
			}
			for dep in node_deps.dependencies.iter() {
				size += self.write_node(writer, content, shallow, dep.clone())?;
			}
		}

		self.chunks.insert(*node.id(), chunk);
		Ok(size)
	}

	/// Append changes to file
	pub fn append<W: std::io::Write + std::io::Seek>(
		&mut self,
		writer: &mut W,
	) -> std::io::Result<usize> {
		let mut size = 0;
		let prev_offset = writer.seek(std::io::SeekFrom::End(0))?;
		let dirty_nodes: Vec<DocumentNode> =
			self.dirty_nodes.drain().map(|(_, node)| node).collect();
		for doc_node in dirty_nodes.iter() {
			size += self.write_node(
				writer,
				doc_node.content,
				doc_node.shallow,
				doc_node.node.clone(),
			)?;
		}

		let mut index_size = 0;
		for (_, chunk) in self.chunks.iter() {
			index_size += chunk.write(writer)?;
		}
		size += index_size;

		self.index.hash = Uuid::new_v4();
		self.index.prev_offset = prev_offset;
		self.index.size = index_size as u32;

		size += self.index.write(writer)?;
		size += (Footer { version: 0 }).write(writer)?;

		Ok(size)
	}

	/// Trim unused chunk to a new file
	pub fn trim<R: std::io::Read + std::io::Seek, W: std::io::Write + std::io::Seek>(
		&self,
		source: &mut R,
		destination: &mut W,
	) -> Result<usize, DocumentError> {
		destination.seek(std::io::SeekFrom::Start(0))?;
		let mut size = 0;

		let mut chunks = vec![];
		for (_, chunk) in self.chunks.iter() {
			let mut buffer = vec![0u8; chunk.size as usize];
			source.seek(std::io::SeekFrom::Start(chunk.offset))?;
			source.read_exact(&mut buffer)?;

			let mut new_chunk = chunk.clone();
			new_chunk.offset = size as u64;
			chunks.push(new_chunk);

			destination.write_all(&buffer)?;
			size += buffer.len();
		}

		let mut index_size = 0;
		for chunk in chunks.drain(..) {
			index_size += chunk.write(destination)?;
		}
		size += index_size;

		let mut index = self.index.clone();
		index.prev_offset = 0;
		index.size = index_size as u32;

		size += index.write(destination)?;
		size += (Footer { version: 0 }).write(destination)?;

		Ok(size)
	}

	/// Retrieve a new document pointing to previous Index
	pub fn read_previous<R: std::io::Read + std::io::Seek>(
		&self,
		reader: &mut R,
	) -> Result<Self, DocumentError> {
		if self.index.prev_offset == 0 {
			Err(DocumentError::NoPreviousVersion)
		} else {
			Self::read_at(reader, self.index.prev_offset)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use document_command::*;
	use document_core::*;

	#[test]
	fn write_and_read_simple_doc() {
		let mut doc = Document::default();
		let note = Note::new("My note", (0, 0), "");
		let root = Arc::new(NodeType::Note(note));
		doc.set_root_node(root.clone());

		let mut buffer: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
		let written = doc.append(&mut buffer).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), written);

		let doc2 = Document::read(&mut buffer).expect("Could not read");
		assert_eq!(doc.index, doc2.index);
		assert_eq!(doc.chunks, doc2.chunks);

		let root2 = doc2.get_root_node(&mut buffer).expect("Could not get root");
		assert_eq!(*root2, *root);
	}

	#[test]
	fn rename_read_previous_doc() {
		let mut doc = Document::default();
		let note = Note::new("My note", (0, 0), "");
		let root = Arc::new(NodeType::Note(note));
		doc.set_root_node(root.clone());

		let mut buffer: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
		let written = doc.append(&mut buffer).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), written);

		let rename = RenameCommand {
			target: *root.id(),
			name: "Your note".into(),
		};
		let root2 = Arc::new(rename.execute(&*root).expect("Could not rename"));
		assert_eq!(root2.name(), "Your note");
		assert_ne!(*root2, *root);

		doc.update_node(root2.clone(), false);

		let written2 = doc.append(&mut buffer).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), written + written2);

		let doc2 = Document::read(&mut buffer).expect("Could not read");
		assert_eq!(doc.index, doc2.index);
		assert_eq!(doc.chunks, doc2.chunks);

		let root3 = doc2.get_root_node(&mut buffer).expect("Could not get root");
		assert_eq!(*root3, *root2);

		let doc3 = doc2
			.read_previous(&mut buffer)
			.expect("Could not read previous");
		let root4 = doc3.get_root_node(&mut buffer).expect("Could not get root");
		assert_eq!(*root4, *root);
	}

	#[test]
	fn trim_doc() {
		let mut buffer: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(vec![
			1, 0, 0, 0, 0, 0, 24, 224, 18, 32, 86, 187, 66, 11, 187, 108, 2, 255, 22, 121, 210,
			221, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 77, 121, 32, 110, 111, 116, 101, 0, 0, 0,
			0, 0, 0, 0, 0, 65, 0, 0, 0, 24, 224, 18, 32, 86, 187, 66, 11, 187, 108, 2, 255, 22,
			121, 210, 221, 195, 112, 8, 189, 227, 99, 69, 139, 143, 161, 176, 32, 202, 148, 6, 201,
			0, 80, 88, 76, 82, 1, 0, 0, 0, 0, 0, 24, 224, 18, 32, 86, 187, 66, 11, 187, 108, 2,
			255, 22, 121, 210, 221, 1, 0, 120, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 89, 111, 117, 114,
			32, 110, 111, 116, 101, 120, 0, 0, 0, 0, 0, 0, 0, 67, 0, 0, 0, 24, 224, 18, 32, 86,
			187, 66, 11, 187, 108, 2, 255, 22, 121, 210, 221, 31, 232, 217, 133, 171, 17, 79, 69,
			131, 66, 82, 225, 238, 60, 215, 141, 0, 80, 88, 76, 82,
		]);
		let doc = Document::read(&mut buffer).expect("Could not read document");
		assert_eq!(doc.index.prev_offset, 120);

		let mut buffer2: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(vec![]);

		let size = doc
			.trim(&mut buffer, &mut buffer2)
			.expect("Could not trim document");
		assert_eq!(buffer2.get_ref().len(), size);
	}
}
