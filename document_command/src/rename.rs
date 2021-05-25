use crate::{Command, CommandType};
use document_core::{Node, NodeType};
use uuid::Uuid;

pub trait Renamable: Node {
	fn rename<S: Into<String>>(&self, name: S) -> (CommandType, CommandType) {
		(
			CommandType::Rename(RenameCommand {
				target: *self.id(),
				name: name.into(),
			}),
			CommandType::Rename(RenameCommand {
				target: *self.id(),
				name: self.name().to_string(),
			}),
		)
	}
}

impl<N: Node> Renamable for N {}

#[derive(Debug, Clone)]
pub struct RenameCommand {
	pub target: Uuid,
	pub name: String,
}

impl Command for RenameCommand {
	fn target(&self) -> &Uuid {
		&self.target
	}
	fn execute_impl(&self, node: &NodeType) -> Option<NodeType> {
		let mut cloned = node.clone();
		match cloned {
			NodeType::Note(ref mut cloned) => {
				cloned.set_name(self.name.clone());
			}
			NodeType::Group(ref mut cloned) => {
				cloned.set_name(self.name.clone());
			}
			NodeType::Palette(ref mut cloned) => {
				cloned.set_name(self.name.clone());
			}
			NodeType::CanvasGroup(ref mut cloned) => {
				cloned.set_name(self.name.clone());
			}
			_ => {
				return None;
			}
		}

		Some(cloned)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rename_note() {
		use document_core::Note;
		let note = Note::default();
		assert_eq!(note.name(), "Note");

		let (rename, _) = note.rename("Foo");

		let note2 = match rename.execute(&NodeType::Note(note)) {
			Some(NodeType::Note(node)) => node,
			_ => panic!("Renamed did not result in a Note."),
		};
		assert_eq!(note2.name(), "Foo");
	}

	#[test]
	fn rename_group() {
		use document_core::Group;
		let group = Group::default();
		assert_eq!(group.name(), "Group");

		let (rename, _) = group.rename("Foo");

		let group2 = match rename.execute(&NodeType::Group(group)) {
			Some(NodeType::Group(node)) => node,
			_ => panic!("Renamed did not result in a Group."),
		};
		assert_eq!(group2.name(), "Foo");
	}
}
