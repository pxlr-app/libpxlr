use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
mod color;
mod group;
mod note;
mod walk;

pub static DOCUMENT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub trait Node {
	fn id(&self) -> &Uuid;
	fn name(&self) -> &str;
}

pub trait NonLeafNode: Node {
	fn is_child_valid(&self, _node: &NodeType) -> bool;
	fn children(&self) -> &Arc<Vec<Arc<NodeType>>>;
}

pub use color::*;
pub use group::*;
pub use note::*;
pub use walk::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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
	fn name(&self) -> &str {
		match self {
			NodeType::Note(node) => node.name(),
			NodeType::Group(node) => node.name(),
		}
	}
}

impl<'a> std::convert::TryFrom<&'a NodeType> for &'a dyn NonLeafNode {
	type Error = ();

	fn try_from(value: &'a NodeType) -> Result<&'a dyn NonLeafNode, Self::Error> {
		match value {
			NodeType::Group(ref node) => Ok(node),
			_ => Err(()),
		}
	}
}

mod lib {
	#[cfg(test)]
	mod tests {
		use crate::{Group, NodeType, NonLeafNode};
		use std::convert::TryInto;

		#[test]
		fn try_into_nonleafnode() {
			let node = NodeType::Group(Group::default());
			let result: Result<&dyn NonLeafNode, ()> = (&node).try_into();
			assert!(result.is_ok());
		}
	}
}
