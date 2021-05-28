use crate::{Command, CommandType};
use document_core::{HasBounds, Node, NodeType};
use uuid::Uuid;
use vek::vec::repr_c::vec2::Vec2;

pub trait Translatable: HasBounds + Node {
	fn translate<P: Into<Vec2<i32>>>(&self, position: P) -> CommandType {
		CommandType::Translate(TranslateCommand {
			target: *self.id(),
			position: position.into(),
		})
	}
}

impl<N: HasBounds + Node> Translatable for N {}

#[derive(Debug, Clone, PartialEq)]
pub struct TranslateCommand {
	target: Uuid,
	position: Vec2<i32>,
}

impl TranslateCommand {
	pub fn new<U: Into<Uuid>, P: Into<Vec2<i32>>>(target: U, position: P) -> Self {
		Self {
			target: target.into(),
			position: position.into(),
		}
	}

	pub fn position(&self) -> &Vec2<i32> {
		&self.position
	}
}

impl Command for TranslateCommand {
	fn target(&self) -> &Uuid {
		&self.target
	}
	fn execute_impl(&self, node: &NodeType) -> Option<NodeType> {
		let mut cloned = node.clone();
		match cloned {
			NodeType::Note(ref mut cloned) => {
				cloned.set_position(self.position.clone());
			}
			NodeType::Group(ref mut cloned) => {
				cloned.set_position(self.position.clone());
			}
			NodeType::Palette(ref mut cloned) => {
				cloned.set_position(self.position.clone());
			}
			NodeType::CanvasGroup(ref mut cloned) => {
				cloned.set_position(self.position.clone());
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
	// use vek::vec::repr_c::vec2::Vec2;

	#[test]
	fn translate_note() {
		use document_core::Note;
		let note = Note::default();
		assert_eq!(note.bounds().into_aabr().min, Vec2::new(0, 0));

		let translate = note.translate(Vec2::new(10, 20));

		let note2 = match translate.execute(&NodeType::Note(note)) {
			Some(NodeType::Note(node)) => node,
			_ => panic!("Renamed did not result in a Note."),
		};
		assert_eq!(note2.bounds().into_aabr().min, Vec2::new(10, 20));
	}

	#[test]
	fn translate_group() {
		use document_core::Group;
		let group = Group::default();
		assert_eq!(group.bounds().into_aabr().min, Vec2::new(0, 0));

		let translate = group.translate(Vec2::new(10, 20));

		let group2 = match translate.execute(&NodeType::Group(group)) {
			Some(NodeType::Group(node)) => node,
			_ => panic!("Renamed did not result in a Group."),
		};
		assert_eq!(group2.bounds().into_aabr().min, Vec2::new(10, 20));
	}
}
