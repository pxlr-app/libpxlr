use crate::{Command, CommandType};
use document_core::{HasContent, Node, NodeType};
use uuid::Uuid;

pub trait EditContent: HasContent + Node {
	fn edit_content<S: Into<String>>(&self, content: S) -> CommandType {
		CommandType::SetNoteContent(SetNoteContentCommand {
			target: *self.id(),
			content: content.into(),
		})
	}
}

impl<N: HasContent + Node> EditContent for N {}

#[derive(Debug, Clone, PartialEq)]
pub struct SetNoteContentCommand {
	target: Uuid,
	content: String,
}

impl SetNoteContentCommand {
	pub fn new<U: Into<Uuid>, S: Into<String>>(target: U, content: S) -> Self {
		Self {
			target: target.into(),
			content: content.into(),
		}
	}

	pub fn content(&self) -> &String {
		&self.content
	}
}

impl Command for SetNoteContentCommand {
	fn target(&self) -> &Uuid {
		&self.target
	}
	fn execute_impl(&self, node: &NodeType) -> Option<NodeType> {
		match node {
			NodeType::Note(node) => {
				let mut cloned = node.clone();
				cloned.set_content(self.content.clone());
				Some(NodeType::Note(cloned))
			}
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Command;
	use document_core::NodeType;

	#[test]
	fn set_content() {
		use document_core::Note;
		let note = Note::default();
		assert_eq!(note.name(), "Note");
		assert_eq!(note.content(), "");

		let set_content = note.edit_content("Lorem ipsum");

		let note2 = match set_content.execute(&NodeType::Note(note)) {
			Some(NodeType::Note(node)) => node,
			_ => panic!("Set content did not result in a Note."),
		};
		assert_eq!(note2.name(), "Note");
		assert_eq!(note2.content(), "Lorem ipsum");
	}
}
