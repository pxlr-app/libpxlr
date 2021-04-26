use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
mod group;
mod note;

pub static DOCUMENT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub trait Node {
	fn id(&self) -> &Uuid;
	fn display(&self) -> &str;
}

pub trait NonLeafNode: Node {
	fn children(&self) -> &Arc<Vec<Arc<NodeType>>>;
}

pub use group::Group;
pub use note::Note;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
	Note(note::Note),
	Group(group::Group),
}

impl Node for NodeType {
	fn id(&self) -> &Uuid {
		match self {
			NodeType::Note(node) => node.id(),
			NodeType::Group(node) => node.id(),
		}
	}
	fn display(&self) -> &str {
		match self {
			NodeType::Note(node) => node.display(),
			NodeType::Group(node) => node.display(),
		}
	}
}
