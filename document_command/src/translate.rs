use crate::{Command, CommandType};
use document_core::{Node, NodeType};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use vek::vec::repr_c::vec2::Vec2;

pub trait Translatable: Node {
	fn position(&self) -> &Vec2<i32>;

	fn translate<P: Into<Vec2<i32>>>(&self, position: P) -> (CommandType, CommandType) {
		(
			CommandType::Translate(TranslateCommand {
				target: *self.id(),
				position: position.into(),
			}),
			CommandType::Translate(TranslateCommand {
				target: *self.id(),
				position: *self.position(),
			}),
		)
	}
}

impl Translatable for document_core::Note {
	fn position(&self) -> &Vec2<i32> {
		&self.position
	}
}

impl Translatable for document_core::Group {
	fn position(&self) -> &Vec2<i32> {
		&self.position
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslateCommand {
	pub target: Uuid,
	pub position: Vec2<i32>,
}

impl Command for TranslateCommand {
	fn target(&self) -> &Uuid {
		&self.target
	}
	fn execute_impl(&self, node: &NodeType) -> Option<NodeType> {
		let mut cloned = node.clone();
		match cloned {
			NodeType::Note(ref mut cloned) => {
				cloned.position = Arc::new(self.position.clone());
			}
			NodeType::Group(ref mut cloned) => {
				cloned.position = Arc::new(self.position.clone());
			}
			NodeType::Palette(ref mut cloned) => {
				cloned.position = Arc::new(self.position.clone());
			}
			NodeType::CanvasGroup(ref mut cloned) => {
				cloned.position = Arc::new(self.position.clone());
			}
		}

		Some(cloned)
	}
}

#[cfg(test)]
mod tests {
	use super::Translatable;
	use crate::Command;
	use document_core::NodeType;
	use vek::vec::repr_c::vec2::Vec2;

	#[test]
	fn translate_note() {
		use document_core::Note;
		let note = Note::default();
		assert_eq!(*note.position, Vec2::new(0, 0));

		let (translate, _) = note.translate(Vec2::new(10, 20));

		let note2 = match translate.execute(&NodeType::Note(note)) {
			Some(NodeType::Note(node)) => node,
			_ => panic!("Renamed did not result in a Note."),
		};
		assert_eq!(*note2.position, Vec2::new(10, 20));
	}

	#[test]
	fn translate_group() {
		use document_core::Group;
		let group = Group::default();
		assert_eq!(*group.position, Vec2::new(0, 0));

		let (translate, _) = group.translate(Vec2::new(10, 20));

		let group2 = match translate.execute(&NodeType::Group(group)) {
			Some(NodeType::Group(node)) => node,
			_ => panic!("Renamed did not result in a Group."),
		};
		assert_eq!(*group2.position, Vec2::new(10, 20));
	}
}
