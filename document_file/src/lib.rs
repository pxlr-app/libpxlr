use document_core::NodeType;
use std::{io, sync::Arc};
use uuid::Uuid;
mod meta;
mod parser;

pub use meta::*;
pub use parser::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Document {
	pub index: Index,
	pub chunks: Vec<Chunk>,
}

impl Default for Document {
	fn default() -> Self {
		Document {
			index: Index::default(),
			chunks: vec![],
		}
	}
}

impl Document {
	/// Retrieve root node
	pub fn get_root_node(&mut self) -> Arc<NodeType> {
		unimplemented!()
	}
	/// Retrieve a node by it's ID
	pub fn get_node_by_id(&mut self, _id: Uuid) -> Arc<NodeType> {
		unimplemented!()
	}
	/// Mark node as dirty (everything) and use it as root node
	pub fn set_root_node(&mut self, _node: Arc<NodeType>) {
		unimplemented!()
	}
	/// Mark node and children (shallow?) as dirty (content, meta)
	pub fn update_node(&mut self, _node: Arc<NodeType>, _shallow: bool) {
		unimplemented!()
	}
	/// Mark node and children (shallow?) as dirty (meta only)
	pub fn touch_node(&mut self, _node: Arc<NodeType>, _shallow: bool) {
		unimplemented!()
	}
	/// Trim unused chunk to a new file
	pub fn trim(&self) -> io::Result<usize> {
		unimplemented!()
	}
	/// Retrieve a new document pointing to previous Index
	pub fn previous_version(&self) -> io::Result<Document> {
		unimplemented!()
	}
}

#[cfg(test)]
mod tests {
	// #[test]
	// fn it_works() {
	// 	assert_eq!(2 + 2, 4);
	// }
}
