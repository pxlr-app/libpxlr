use crate::{Command, CommandType};
use document_core::{Node, NodeType, NonLeafNode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParentingError {
	ExistingChild,
	InvalidChild,
}

impl std::fmt::Display for ParentingError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ParentingError::ExistingChild => write!(f, "Child already present"),
			ParentingError::InvalidChild => write!(f, "Invalid child"),
		}
	}
}

impl std::error::Error for ParentingError {}

pub trait Parenting: NonLeafNode {
	fn add_child(
		&self,
		child: Arc<NodeType>,
	) -> Result<(CommandType, CommandType), ParentingError> {
		if !self.is_child_valid(&child) {
			return Err(ParentingError::InvalidChild);
		}
		let child_found = self
			.children()
			.iter()
			.find(|c| c.id() == child.id())
			.is_some();
		if child_found {
			Err(ParentingError::ExistingChild)
		} else {
			Ok((
				CommandType::AddChild(AddChildCommand {
					target: *self.id(),
					child: child.clone(),
				}),
				CommandType::RemoveChild(RemoveChildCommand {
					target: *self.id(),
					child_id: *child.id(),
				}),
			))
		}
	}

	fn remove_child(&self, child_id: Uuid) -> Result<(CommandType, CommandType), ParentingError> {
		let child = self.children().iter().find(|child| child.id() == &child_id);
		match child {
			Some(child) => Ok((
				CommandType::RemoveChild(RemoveChildCommand {
					target: *self.id(),
					child_id: child_id,
				}),
				CommandType::AddChild(AddChildCommand {
					target: *self.id(),
					child: child.clone(),
				}),
			)),
			None => Err(ParentingError::InvalidChild),
		}
	}

	fn move_child(
		&self,
		child_id: Uuid,
		position: usize,
	) -> Result<(CommandType, CommandType), ParentingError> {
		let old_position = self
			.children()
			.iter()
			.position(|child| child.id() == &child_id);
		match old_position {
			Some(old_position) => Ok((
				CommandType::MoveChild(MoveChildCommand {
					target: *self.id(),
					child_id,
					position,
				}),
				CommandType::MoveChild(MoveChildCommand {
					target: *self.id(),
					child_id,
					position: old_position,
				}),
			)),
			None => Err(ParentingError::InvalidChild),
		}
	}
}

impl<N: NonLeafNode> Parenting for N {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddChildCommand {
	pub target: Uuid,
	pub child: Arc<NodeType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveChildCommand {
	pub target: Uuid,
	pub child_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveChildCommand {
	pub target: Uuid,
	pub child_id: Uuid,
	pub position: usize,
}

impl Command for AddChildCommand {
	fn target(&self) -> &Uuid {
		&self.target
	}
	fn execute_impl(&self, node: &NodeType) -> Option<NodeType> {
		match node {
			NodeType::Group(node) => {
				let mut cloned = node.clone();
				let mut children: Vec<Arc<NodeType>> =
					cloned.children.iter().map(|child| child.clone()).collect();
				children.push(self.child.clone());
				cloned.children = Arc::new(children);
				Some(NodeType::Group(cloned))
			}
			_ => None,
		}
	}
}

impl Command for RemoveChildCommand {
	fn target(&self) -> &Uuid {
		&self.target
	}
	fn execute_impl(&self, node: &NodeType) -> Option<NodeType> {
		match node {
			NodeType::Group(node) => {
				let mut cloned = node.clone();
				let children: Vec<Arc<NodeType>> = cloned
					.children
					.iter()
					.filter_map(|child| {
						if child.id() == &self.child_id {
							None
						} else {
							Some(child.clone())
						}
					})
					.collect();
				cloned.children = Arc::new(children);
				Some(NodeType::Group(cloned))
			}
			_ => None,
		}
	}
}

impl Command for MoveChildCommand {
	fn target(&self) -> &Uuid {
		&self.target
	}
	fn execute_impl(&self, node: &NodeType) -> Option<NodeType> {
		match node {
			NodeType::Group(node) => {
				let mut cloned = node.clone();
				let mut children: Vec<Arc<NodeType>> =
					cloned.children.iter().map(|child| child.clone()).collect();
				let child = children.remove(self.position);
				if self.position > children.len() {
					children.push(child);
				} else {
					children.insert(self.position, child);
				}
				cloned.children = Arc::new(children);
				Some(NodeType::Group(cloned))
			}
			_ => None,
		}
	}
}
