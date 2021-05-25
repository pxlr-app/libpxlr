use crate::{Command, CommandType};
use document_core::{HasChildren, Node, NodeType};
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

pub trait Parenting: HasChildren + Node {
	fn add_child(&self, child: Arc<NodeType>) -> Result<CommandType, ParentingError> {
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
			Ok(CommandType::AddChild(AddChildCommand {
				target: *self.id(),
				child: child.clone(),
			}))
		}
	}

	fn remove_child(&self, child_id: Uuid) -> Result<CommandType, ParentingError> {
		let child = self.children().iter().find(|child| child.id() == &child_id);
		match child {
			Some(_) => Ok(CommandType::RemoveChild(RemoveChildCommand {
				target: *self.id(),
				child_id: child_id,
			})),
			None => Err(ParentingError::InvalidChild),
		}
	}

	fn move_child(&self, child_id: Uuid, position: usize) -> Result<CommandType, ParentingError> {
		let old_position = self
			.children()
			.iter()
			.position(|child| child.id() == &child_id);
		match old_position {
			Some(_) => Ok(CommandType::MoveChild(MoveChildCommand {
				target: *self.id(),
				child_id,
				position,
			})),
			None => Err(ParentingError::InvalidChild),
		}
	}
}

impl<N: HasChildren + Node> Parenting for N {}

#[derive(Debug, Clone)]
pub struct AddChildCommand {
	pub target: Uuid,
	pub child: Arc<NodeType>,
}

#[derive(Debug, Clone)]
pub struct RemoveChildCommand {
	pub target: Uuid,
	pub child_id: Uuid,
}

#[derive(Debug, Clone)]
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
		let mut cloned = node.clone();
		if let Ok(group) = std::convert::TryInto::<&mut dyn HasChildren>::try_into(&mut cloned) {
			let mut children: Vec<_> = group.children().iter().map(|child| child.clone()).collect();
			children.push(self.child.clone());
			group.set_children(children);
			Some(cloned)
		} else {
			None
		}
	}
}

impl Command for RemoveChildCommand {
	fn target(&self) -> &Uuid {
		&self.target
	}
	fn execute_impl(&self, node: &NodeType) -> Option<NodeType> {
		let mut cloned = node.clone();
		if let Ok(group) = std::convert::TryInto::<&mut dyn HasChildren>::try_into(&mut cloned) {
			let children: Vec<_> = group
				.children()
				.iter()
				.filter_map(|child| {
					if child.id() == &self.child_id {
						None
					} else {
						Some(child.clone())
					}
				})
				.collect();
			group.set_children(children);
			Some(cloned)
		} else {
			None
		}
	}
}

impl Command for MoveChildCommand {
	fn target(&self) -> &Uuid {
		&self.target
	}
	fn execute_impl(&self, node: &NodeType) -> Option<NodeType> {
		let mut cloned = node.clone();
		if let Ok(group) = std::convert::TryInto::<&mut dyn HasChildren>::try_into(&mut cloned) {
			let mut children: Vec<_> = group.children().iter().map(|child| child.clone()).collect();
			let child = children.remove(self.position);
			if self.position > children.len() {
				children.push(child);
			} else {
				children.insert(self.position, child);
			}
			group.set_children(children);
			Some(cloned)
		} else {
			None
		}
	}
}
