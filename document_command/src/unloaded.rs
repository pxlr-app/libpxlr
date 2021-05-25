use crate::{Command, CommandType};
use document_core::{HasBounds, Node, NodeType, Unloaded};
use std::sync::Arc;
use uuid::Uuid;
use vek::Rect;

pub trait LoadNode {
	fn load(&self) -> CommandType;
}

pub trait UnloadNode {
	fn unload(&self) -> CommandType;
}

impl LoadNode for Arc<NodeType> {
	fn load(&self) -> CommandType {
		CommandType::LoadNode(LoadNodeCommand {
			target: *self.id(),
			node: self.clone(),
		})
	}
}

impl<N: Node> UnloadNode for N {
	fn unload(&self) -> CommandType {
		CommandType::UnloadNode(UnloadNodeCommand { target: *self.id() })
	}
}

#[derive(Debug, Clone)]
pub struct LoadNodeCommand {
	pub target: Uuid,
	pub node: Arc<NodeType>,
}

#[derive(Debug, Clone)]
pub struct UnloadNodeCommand {
	pub target: Uuid,
}

impl Command for LoadNodeCommand {
	fn target(&self) -> &Uuid {
		&self.target
	}
	fn execute_impl(&self, _node: &NodeType) -> Option<NodeType> {
		Some((*self.node).clone())
	}
}

impl Command for UnloadNodeCommand {
	fn target(&self) -> &Uuid {
		&self.target
	}
	fn execute_impl(&self, node: &NodeType) -> Option<NodeType> {
		let bounds = match std::convert::TryInto::<&dyn HasBounds>::try_into(node) {
			Ok(bounds) => bounds.bounds().clone(),
			Err(_) => Rect::new(0, 0, 0, 0),
		};
		Some(NodeType::Unloaded(unsafe {
			Unloaded::construct(*node.id(), node.name().to_string(), bounds)
		}))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn load_node() {
		use document_core::{HasBounds, Node, Note, Unloaded};
		let note = Note::default();
		let unloaded = NodeType::Unloaded(unsafe {
			Unloaded::construct(*note.id(), note.name().to_string(), note.bounds())
		});

		let load = LoadNodeCommand {
			target: *unloaded.id(),
			node: Arc::new(NodeType::Note(note)),
		};

		let note2 = match load.execute(&unloaded) {
			Some(NodeType::Note(node)) => node,
			_ => panic!("Load did not result in a Note."),
		};
		assert_eq!(note2.name(), unloaded.name());
	}

	#[test]
	fn unload_node() {
		use document_core::{Node, Note};
		let note = NodeType::Note(Note::default());

		let load = UnloadNodeCommand { target: *note.id() };

		let note2 = match load.execute(&note) {
			Some(NodeType::Unloaded(node)) => node,
			_ => panic!("Unload did not unload Note."),
		};
		assert_eq!(note2.name(), note.name());
	}
}
