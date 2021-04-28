use document_core::{Node, NodeType};
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;
mod group;
pub mod io;
mod meta;
mod node;
mod note;
mod parser;

pub use group::*;
pub use meta::*;
pub use node::*;
pub use note::*;
pub use parser::*;

#[derive(Debug, Clone, Copy)]
enum Dirty {
	META,
	CONTENT,
	BOTH,
}

struct DocumentNode {
	mode: Dirty,
	shallow: bool,
	node: Arc<NodeType>,
}

pub struct Document {
	pub index: Index,
	pub chunks: HashMap<Uuid, Chunk>,
	pub(crate) dirty_nodes: HashMap<Uuid, DocumentNode>,
}

impl Default for Document {
	fn default() -> Self {
		Document {
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
}

impl std::error::Error for DocumentError {}

impl std::fmt::Display for DocumentError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			DocumentError::Parse(err) => write!(f, "Parse error: {}", err),
			DocumentError::IO(err) => write!(f, "File IO error: {}", err),
			DocumentError::UnsupportedVersion(ver) => write!(f, "Unsupported version: {}", ver),
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

				Ok(Document {
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

	/// Retrieve a node by it's ID
	pub fn get_node_by_id<R: std::io::Read + std::io::Seek>(
		&self,
		_reader: &mut R,
		_id: Uuid,
	) -> Result<Arc<NodeType>, DocumentError> {
		unimplemented!()
	}

	/// Mark node as dirty (everything) and use it as root node
	pub fn set_root_node(&mut self, node: Arc<NodeType>) {
		self.index.root = *node.id();
		self.dirty_nodes.insert(
			*node.id(),
			DocumentNode {
				mode: Dirty::BOTH,
				shallow: false,
				node: node.clone(),
			},
		);
	}

	/// Mark node as dirty
	fn mark_dirty_node(&mut self, mode: Dirty, shallow: bool, node: Arc<NodeType>) {
		let id = *node.id();
		let doc_node = match self.dirty_nodes.remove(&id) {
			Some(node) => node,
			None => DocumentNode {
				node,
				mode,
				shallow,
			},
		};
		self.dirty_nodes.insert(id, doc_node);
	}

	/// Mark node and children (shallow?) as dirty (content, meta)
	pub fn update_node(&mut self, node: Arc<NodeType>, shallow: bool) {
		self.mark_dirty_node(Dirty::BOTH, shallow, node.clone());
	}

	/// Mark node and children (shallow?) as dirty (meta only)
	pub fn touch_node(&mut self, node: Arc<NodeType>, shallow: bool) {
		self.mark_dirty_node(Dirty::META, shallow, node.clone());
	}

	/// Write node at the end of file and return chunk dependencies
	fn write_node<W: std::io::Write + std::io::Seek>(
		&mut self,
		writer: &mut W,
		mode: &Dirty,
		shallow: bool,
		node: Arc<NodeType>,
	) -> std::io::Result<usize> {
		let mut size = 0;
		let mut chunk = match self.chunks.remove(node.id()) {
			Some(chunk) => chunk,
			None => Chunk {
				id: *node.id(),
				node: 0,
				offset: writer.seek(std::io::SeekFrom::Current(0))?,
				name: node.name().to_string(),
				..Default::default()
			},
		};
		let (node_size, node_rect, node_deps) = if let Dirty::CONTENT | Dirty::BOTH = mode {
			node.write(writer)
		} else {
			node.write(&mut crate::io::Void)
		}?;

		if let Dirty::META | Dirty::BOTH = mode {
			chunk.size = node_size as u32;
			chunk.rect = node_rect;
		}
		if let Dirty::CONTENT | Dirty::BOTH = mode {
			size += node_size;
		}
		if !shallow {
			for child in node_deps.children.iter() {
				size += self.write_node(writer, mode, shallow, child.clone())?;
			}
			for dep in node_deps.dependencies.iter() {
				size += self.write_node(writer, mode, shallow, dep.clone())?;
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
				&doc_node.mode,
				doc_node.shallow,
				doc_node.node.clone(),
			)?;
		}

		let mut index_size = 0;
		for (_, chunk) in self.chunks.iter() {
			index_size += chunk.write(writer)?;
		}
		size += index_size;

		self.index.prev_offset = prev_offset;
		self.index.size = index_size as u32;
		size += self.index.write(writer)?;

		size += (Footer { version: 0 }).write(writer)?;

		Ok(size)
	}

	/// Trim unused chunk to a new file
	pub fn trim(&self) -> std::io::Result<usize> {
		unimplemented!()
	}

	/// Retrieve a new document pointing to previous Index
	pub fn previous_version(&self) -> std::io::Result<Document> {
		unimplemented!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use document_core::*;

	#[test]
	fn write_and_read_simple_doc() {
		let mut doc = Document::default();
		let root = Arc::new(NodeType::Note(Note::default()));
		doc.set_root_node(root);

		let mut buffer: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(Vec::new());
		let written = doc.append(&mut buffer).expect("Could not write");
		assert_eq!(buffer.get_ref().len(), written);

		let doc2 = Document::read(&mut buffer).expect("Could not read");
		assert_eq!(doc.index, doc2.index);
		assert_eq!(doc.chunks, doc2.chunks);

		let _root = doc2.get_root_node(&mut buffer).expect("Could not get root");
	}
}
