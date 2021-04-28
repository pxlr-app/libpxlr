use crate::{Command, CommandType};
use document_core::{NodeType, Note};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

pub trait Content {
	fn set_content<S: Into<String>>(&self, content: S) -> (CommandType, CommandType);
}

impl Content for Note {
	fn set_content<S: Into<String>>(&self, content: S) -> (CommandType, CommandType) {
		(
			CommandType::SetNoteContent(SetNoteContentCommand {
				target: self.id,
				content: content.into(),
			}),
			CommandType::SetNoteContent(SetNoteContentCommand {
				target: self.id,
				content: self.content.to_string(),
			}),
		)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetNoteContentCommand {
	pub target: Uuid,
	pub content: String,
}

impl Command for SetNoteContentCommand {
	fn target(&self) -> &Uuid {
		&self.target
	}
	fn execute_impl(&self, node: &NodeType) -> Option<NodeType> {
		match node {
			NodeType::Note(node) => {
				let mut cloned = node.clone();
				cloned.content = Arc::new(self.content.clone());
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
		assert_eq!(*note.name, "Note");
		assert_eq!(*note.content, "");

		let (set_content, _) = note.set_content("Lorem ipsum");

		let note2 = match set_content.execute(&NodeType::Note(note)) {
			Some(NodeType::Note(node)) => node,
			_ => panic!("Set content did not result in a Note."),
		};
		assert_eq!(*note2.name, "Note");
		assert_eq!(*note2.content, "Lorem ipsum");
	}
}
